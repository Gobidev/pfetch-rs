use std::{collections::VecDeque, env, fs, io::Result, process::Command};

use glob::glob;
use globset::Glob;
use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    traits::PackageReadout as _, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};
use pfetch_logo_parser::{parse_logo, Logo};

#[derive(Debug)]
pub enum PackageManager {
    Pacman,
    Dpkg,
    Xbps,
    Apk,
    Rpm,
    Flatpak,
    Crux,
    Guix,
    Opkg,
    Kiss,
    Portage,
    Pkgtool,
    Nix,
}

/// Obtain the amount of installed packages on the system by checking all installed supported package
/// managers and adding the amounts
pub fn total_packages(package_readout: &PackageReadout, skip_slow: bool) -> usize {
    match env::consts::OS {
        "linux" => {
            let macchina_package_count: Vec<(String, usize)> = package_readout
                .count_pkgs()
                .iter()
                .map(|(macchina_manager, count)| (macchina_manager.to_string(), *count))
                .collect();
            [
                PackageManager::Pacman,
                PackageManager::Dpkg,
                PackageManager::Xbps,
                PackageManager::Apk,
                PackageManager::Rpm,
                PackageManager::Flatpak,
                PackageManager::Crux,
                PackageManager::Guix,
                PackageManager::Opkg,
                PackageManager::Kiss,
                PackageManager::Portage,
                PackageManager::Pkgtool,
                PackageManager::Nix,
            ]
            .iter()
            .map(|mngr| packages(mngr, &macchina_package_count, skip_slow))
            .sum()
        }
        _ => package_readout.count_pkgs().iter().map(|elem| elem.1).sum(),
    }
}

fn get_macchina_package_count(
    macchina_result: &[(String, usize)],
    package_manager_name: &str,
) -> Option<usize> {
    macchina_result
        .iter()
        .find(|entry| entry.0 == package_manager_name)
        .map(|entry| entry.1)
}

/// return the amount of packages installed with a given linux package manager
/// Return `0` if the package manager is not installed
fn packages(
    pkg_manager: &PackageManager,
    macchina_package_count: &[(String, usize)],
    skip_slow: bool,
) -> usize {
    match pkg_manager {
        // libmacchina has very fast implementations for most package managers, so we use them
        // where we can, otherwise we fall back to method used by dylans version of pfetch
        PackageManager::Pacman
        | PackageManager::Flatpak
        | PackageManager::Dpkg
        | PackageManager::Xbps
        | PackageManager::Apk
        | PackageManager::Portage
        | PackageManager::Nix
        | PackageManager::Opkg => get_macchina_package_count(
            macchina_package_count,
            &format!("{pkg_manager:?}").to_lowercase(),
        )
        .unwrap_or(0),
        PackageManager::Rpm => match get_macchina_package_count(
            macchina_package_count,
            &format!("{pkg_manager:?}").to_lowercase(),
        ) {
            Some(count) => count,
            None => {
                if !skip_slow {
                    run_and_count_lines("rpm", &["-qa"])
                } else {
                    0
                }
            }
        },
        PackageManager::Guix => run_and_count_lines("guix", &["package", "--list-installed"]),
        PackageManager::Crux => {
            if check_if_command_exists("crux") {
                run_and_count_lines("pkginfo", &["-i"])
            } else {
                0
            }
        }
        PackageManager::Kiss => {
            if check_if_command_exists("kiss") {
                match glob("/var/db/kiss/installed/*/") {
                    Ok(files) => files.count(),
                    Err(_) => 0,
                }
            } else {
                0
            }
        }
        PackageManager::Pkgtool => {
            if check_if_command_exists("pkgtool") {
                match glob("/var/log/packages/*") {
                    Ok(files) => files.count(),
                    Err(_) => 0,
                }
            } else {
                0
            }
        }
    }
}

pub fn user_at_hostname(
    general_readout: &GeneralReadout,
    username_override: &Option<String>,
    hostname_override: &Option<String>,
) -> Option<String> {
    let username = match username_override {
        Some(username) => Ok(username.to_string()),
        None => general_readout.username(),
    };
    let hostname = match hostname_override {
        Some(hostname) => Ok(hostname.to_string()),
        None => general_readout.hostname(),
    };
    if username.is_err() || hostname.is_err() {
        None
    } else {
        Some(format!(
            "{}@{}",
            username.unwrap_or_default(),
            hostname.unwrap_or_default()
        ))
    }
}

pub fn memory(memory_readout: &MemoryReadout) -> Option<String> {
    let total_memory = memory_readout.total();
    let used_memory = memory_readout.used();
    if total_memory.is_err() || used_memory.is_err() {
        None
    } else {
        Some(format!(
            "{}M / {}M",
            used_memory.unwrap() / 1024,
            total_memory.unwrap() / 1024
        ))
    }
}

pub fn cpu(general_readout: &GeneralReadout) -> Option<String> {
    general_readout.cpu_model_name().ok()
}

pub fn os(general_readout: &GeneralReadout) -> Option<String> {
    match env::consts::OS {
        "linux" => {
            // check for Bedrock Linux
            if dotenvy::var("PATH")
                .unwrap_or_default()
                .contains("/bedrock/cross/")
            {
                return Some("Bedrock Linux".to_string());
            }
            let content = os_release::OsRelease::new().ok()?;
            let version = if !content.version.is_empty() {
                content.version
            } else {
                content.version_id
            };
            // check for Bazzite
            if content.pretty_name.contains("Bazzite") {
                return Some(format!("Bazzite {version}"));
            }
            if !version.is_empty() {
                return Some(format!("{} {}", content.name, version));
            }
            Some(content.name)
        }
        _ => Some(general_readout.os_name().ok()?.replace("Unknown", "")),
    }
}

pub fn kernel(kernel_readout: &KernelReadout) -> Option<String> {
    kernel_readout.os_release().ok()
}

pub fn seconds_to_string(seconds: usize) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    let mut result = String::with_capacity(10);

    if days > 0 {
        result.push_str(&format!("{}d", days));
    }
    if hours > 0 {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&format!("{}h", hours));
    }
    if minutes > 0 || result.is_empty() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&format!("{}m", minutes));
    }

    result
}

pub fn uptime(general_readout: &GeneralReadout) -> Option<String> {
    Some(seconds_to_string(general_readout.uptime().ok()?))
}

pub fn host(general_readout: &GeneralReadout) -> Option<String> {
    match env::consts::OS {
        "linux" => {
            const BLACKLIST: &[&str] = &[
                "To",
                "Be",
                "be",
                "Filled",
                "filled",
                "By",
                "by",
                "O.E.M.",
                "OEM",
                "Not",
                "Applicable",
                "Specified",
                "System",
                "Product",
                "Name",
                "Version",
                "Undefined",
                "Default",
                "string",
                "INVALID",
                "�",
                "os",
                "Type1ProductConfigId",
                "",
            ];

            // get device from system files
            let product_name =
                fs::read_to_string("/sys/devices/virtual/dmi/id/product_name").unwrap_or_default();
            let product_name = product_name.trim();
            let product_version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
                .unwrap_or_default();
            let product_version = product_version.trim();
            let product_model =
                fs::read_to_string("/sys/firmware/devicetree/base/model").unwrap_or_default();
            let product_model = product_model.trim();

            let final_str = format!("{product_name} {product_version} {product_model}")
                .split(' ')
                .filter(|word| !BLACKLIST.contains(word))
                .collect::<Vec<_>>()
                .join(" ");

            // if string is empty, display system architecture instead
            let final_str = if final_str.is_empty() {
                run_system_command("uname", &["-m"]).unwrap_or("Unknown".to_owned())
            } else {
                final_str
            };
            if final_str.is_empty() {
                None
            } else {
                Some(final_str)
            }
        }
        // on non-linux systems, try general_readout.machine(), use cpu model name as fallback
        _ => general_readout
            .machine()
            .ok()
            .or_else(|| general_readout.cpu_model_name().ok()),
    }
}

fn parse_custom_logos(filename: &str) -> Vec<Option<Logo>> {
    let file_contents = fs::read_to_string(filename).expect("Could not open custom logo file");
    file_contents
        .split(";;")
        .map(|raw_logo| parse_logo(raw_logo).map(|(_, logo)| logo))
        .collect::<Vec<_>>()
}

pub fn logo(logo_name: &str) -> Logo {
    let (tux, included_logos) = pfetch_extractor::parse_logos!();
    let mut logos: VecDeque<_> = included_logos.into();
    if let Ok(filename) = dotenvy::var("PF_CUSTOM_LOGOS") {
        // insert custom logos in front of incuded logos
        for custom_logo in parse_custom_logos(&filename).into_iter().flatten() {
            logos.insert(0, custom_logo.clone());
        }
    };
    logos
        .into_iter()
        .find(|logo| {
            logo.pattern.split('|').any(|glob| {
                Glob::new(glob.trim())
                    .expect("Invalid logo pattern")
                    .compile_matcher()
                    .is_match(logo_name)
            })
        })
        .unwrap_or(tux)
}

pub fn shell(general_readout: &GeneralReadout) -> Option<String> {
    general_readout
        .shell(
            libmacchina::traits::ShellFormat::Relative,
            libmacchina::traits::ShellKind::Default,
        )
        .ok()
        .or_else(|| dotenvy::var("SHELL").ok())
}

pub fn editor() -> Option<String> {
    env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .ok()
        .map(|editor| editor.trim().to_owned())
}

pub fn wm(general_readout: &GeneralReadout) -> Option<String> {
    general_readout.window_manager().ok()
}

pub fn de(general_readout: &GeneralReadout) -> Option<String> {
    general_readout
        .desktop_environment()
        .ok()
        .or_else(|| dotenvy::var("XDG_CURRENT_DESKTOP").ok())
}

pub fn palette() -> String {
    (1..7).fold("".to_string(), |a, e| a + &format!("\x1b[4{e}m  ")) + "\x1b[0m"
}

fn run_system_command(command: &str, args: &[&str]) -> Result<String> {
    let mut output =
        String::from_utf8_lossy(&Command::new(command).args(args).output()?.stdout).into_owned();
    output.truncate(output.trim_end().len());
    Ok(output)
}

fn check_if_command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}

fn _system_command_error(command: &str, args: &[&str]) -> Result<String> {
    let mut output =
        String::from_utf8_lossy(&Command::new(command).args(args).output()?.stderr).into_owned();
    output.truncate(output.trim_end().len());
    Ok(output)
}

/// Return the amount of line the output of a system command produces
/// Returns `0` if command fails
fn run_and_count_lines(command: &str, args: &[&str]) -> usize {
    run_system_command(command, args)
        .unwrap_or_default()
        .lines()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_to_string_0() {
        assert_eq!(seconds_to_string(0), "0m".to_string());
    }

    #[test]
    fn test_seconds_to_string_60() {
        assert_eq!(seconds_to_string(60), "1m".to_string());
    }

    #[test]
    fn test_seconds_to_string_3600() {
        assert_eq!(seconds_to_string(3600), "1h".to_string());
    }

    #[test]
    fn test_seconds_to_string_3660() {
        assert_eq!(seconds_to_string(3660), "1h 1m".to_string());
    }

    #[test]
    fn test_seconds_to_string_86400() {
        assert_eq!(seconds_to_string(86400), "1d".to_string());
    }

    #[test]
    fn test_seconds_to_string_90000() {
        assert_eq!(seconds_to_string(90000), "1d 1h".to_string());
    }

    #[test]
    fn test_seconds_to_string_86460() {
        assert_eq!(seconds_to_string(86460), "1d 1m".to_string());
    }

    #[test]
    fn test_seconds_to_string_90060() {
        assert_eq!(seconds_to_string(90060), "1d 1h 1m".to_string());
    }
}

