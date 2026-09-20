use libmacchina::{
    traits::GeneralReadout as _, traits::KernelReadout as _, traits::MemoryReadout as _,
    traits::PackageReadout as _, GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};
use pfetch_logo_parser::Color;
use std::env;

mod cli;
mod config;
mod render;

use config::{Config, PfetchInfo};

struct Readouts {
    general_readout: GeneralReadout,
    package_readout: PackageReadout,
    memory_readout: MemoryReadout,
    kernel_readout: KernelReadout,
}

fn get_info(info: &PfetchInfo, readouts: &Readouts, config: &Config) -> Option<String> {
    match info {
        PfetchInfo::Ascii => None,
        PfetchInfo::Os => pfetch::os(&readouts.general_readout),
        PfetchInfo::Title => pfetch::user_at_hostname(
            &readouts.general_readout,
            &config.user,
            &config.hostname,
        ),
        PfetchInfo::Host => pfetch::host(&readouts.general_readout),
        PfetchInfo::Kernel => pfetch::kernel(&readouts.kernel_readout),
        PfetchInfo::Uptime => pfetch::uptime(&readouts.general_readout),
        PfetchInfo::Pkgs => Some(
            pfetch::total_packages(&readouts.package_readout, config.fast_pkg_count).to_string(),
        ),
        PfetchInfo::Cpu => pfetch::cpu(&readouts.general_readout),
        PfetchInfo::Memory => pfetch::memory(&readouts.memory_readout),
        PfetchInfo::Shell => pfetch::shell(&readouts.general_readout),
        PfetchInfo::Editor => pfetch::editor(),
        PfetchInfo::Wm => pfetch::wm(&readouts.general_readout),
        PfetchInfo::De => pfetch::de(&readouts.general_readout),
        PfetchInfo::Palette => Some(pfetch::palette(config.color)),
        PfetchInfo::BlankLine => Some("".to_string()),
    }
}

fn main() {
    let config = Config::load();

    if config.print_version {
        println!("pfetch-rs {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    if config.print_help {
        cli::print_unknown_args(&config.args);
        cli::print_help();
        std::process::exit(if config.args.unknown_args.is_empty() { 0 } else { 1 });
    }

    let general_readout = GeneralReadout::new();
    let os = pfetch::os(&general_readout).unwrap_or_default();

    let readouts = Readouts {
        general_readout,
        package_readout: PackageReadout::new(),
        memory_readout: MemoryReadout::new(),
        kernel_readout: KernelReadout::new(),
    };

    let logo_name = match config.ascii.as_deref() {
        Some(name) => name.to_owned(),
        None => match env::consts::OS {
            "linux" => os.clone(),
            other => other.to_owned(),
        },
    };

    let mut logo = pfetch::logo(&logo_name, config.custom_logos.as_deref());

    if let Some(color) = config.col1 {
        logo.primary_color = color;
    }
    if config.col3_is_col1 {
        logo.secondary_color = logo.primary_color;
    } else if let Some(color) = config.col3 {
        logo.secondary_color = color;
    }

    let gathered_pfetch_info: Vec<(Color, String, String)> = config
        .info
        .iter()
        .filter_map(|info| {
            get_info(info, &readouts, &config).map(|info_str| match info {
                PfetchInfo::Title => (logo.secondary_color, info_str, "".to_string()),
                PfetchInfo::BlankLine => (logo.primary_color, "".to_string(), "".to_string()),
                PfetchInfo::Palette => (logo.primary_color, info_str, "".to_string()),
                _ => (logo.primary_color, info.to_string(), info_str),
            })
        })
        .collect();

    let logo_enabled = config.raw_logo.is_some()
        || config
            .info
            .iter()
            .any(|x| matches!(x, PfetchInfo::Ascii));

    render::pfetch(gathered_pfetch_info, logo, logo_enabled, &config);
}
