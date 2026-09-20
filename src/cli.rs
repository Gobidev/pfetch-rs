use std::env;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Args {
    pub print_version: bool,
    pub print_help: bool,
    pub raw_logo: Option<String>,
    pub logo: Option<String>,
    pub color: Option<bool>,
    pub info: Option<String>,
    pub sep: Option<String>,
    pub source: Option<String>,
    pub unknown_args: Vec<String>,
}

fn parse_value_arg(iter: &mut impl Iterator<Item = String>, name: &str) -> String {
    if let Some(value) = iter.next() {
        value
    } else {
        eprintln!("Error: {name} requires an argument");
        std::process::exit(1);
    }
}

fn parse_key_value_arg(arg: &str) -> Option<String> {
    arg.split_once('=').map(|(_, value)| value.to_string())
}

pub fn parse_args() -> Args {
    parse_args_from(env::args().skip(1))
}

pub fn parse_args_from(iter: impl IntoIterator<Item = String>) -> Args {
    let mut args = Args::default();
    let mut iter = iter.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-v" | "--version" | "--v" => args.print_version = true,
            "-h" | "--help" => args.print_help = true,
            "--color" => args.color = Some(true),
            "--no-color" => args.color = Some(false),
            "--file-raw" | "--raw" => {
                args.raw_logo = Some(parse_value_arg(&mut iter, "--file-raw"))
            }
            arg if arg.starts_with("--file-raw=") || arg.starts_with("--raw=") => {
                args.raw_logo = parse_key_value_arg(arg);
            }
            "-l" | "--logo" | "--ascii" => args.logo = Some(parse_value_arg(&mut iter, &arg)),
            arg if arg.starts_with("--logo=") || arg.starts_with("--ascii=") => {
                args.logo = parse_key_value_arg(arg);
            }
            "--info" => args.info = Some(parse_value_arg(&mut iter, "--info")),
            arg if arg.starts_with("--info=") => args.info = parse_key_value_arg(arg),
            "--sep" => args.sep = Some(parse_value_arg(&mut iter, "--sep")),
            arg if arg.starts_with("--sep=") => args.sep = parse_key_value_arg(arg),
            "--source" => args.source = Some(parse_value_arg(&mut iter, "--source")),
            arg if arg.starts_with("--source=") => args.source = parse_key_value_arg(arg),
            _ => {
                args.unknown_args.push(arg);
                args.print_help = true;
            }
        }
    }

    args
}

pub fn print_unknown_args(args: &Args) {
    for arg in &args.unknown_args {
        eprintln!("error: unknown option '{arg}'");
    }
}

pub fn print_help() {
    println!("pfetch-rs {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Usage: pfetch [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -v, --version            Print version");
    println!("  -h, --help               Print help");
    println!("  -l, --logo, --ascii      Logo to display");
    println!("      --color              Force colors");
    println!("      --no-color           Disable colors");
    println!("      --info <items>       Info items to display");
    println!("      --sep <string>       Separator between label and value");
    println!("      --source <file>      Load environment from file");
    println!("      --file-raw <path>    Read raw logo from file or stdin (-)");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args<const N: usize>(args: [&str; N]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_version() {
        let args = parse_args_from(args(["-v"]));
        assert!(args.print_version);
        assert!(!args.print_help);
    }

    #[test]
    fn parse_help() {
        let args = parse_args_from(args(["--help"]));
        assert!(args.print_help);
    }

    #[test]
    fn parse_file_raw() {
        let args = parse_args_from(args(["--file-raw", "logo.txt"]));
        assert_eq!(args.raw_logo, Some("logo.txt".to_string()));
    }

    #[test]
    fn parse_logo() {
        let args = parse_args_from(args(["--logo", "openbsd"]));
        assert_eq!(args.logo, Some("openbsd".to_string()));
    }

    #[test]
    fn parse_color_flags() {
        assert_eq!(parse_args_from(args(["--color"])).color, Some(true));
        assert_eq!(parse_args_from(args(["--no-color"])).color, Some(false));
    }

    #[test]
    fn parse_info_and_sep() {
        let args = parse_args_from(args(["--info", "title os", "--sep", "~"]));
        assert_eq!(args.info, Some("title os".to_string()));
        assert_eq!(args.sep, Some("~".to_string()));
    }

    #[test]
    fn parse_unknown_arg() {
        let args = parse_args_from(args(["--bogus"]));
        assert!(args.print_help);
        assert_eq!(args.unknown_args, vec!["--bogus"]);
    }

    #[test]
    fn parse_key_value_form() {
        let args = parse_args_from(args(["--file-raw=logo.txt", "--logo=openbsd"]));
        assert_eq!(args.raw_logo, Some("logo.txt".to_string()));
        assert_eq!(args.logo, Some("openbsd".to_string()));
    }
}
