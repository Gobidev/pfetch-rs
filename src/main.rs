use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    traits::PackageReadout as _, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};
use pfetch_logo_parser::{Color, Logo, LogoPart};
use std::{env, fmt::Display, str::FromStr};
use unicode_width::UnicodeWidthStr;

struct Config {
    pf_ascii: Option<String>,
    pf_col1: String,
    pf_col2: String,
    pf_col3: String,
    pf_color: String,
    pf_sep: String,
    pf_pad1: String,
    pf_pad2: String,
    pf_pad3: String,
    user: String,
    hostname: String,
    pf_source: String,
    pf_fast_pkg_count: bool,
    pf_info: Vec<PfetchInfo>,
    pf_custom_logos: String,
    print_version: bool,
}

impl Config {
    fn load() -> Self {
        // source file specified by env: PF_SOURCE
        if let Ok(filepath) = dotenvy::var("PF_SOURCE") {
            let _ = dotenvy::from_path(filepath);
        }
        Self {
            pf_ascii: dotenvy::var("PF_ASCII").ok(),
            pf_col1: dotenvy::var("PF_COL1").unwrap_or_default(),
            pf_col2: dotenvy::var("PF_COL2").unwrap_or_default(),
            pf_col3: dotenvy::var("PF_COL3").unwrap_or_default(),
            pf_color: dotenvy::var("PF_COLOR").unwrap_or_default(),
            pf_sep: dotenvy::var("PF_SEP").unwrap_or_default(),
            pf_pad1: dotenvy::var("PF_PAD1").unwrap_or_default(),
            pf_pad2: dotenvy::var("PF_PAD2").unwrap_or_default(),
            pf_pad3: dotenvy::var("PF_PAD3").unwrap_or_default(),
            user: dotenvy::var("USER").unwrap_or_default(),
            hostname: dotenvy::var("HOSTNAME").unwrap_or_default(),
            pf_source: dotenvy::var("PF_SOURCE").unwrap_or_default(),
            pf_fast_pkg_count: dotenvy::var("PF_FAST_PKG_COUNT").is_ok(),
            pf_info: match dotenvy::var("PF_INFO") {
                Ok(pfetch_infos) => pfetch_infos
                    .trim()
                    .split(' ')
                    .map(PfetchInfo::from_str)
                    .filter_map(|i| i.ok())
                    .collect(),
                Err(_) => vec![
                    PfetchInfo::Ascii,
                    PfetchInfo::Title,
                    PfetchInfo::Os,
                    PfetchInfo::Host,
                    PfetchInfo::Kernel,
                    PfetchInfo::Uptime,
                    PfetchInfo::Pkgs,
                    PfetchInfo::Memory,
                ],
            },
            pf_custom_logos: dotenvy::var("PF_CUSTOM_LOGOS").unwrap_or_default(),
            print_version: std::env::args()
                .any(|arg| arg.starts_with("-v") || arg.starts_with("--v")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum PfetchInfo {
    Ascii,
    Title,
    Os,
    Host,
    Kernel,
    Uptime,
    Pkgs,
    Cpu,
    Memory,
    Shell,
    Editor,
    Wm,
    De,
    Palette,
    BlankLine,
}

impl Display for PfetchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

impl FromStr for PfetchInfo {
    type Err = String;

    fn from_str(info: &str) -> Result<Self, Self::Err> {
        match info {
            "ascii" => Ok(PfetchInfo::Ascii),
            "title" => Ok(PfetchInfo::Title),
            "os" => Ok(PfetchInfo::Os),
            "host" => Ok(PfetchInfo::Host),
            "kernel" => Ok(PfetchInfo::Kernel),
            "uptime" => Ok(PfetchInfo::Uptime),
            "pkgs" => Ok(PfetchInfo::Pkgs),
            "cpu" => Ok(PfetchInfo::Cpu),
            "memory" => Ok(PfetchInfo::Memory),
            "shell" => Ok(PfetchInfo::Shell),
            "editor" => Ok(PfetchInfo::Editor),
            "wm" => Ok(PfetchInfo::Wm),
            "de" => Ok(PfetchInfo::De),
            "palette" => Ok(PfetchInfo::Palette),
            unknown_info => Err(format!("Unknown pfetch info: {unknown_info}")),
        }
    }
}

fn pfetch(info: Vec<(Color, String, String)>, logo: Logo, logo_enabled: bool) {
    let raw_logo = if logo_enabled {
        logo.logo_parts
            .iter()
            .map(|LogoPart { content, .. }| content.as_ref())
            .collect::<String>()
    } else {
        "".to_string()
    };
    let color_enabled = dotenvy::var("PF_COLOR").unwrap_or_default() != "0";
    let logo = if color_enabled {
        logo.to_string()
    } else {
        format!("{:#}", logo)
    };
    let mut logo_lines = logo.lines();
    let raw_logo_lines: Vec<_> = raw_logo.lines().collect();
    let logo_width = raw_logo_lines
        .iter()
        .map(|line| line.width())
        .max()
        .unwrap_or(0);
    let line_amount = usize::max(raw_logo_lines.len(), info.len());

    let info1_width = info
        .iter()
        .skip(1)
        .map(|(_, line, _)| {
            if line.starts_with("\x1b[4") {
                // exclude palette from info1 width
                0
            } else {
                line.len()
            }
        })
        .max()
        .unwrap_or(0);

    let padding1 = match dotenvy::var("PF_PAD1") {
        Ok(padding0) => padding0.parse::<usize>().unwrap_or(0),
        Err(_) => 0,
    };
    let padding2 = match dotenvy::var("PF_PAD2") {
        Ok(padding1) => padding1.parse::<usize>().unwrap_or(0),
        Err(_) => 3,
    };
    let padding3 = match dotenvy::var("PF_PAD3") {
        Ok(padding2) => padding2.parse::<usize>().unwrap_or(0),
        Err(_) => 1,
    };

    let mut pfetch_str = String::new();

    for l in 0..line_amount {
        pfetch_str += &format!(
            "{padding1}{bold}{logo}{padding2}{color}{info1}{nobold}{separator}{padding3}{color2}{info2}\n",
            padding1 = " ".repeat(padding1),
            bold = if color_enabled {"\x1b[1m"} else {""},
            logo = if logo_enabled {
                logo_lines.next().unwrap_or("")
            } else {
                ""
            },
            padding2 = " ".repeat(
                logo_width - raw_logo_lines.get(l).map_or(0, |line| line.width())
                    + if logo_enabled { padding2 } else { 0 }
            ),
            color = if color_enabled {info.get(l).map_or("".to_owned(), |line| line.0.to_string())} else {"".to_string()},
            info1 = info.get(l).map_or("", |line| &line.1),
            nobold = if color_enabled {"\x1b[0m"} else {""},
            separator = info.get(l).map_or("".to_string(), |line|
                if ! &line.2.is_empty() {
                    dotenvy::var("PF_SEP").unwrap_or_default()
                } else { "".to_string() }
            ),
            padding3 = " ".repeat(
                info1_width.saturating_sub(info.get(l).map_or(0, |(_, line, _)| line.len()))
                    + padding3
            ),
            color2 = if color_enabled {match dotenvy::var("PF_COL2") {
                Ok(newcolor) => {
                    match Color::from_str(&newcolor) {
                        Ok(newcolor) => format!("{newcolor}"),
                        Err(_) => "".to_string(),
                    }
                },
                Err(_) => "".to_string()
            }} else {"".to_string()},
            info2 = info.get(l).map_or("", |line| &line.2)
        )
    }

    // if colors are disabled, remove them from string
    // if dotenvy::var("PF_COLOR").unwrap_or_default() == "0" {
    //     pfetch_str = pfetch_str
    //         .split("\x1b[")
    //         .map(|chunk| chunk.chars().skip(3).collect::<String>())
    //         .collect();
    // }

    // disable line wrap
    crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap)
        .unwrap_or_default();

    println!("{pfetch_str}");

    // enable line wrap
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap).unwrap_or_default();
}

struct Readouts {
    general_readout: GeneralReadout,
    package_readout: PackageReadout,
    memory_readout: MemoryReadout,
    kernel_readout: KernelReadout,
}

fn get_info(
    info: &PfetchInfo,
    readouts: &Readouts,
    skip_slow_package_managers: bool,
) -> Option<String> {
    match info {
        PfetchInfo::Ascii => None,
        PfetchInfo::Title => pfetch::user_at_hostname(
            &readouts.general_readout,
            &dotenvy::var("USER").ok(),
            &dotenvy::var("HOSTNAME").ok(),
        ),
        PfetchInfo::Os => pfetch::os(&readouts.general_readout),
        PfetchInfo::Host => pfetch::host(&readouts.general_readout),
        PfetchInfo::Kernel => pfetch::kernel(&readouts.kernel_readout),
        PfetchInfo::Uptime => pfetch::uptime(&readouts.general_readout),
        PfetchInfo::Pkgs => Some(
            pfetch::total_packages(&readouts.package_readout, skip_slow_package_managers)
                .to_string(),
        ),
        PfetchInfo::Cpu => pfetch::cpu(&readouts.general_readout),
        PfetchInfo::Memory => pfetch::memory(&readouts.memory_readout),
        PfetchInfo::Shell => pfetch::shell(&readouts.general_readout),
        PfetchInfo::Editor => pfetch::editor(),
        PfetchInfo::Wm => pfetch::wm(&readouts.general_readout),
        PfetchInfo::De => pfetch::de(&readouts.general_readout),
        PfetchInfo::Palette => Some(pfetch::palette()),
        PfetchInfo::BlankLine => Some("".to_string()),
    }
}

fn main() {
    let config = Config::load();
    if config.print_version {
        println!("pfetch-rs {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    } else if std::env::args().len() > 1 {
        // TODO: change this for raw logos
        println!("pfetch     show system information");
        println!("pfetch -v  show version");
        std::process::exit(0);
    }

    let readouts = Readouts {
        general_readout: GeneralReadout::new(),
        package_readout: PackageReadout::new(),
        memory_readout: MemoryReadout::new(),
        kernel_readout: KernelReadout::new(),
    };

    let os = pfetch::os(&GeneralReadout::new()).unwrap_or_default();

    let logo_name = config.pf_ascii.unwrap_or(match env::consts::OS {
        "linux" => os.clone(),
        other => other.to_owned(),
    });

    let mut logo = pfetch::logo(&logo_name);

    // color overrides
    if let Ok(newcolor) = Color::from_str(&config.pf_col1) {
        logo.primary_color = newcolor;
    }

    if config.pf_col3 == "COL1" {
        logo.secondary_color = logo.primary_color;
    } else if let Ok(newcolor) = Color::from_str(&config.pf_col3) {
        logo.secondary_color = newcolor;
    }

    let gathered_pfetch_info: Vec<(Color, String, String)> = config
        .pf_info
        .iter()
        .filter_map(|info| match info {
            PfetchInfo::Os => Some((logo.primary_color, info.to_string(), os.clone())),
            _ => get_info(info, &readouts, config.pf_fast_pkg_count).map(|info_str| match info {
                PfetchInfo::Title => (logo.secondary_color, info_str, "".to_string()),
                PfetchInfo::BlankLine => (logo.primary_color, "".to_string(), "".to_string()),
                PfetchInfo::Palette => (logo.primary_color, info_str, "".to_string()),
                _ => (logo.primary_color, info.to_string(), info_str),
            }),
        })
        .collect();

    pfetch(
        gathered_pfetch_info,
        logo,
        config
            .pf_info
            .iter()
            .any(|x| matches!(&x, PfetchInfo::Ascii)),
    );
}
