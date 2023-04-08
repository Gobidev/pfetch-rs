use std::{env, fmt::Display, fs, io::Result, process::Command, str::FromStr};

use glob::glob;
use globset::Glob;
use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    traits::PackageReadout as _, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};
use pfetch_extractor::parse_logos;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Option<u8>);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(color) => write!(f, "\x1b[38;5;{color}m"),
            None => write!(f, "\x1b[39m"),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Color(s.parse::<u8>().ok()))
    }
}

pub struct Logo {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub pattern: &'static str,
    pub logo_parts: &'static [(Color, &'static str)],
}

impl Display for Logo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.logo_parts
                .iter()
                .map(|(color, part)| format!("{color}{part}"))
                .collect::<String>()
        )
    }
}

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
        | PackageManager::Rpm
        | PackageManager::Portage
        | PackageManager::Opkg => get_macchina_package_count(
            macchina_package_count,
            &format!("{pkg_manager:?}").to_lowercase(),
        )
        .unwrap_or(0),
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
        // TODO: nix -q is very slow
        PackageManager::Nix => {
            if check_if_command_exists("nix-store") && !skip_slow {
                run_and_count_lines(
                    "nix-store",
                    &["-q", "--requisites", "/run/current-system/sw"],
                ) + run_and_count_lines(
                    "nix-store",
                    &[
                        "-q",
                        "--requisites",
                        &format!("{}/.nix-profile", env::var("HOME").unwrap_or_default()),
                    ],
                )
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

pub fn os(general_readout: &GeneralReadout) -> Option<String> {
    match env::consts::OS {
        "linux" => {
            // check for Bedrock Linux
            if dotenvy::var("PATH")
                .unwrap_or_default()
                .contains("/bedrock/cross/")
            {
                Some("Bedrock Linux".to_string())
            } else {
                match general_readout.distribution() {
                    Ok(distribution) => Some(distribution.replace(" TEMPLATE_VERSION_ID", "")),
                    Err(_) => None,
                }
            }
        }
        _ => match general_readout.os_name() {
            Ok(os) => Some(os),
            Err(_) => None,
        },
    }
}

pub fn kernel(kernel_readout: &KernelReadout) -> Option<String> {
    match kernel_readout.os_release() {
        Ok(kernel_version) => Some(kernel_version),
        Err(_) => None,
    }
}

fn seconds_to_string(time: usize) -> String {
    let days = if time > 86400 {
        let days_pre = time / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "".to_string()
    };
    let hours = if time > 3600 {
        let hours_pre = (time / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };
    let minutes = if time > 60 {
        let minutes_pre = (time / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "0m".to_string()
    };
    format!("{days} {hours} {minutes}").trim_start().to_owned()
}

pub fn uptime(general_readout: &GeneralReadout) -> Option<String> {
    match general_readout.uptime() {
        Ok(uptime) => Some(seconds_to_string(uptime)),
        Err(_) => None,
    }
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
        _ => match general_readout.machine() {
            Ok(host) => Some(host),
            Err(_) => match general_readout.cpu_model_name() {
                Ok(host) => Some(host),
                Err(_) => None,
            },
        },
    }
}

pub fn logo(logo_name: &str) -> Logo {
    let (tux, logos) = parse_logos!();
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
    match general_readout.shell(
        libmacchina::traits::ShellFormat::Relative,
        libmacchina::traits::ShellKind::Default,
    ) {
        Ok(shell) => Some(shell),
        Err(_) => match dotenvy::var("SHELL") {
            Ok(shell) => Some(shell),
            Err(_) => None,
        },
    }
}

pub fn editor() -> Option<String> {
    match env::var("VISUAL") {
        Ok(editor) => Some(editor.trim().to_owned()),
        Err(_) => match env::var("EDITOR") {
            Ok(editor) => Some(editor.trim().to_owned()),
            Err(_) => None,
        },
    }
}

pub fn wm(general_readout: &GeneralReadout) -> Option<String> {
    match general_readout.window_manager() {
        Ok(wm) => Some(wm),
        Err(_) => None,
    }
}

pub fn de(general_readout: &GeneralReadout) -> Option<String> {
    match general_readout.desktop_environment() {
        Ok(de) => Some(de),
        Err(_) => match dotenvy::var("XDG_CURRENT_DESKTOP") {
            Ok(de) => Some(de),
            Err(_) => None,
        },
    }
}

pub fn palette() -> String {
    (1..7)
        .map(|num| format!("\x1b[4{num}m  "))
        .collect::<String>()
        + "\x1b[0m"
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
