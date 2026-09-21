use crate::cli::Args;
use pfetch_logo_parser::Color;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{IsTerminal, Read},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone)]
pub enum PfetchInfo {
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
        let name = match self {
            PfetchInfo::Ascii => "ascii",
            PfetchInfo::Title => "title",
            PfetchInfo::Os => "os",
            PfetchInfo::Host => "host",
            PfetchInfo::Kernel => "kernel",
            PfetchInfo::Uptime => "uptime",
            PfetchInfo::Pkgs => "pkgs",
            PfetchInfo::Cpu => "cpu",
            PfetchInfo::Memory => "memory",
            PfetchInfo::Shell => "shell",
            PfetchInfo::Editor => "editor",
            PfetchInfo::Wm => "wm",
            PfetchInfo::De => "de",
            PfetchInfo::Palette => "palette",
            PfetchInfo::BlankLine => "blankline",
        };
        write!(f, "{name}")
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

#[derive(Debug)]
pub struct Config {
    pub args: Args,
    pub ascii: Option<String>,
    pub col1: Option<Color>,
    pub col2: Option<Color>,
    pub col3: Option<Color>,
    pub col3_is_col1: bool,
    pub color: bool,
    pub sep: String,
    pub pad1: usize,
    pub pad2: usize,
    pub pad3: usize,
    pub user: Option<String>,
    pub hostname: Option<String>,
    pub fast_pkg_count: bool,
    pub info: Vec<PfetchInfo>,
    pub custom_logos: Option<String>,
    pub raw_logo: Option<String>,
    pub print_version: bool,
    pub print_help: bool,
    pub stdout_is_terminal: bool,
}

impl Config {
    pub fn load() -> Self {
        let args = crate::cli::parse_args();
        if args.print_version || args.print_help {
            return Self::for_help_or_version(args);
        }
        let source = args
            .source
            .clone()
            .or_else(|| dotenvy::var("PF_SOURCE").ok());
        if let Some(filepath) = source {
            let _ = dotenvy::from_path(pfetch::expand_tilde(&filepath));
        }
        let stdout_is_terminal = std::io::stdout().is_terminal();
        Self::from_env(std::env::vars(), args, stdout_is_terminal)
    }

    fn for_help_or_version(args: Args) -> Self {
        let stdout_is_terminal = std::io::stdout().is_terminal();
        Self {
            ascii: None,
            col1: None,
            col2: None,
            col3: None,
            col3_is_col1: false,
            color: false,
            sep: String::new(),
            pad1: 0,
            pad2: 3,
            pad3: 1,
            user: None,
            hostname: None,
            fast_pkg_count: false,
            info: Vec::new(),
            custom_logos: None,
            raw_logo: None,
            print_version: args.print_version,
            print_help: args.print_help,
            stdout_is_terminal,
            args,
        }
    }

    pub fn from_env(
        env: impl Iterator<Item = (String, String)>,
        args: Args,
        stdout_is_terminal: bool,
    ) -> Self {
        let env: HashMap<String, String> = env.collect();

        let parse_color =
            |key: &str| -> Option<Color> { env.get(key).and_then(|s| Color::from_str(s).ok()) };

        let col1 = parse_color("PF_COL1");
        let col3_is_col1 = env.get("PF_COL3").map(|s| s.as_str()) == Some("COL1");
        let col3 = if col3_is_col1 {
            None
        } else {
            parse_color("PF_COL3")
        };

        let mut config = Self {
            args: Args::default(),
            ascii: env.get("PF_ASCII").cloned(),
            col1,
            col2: parse_color("PF_COL2"),
            col3,
            col3_is_col1,
            color: env.get("PF_COLOR").map(|s| s.as_str()) != Some("0") && stdout_is_terminal,
            sep: env.get("PF_SEP").cloned().unwrap_or_default(),
            pad1: env.get("PF_PAD1").and_then(|s| s.parse().ok()).unwrap_or(0),
            pad2: env.get("PF_PAD2").and_then(|s| s.parse().ok()).unwrap_or(3),
            pad3: env.get("PF_PAD3").and_then(|s| s.parse().ok()).unwrap_or(1),
            user: env.get("USER").cloned(),
            hostname: env.get("HOSTNAME").cloned(),
            fast_pkg_count: env.contains_key("PF_FAST_PKG_COUNT"),
            info: parse_pf_info(env.get("PF_INFO").map(|s| s.as_str())),
            custom_logos: env.get("PF_CUSTOM_LOGOS").cloned(),
            raw_logo: None,
            print_version: args.print_version,
            print_help: args.print_help,
            stdout_is_terminal,
        };

        if let Some(path) = &args.raw_logo {
            config.raw_logo = Some(read_raw_logo(path).unwrap_or_else(|err| {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }));
        }

        config.apply_cli_overrides(&args);
        config.args = args;
        config
    }

    fn apply_cli_overrides(&mut self, args: &Args) {
        if let Some(logo) = &args.logo {
            self.ascii = Some(logo.clone());
        }
        if let Some(color) = args.color {
            self.color = color;
        }
        if let Some(info) = &args.info {
            self.info = parse_pf_info(Some(info));
        }
        if let Some(sep) = &args.sep {
            self.sep = sep.clone();
        }
    }
}

fn read_raw_logo(path: &str) -> Result<String, String> {
    if path == "-" {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Could not read logo from stdin: {e}"))?;
        Ok(buffer)
    } else {
        std::fs::read_to_string(pfetch::expand_tilde(path))
            .map_err(|e| format!("Could not read logo file '{path}': {e}"))
    }
}

fn parse_pf_info(info: Option<&str>) -> Vec<PfetchInfo> {
    let base: Vec<PfetchInfo> = match info {
        Some(pfetch_infos) => pfetch_infos
            .trim()
            .split(' ')
            .map(PfetchInfo::from_str)
            .filter_map(|i| i.ok())
            .collect(),
        None => vec![
            PfetchInfo::Ascii,
            PfetchInfo::Title,
            PfetchInfo::Os,
            PfetchInfo::Host,
            PfetchInfo::Kernel,
            PfetchInfo::Uptime,
            PfetchInfo::Pkgs,
            PfetchInfo::Memory,
        ],
    };

    let mut with_blanks = Vec::with_capacity(base.len() + 2);
    for item in base {
        match item {
            PfetchInfo::Palette => {
                with_blanks.push(PfetchInfo::BlankLine);
                with_blanks.push(PfetchInfo::Palette);
                with_blanks.push(PfetchInfo::BlankLine);
            }
            i => with_blanks.push(i),
        }
    }
    with_blanks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_env<'a>(
        pairs: &'a [(&'a str, &'a str)],
    ) -> impl Iterator<Item = (String, String)> + 'a {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string()))
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::from_env(test_env(&[]), Args::default(), true);
        assert!(config.color);
        assert_eq!(config.pad1, 0);
        assert_eq!(config.pad2, 3);
        assert_eq!(config.pad3, 1);
        assert!(config.info.contains(&PfetchInfo::Ascii));
    }

    #[test]
    fn test_config_color_disabled() {
        let config = Config::from_env(test_env(&[("PF_COLOR", "0")]), Args::default(), true);
        assert!(!config.color);
    }

    #[test]
    fn test_config_color_non_tty() {
        let config = Config::from_env(test_env(&[]), Args::default(), false);
        assert!(!config.color);
    }

    #[test]
    fn test_config_palette_inserts_blank_lines() {
        let config = Config::from_env(test_env(&[("PF_INFO", "palette")]), Args::default(), true);
        assert_eq!(
            config.info,
            vec![
                PfetchInfo::BlankLine,
                PfetchInfo::Palette,
                PfetchInfo::BlankLine,
            ]
        );
    }

    #[test]
    fn test_config_col3_col1() {
        let config = Config::from_env(
            test_env(&[("PF_COL1", "4"), ("PF_COL3", "COL1")]),
            Args::default(),
            true,
        );
        assert!(config.col3_is_col1);
        assert_eq!(config.col1.map(|c| c.0), Some(Some(4)));
        assert!(config.col3.is_none());
    }

    #[test]
    fn test_config_paddings() {
        let config = Config::from_env(
            test_env(&[("PF_PAD1", "2"), ("PF_PAD2", "5"), ("PF_PAD3", "7")]),
            Args::default(),
            true,
        );
        assert_eq!(config.pad1, 2);
        assert_eq!(config.pad2, 5);
        assert_eq!(config.pad3, 7);
    }

    #[test]
    fn test_cli_overrides() {
        let args = Args {
            logo: Some("openbsd".to_string()),
            color: Some(true),
            info: Some("title os".to_string()),
            sep: Some("~".to_string()),
            ..Args::default()
        };
        let config = Config::from_env(
            test_env(&[("PF_ASCII", "arch"), ("PF_SEP", ":"), ("PF_INFO", "memory")]),
            args,
            true,
        );
        assert_eq!(config.ascii, Some("openbsd".to_string()));
        assert!(config.color);
        assert_eq!(config.sep, "~");
        assert_eq!(config.info, vec![PfetchInfo::Title, PfetchInfo::Os]);
    }

    #[test]
    fn test_pfetch_info_display() {
        assert_eq!(PfetchInfo::Os.to_string(), "os");
        assert_eq!(PfetchInfo::Memory.to_string(), "memory");
        assert_eq!(PfetchInfo::Palette.to_string(), "palette");
    }
}
