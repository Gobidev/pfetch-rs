use regex::Regex;

use std::{borrow::Cow, fmt::Display, str::FromStr, sync::OnceLock};

#[cfg(feature = "proc-macro")]
use proc_macro2::TokenStream;
#[cfg(feature = "proc-macro")]
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Option<u8>);

#[cfg(feature = "proc-macro")]
impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = match &self.0 {
            Some(val) => quote! { Some(#val) },
            None => quote! { None },
        };
        tokens.append_all(quote! {
            ::pfetch_logo_parser::Color(#value)
        });
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(color @ 0..=7) => write!(f, "\x1b[3{color}m"),
            Some(color) => write!(f, "\x1b[38;5;{color}m"),
            None => write!(f, "\x1b[39m"),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("No string given".to_string());
        }
        s.parse::<u8>()
            .map(|value| Color(Some(value)))
            .map_err(|_| format!("'{s}' is not a valid color (expected a number)"))
    }
}

#[derive(Clone, Debug)]
pub struct LogoPart {
    pub color: Color,
    pub content: Cow<'static, str>,
}

#[cfg(feature = "proc-macro")]
impl ToTokens for LogoPart {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let color = &self.color;
        let content = &self.content;
        tokens.append_all(quote! {
            ::pfetch_logo_parser::LogoPart {
                color: #color,
                content: ::std::borrow::Cow::Borrowed(#content),
            }
        });
    }
}

#[derive(Clone, Debug)]
pub struct Logo {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub pattern: Cow<'static, str>,
    pub logo_parts: Cow<'static, [LogoPart]>,
}

#[cfg(feature = "proc-macro")]
impl ToTokens for Logo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let primary_color = &self.primary_color;
        let secondary_color = &self.secondary_color;
        let pattern = &self.pattern;
        let logo_parts = &self.logo_parts;

        tokens.append_all(quote! {
            ::pfetch_logo_parser::Logo {
                primary_color: #primary_color,
                secondary_color: #secondary_color,
                pattern: ::std::borrow::Cow::Borrowed(#pattern),
                logo_parts: ::std::borrow::Cow::Borrowed(&[#(#logo_parts),*]),
            }
        });
    }
}

impl Display for Logo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for LogoPart { color, content } in self.logo_parts.iter() {
            if f.alternate() {
                write!(f, "{content}")?;
            } else {
                write!(f, "{color}{content}")?;
            }
        }
        Ok(())
    }
}

fn logo_regex() -> &'static Regex {
    static LOGO_REGEX: OnceLock<Regex> = OnceLock::new();
    LOGO_REGEX.get_or_init(|| Regex::new(r"^\(?(.*)\)[\s\S]*read_ascii *(\d)?").unwrap())
}

/// Parses a logo in pfetch format.
///
/// Panics on invalid input; use [`try_parse_logo`] for untrusted input.
pub fn parse_logo(input: &str) -> Option<(bool, Logo)> {
    try_parse_logo(input).unwrap_or_else(|err| panic!("{err}"))
}

/// Fallible version of [`parse_logo`]; returns `Ok(None)` for empty input.
pub fn try_parse_logo(input: &str) -> Result<Option<(bool, Logo)>, String> {
    let input = input.trim().replace('\t', "");
    if input.is_empty() {
        return Ok(None);
    }
    let regex = logo_regex();

    let groups = regex
        .captures(&input)
        .ok_or_else(|| "Error while parsing logo".to_string())?;

    let pattern = &groups[1];
    let primary_color = match groups.get(2) {
        Some(color) => color
            .as_str()
            .parse::<u8>()
            .map_err(|_| format!("Invalid color: {}", color.as_str()))?,
        None => 7,
    };
    let secondary_color = (primary_color + 1) % 8;
    let logo = input
        .split_once("EOF\n")
        .ok_or_else(|| {
            "Could not find start of logo, make sure to include the `<<- EOF` and to use tabs for indentation".to_string()
        })?
        .1
        .split_once("\nEOF")
        .ok_or_else(|| {
            "Could not find end of logo, make sure to include the closing EOF and to use tabs for indentation".to_string()
        })?
        .0;

    let mut logo_parts = vec![];
    for logo_part in logo.split("${") {
        if let Some((new_color, rest)) = logo_part.split_once('}') {
            let new_color: u8 = new_color
                .get(1..)
                .and_then(|num| num.parse().ok())
                .ok_or_else(|| format!("Invalid color: {new_color}"))?;
            let rest = rest.replace("\\\\", "\\");
            let rest = rest.replace("\\`", "`");
            let lines = rest.split('\n').collect::<Vec<_>>();
            let last_index = lines.len() - 1;
            for (index, line) in lines.into_iter().enumerate() {
                let mut line = line.to_owned();
                if index != last_index {
                    line += "\n";
                }
                logo_parts.push(LogoPart {
                    color: Color(Some(new_color)),
                    content: line.into(),
                });
            }
        } else if !logo_part.is_empty() {
            let logo_part = logo_part.replace("\\\\", "\\");
            logo_parts.push(LogoPart {
                color: Color(None),
                content: logo_part.into(),
            });
        }
    }

    Ok(Some((
        pattern == "[Ll]inux*",
        Logo {
            primary_color: Color(Some(primary_color)),
            secondary_color: Color(Some(secondary_color)),
            pattern: pattern.to_owned().into(),
            logo_parts: logo_parts.into(),
        },
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "[Aa]rch*)\n\tread_ascii 1 <<- EOF\n\t\t${c1}  /\\\n\t\t${c1} /  \\\n\tEOF\n";

    #[test]
    fn test_parse_logo() {
        let (is_tux, logo) = parse_logo(SAMPLE).unwrap();
        assert!(!is_tux);
        assert_eq!(logo.pattern, "[Aa]rch*");
        assert_eq!(logo.primary_color.0, Some(1));
        assert_eq!(logo.secondary_color.0, Some(2));
        assert!(!logo.logo_parts.is_empty());
    }

    #[test]
    fn test_parse_tux_logo() {
        let input = "[Ll]inux*)\n\tread_ascii 6 <<- EOF\n\t\t${c6}TUX\n\tEOF\n";
        let (is_tux, logo) = parse_logo(input).unwrap();
        assert!(is_tux);
        assert_eq!(logo.primary_color.0, Some(6));
    }

    #[test]
    fn test_logo_display() {
        let (_, logo) = parse_logo(SAMPLE).unwrap();

        let colored = logo.to_string();
        let plain = format!("{logo:#}");

        assert!(colored.contains("\x1b[31m"));
        assert!(!plain.contains('\x1b'));
        assert_eq!(colored.replace("\x1b[31m", ""), plain);
    }

    #[test]
    fn test_parse_logo_empty() {
        assert!(try_parse_logo(" \n\t").unwrap().is_none());
        assert!(parse_logo("").is_none());
    }

    #[test]
    fn test_try_parse_logo_missing_eof() {
        let err =
            try_parse_logo("[Aa]rch*)\n\tread_ascii 1 <<- EOF\n\t\t${c1}  /\\\n").unwrap_err();
        assert!(err.contains("Could not find end of logo"), "{err}");
    }

    #[test]
    fn test_try_parse_logo_missing_read_ascii() {
        let err = try_parse_logo("[Aa]rch*)").unwrap_err();
        assert!(err.contains("Error while parsing logo"), "{err}");
    }

    #[test]
    fn test_try_parse_logo_missing_start_of_logo() {
        let err = try_parse_logo("[Aa]rch*)\n\tread_ascii 1").unwrap_err();
        assert!(err.contains("Could not find start of logo"), "{err}");
    }

    #[test]
    fn test_try_parse_logo_invalid_color() {
        let input = "[Aa]rch*)\n\tread_ascii 1 <<- EOF\n\t\t${cx}  /\\\n\tEOF\n";
        let err = try_parse_logo(input).unwrap_err();
        assert!(err.contains("Invalid color"), "{err}");
    }

    #[test]
    fn test_parse_logo_panics_on_invalid_input() {
        let result = std::panic::catch_unwind(|| parse_logo("[Aa]rch*)"));
        assert!(result.is_err());
    }
}
