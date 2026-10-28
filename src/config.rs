use crate::cli::Args;
use pfetch::HostField;
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
            "blankline" => Ok(PfetchInfo::BlankLine),
            unknown_info => Err(format!("Unknown pfetch info: {unknown_info}")),
        }
    }
}

#[derive(Debug)]
pub struct Config {
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
    pub host_fields: Vec<HostField>,
    pub custom_logos: Option<String>,
    pub raw_logo: Option<String>,
    pub stdout_is_terminal: bool,
}

impl Config {
    pub fn load(args: Args) -> Self {
        let source = args
            .source
            .clone()
            .or_else(|| std::env::var("PF_SOURCE").ok());
        if let Some(filepath) = source {
            if let Err(err) = dotenvy::from_path(pfetch::expand_tilde(&filepath)) {
                eprintln!("Warning: Could not load source file '{filepath}': {err}");
            }
        }
        let stdout_is_terminal = std::io::stdout().is_terminal();
        Self::from_env(std::env::vars(), args, stdout_is_terminal)
    }

    pub fn from_env(
        env: impl Iterator<Item = (String, String)>,
        args: Args,
        stdout_is_terminal: bool,
    ) -> Self {
        let env: HashMap<String, String> = env.collect();

        let parse_color = |key: &str| -> Option<Color> {
            env.get(key).and_then(|s| match Color::from_str(s) {
                Ok(color) => Some(color),
                Err(err) => {
                    eprintln!("Warning: Ignoring {key}: {err}");
                    None
                }
            })
        };

        let col1 = parse_color("PF_COL1");
        let col3_is_col1 = env.get("PF_COL3").map(|s| s.as_str()) == Some("COL1");
        let col3 = if col3_is_col1 {
            None
        } else {
            parse_color("PF_COL3")
        };

        let mut config = Self {
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
            host_fields: parse_pf_host(env.get("PF_HOST").map(|s| s.as_str())),
            custom_logos: env.get("PF_CUSTOM_LOGOS").cloned(),
            raw_logo: None,
            stdout_is_terminal,
        };

        if let Some(path) = &args.raw_logo {
            config.raw_logo = Some(read_raw_logo(path).unwrap_or_else(|err| {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }));
        }

        config.apply_cli_overrides(&args);
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
            .split_whitespace()
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

fn parse_pf_host(host: Option<&str>) -> Vec<HostField> {
    match host {
        Some(fields) => fields
            .split_whitespace()
            .filter_map(|field| field.parse().ok())
            .collect(),
        None => HostField::ALL.to_vec(),
    }
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
    fn test_config_invalid_color_is_ignored() {
        let config = Config::from_env(
            test_env(&[("PF_COL1", "not-a-color")]),
            Args::default(),
            true,
        );
        assert!(config.col1.is_none());
    }

    #[test]
    fn test_config_blankline_info() {
        let config = Config::from_env(
            test_env(&[("PF_INFO", "title blankline os")]),
            Args::default(),
            true,
        );
        assert_eq!(
            config.info,
            vec![PfetchInfo::Title, PfetchInfo::BlankLine, PfetchInfo::Os]
        );
    }

    #[test]
    fn test_config_info_splits_on_whitespace() {
        let config = Config::from_env(test_env(&[("PF_INFO", "title\tos")]), Args::default(), true);
        assert_eq!(config.info, vec![PfetchInfo::Title, PfetchInfo::Os]);
    }

    #[test]
    fn test_config_empty_info() {
        let config = Config::from_env(test_env(&[("PF_INFO", "")]), Args::default(), true);
        assert!(config.info.is_empty());
    }

    #[test]
    fn test_read_raw_logo_from_file() {
        let path = std::env::temp_dir().join(format!("pfetch-raw-logo-{}", std::process::id()));
        std::fs::write(&path, "logo contents").unwrap();
        let result = read_raw_logo(path.to_str().unwrap());
        std::fs::remove_file(&path).unwrap();
        assert_eq!(result, Ok("logo contents".to_string()));
    }

    #[test]
    fn test_read_raw_logo_missing_file() {
        let path = std::env::temp_dir().join(format!("pfetch-missing-logo-{}", std::process::id()));
        let path = path.to_str().unwrap();
        let err = read_raw_logo(path).unwrap_err();
        assert!(err.contains(path));
    }

    #[test]
    fn test_config_default_host_fields() {
        let config = Config::from_env(test_env(&[]), Args::default(), true);
        assert_eq!(config.host_fields, HostField::ALL.to_vec());
    }

    #[test]
    fn test_config_host_fields() {
        let config = Config::from_env(
            test_env(&[("PF_HOST", "  version\tmodel ")]),
            Args::default(),
            true,
        );
        assert_eq!(
            config.host_fields,
            vec![HostField::Version, HostField::Model]
        );
    }

    #[test]
    fn test_config_invalid_host_fields_are_ignored() {
        let config = Config::from_env(
            test_env(&[("PF_HOST", "name bogus")]),
            Args::default(),
            true,
        );
        assert_eq!(config.host_fields, vec![HostField::Name]);
    }

    #[test]
    fn test_config_empty_host_fields() {
        for value in ["", "   "] {
            let config = Config::from_env(test_env(&[("PF_HOST", value)]), Args::default(), true);
            assert!(config.host_fields.is_empty());
        }
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
