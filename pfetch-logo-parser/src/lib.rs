use regex::Regex;

use std::{borrow::Cow, fmt::Display, str::FromStr};

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
        Ok(Color(s.parse::<u8>().ok()))
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
        write!(
            f,
            "{}",
            self.logo_parts
                .iter()
                .map(|LogoPart { color, content }| format!("{color}{content}"))
                .collect::<String>()
        )
    }
}

/// Parses a logo in pfetch formant and returns wether it is the linux (tux) logo and the logo itself
pub fn parse_logo(input: &str) -> Option<(bool, Logo)> {
    let input = input.trim().replace('\t', "");
    if input.is_empty() {
        return None;
    }
    let regex = Regex::new(r"^\(?(.*)\)[\s\S]*read_ascii *(\d)? *(\d)?").unwrap();

    let groups = regex.captures(&input).expect("Error while parsing logo");

    let pattern = &groups[1];
    let primary_color = match groups.get(2) {
        Some(color) => color.as_str().parse::<u8>().unwrap(),
        None => 7,
    };
    let secondary_color = match groups.get(3) {
        Some(color) => color.as_str().parse::<u8>().unwrap(),
        None => (primary_color + 1) % 8,
    };
    let logo = input
        .split_once("EOF\n")
        .expect("Could not find start of logo, make sure to include the `<<- EOF` and to use tabs for indentation")
        .1
        .split_once("\nEOF")
        .expect("Could not find end of logo, make sure to include the closing EOF and to use tabs for indentation")
        .0;

    let mut logo_parts = vec![];
    for logo_part in logo.split("${") {
        if let Some((new_color, rest)) = logo_part.split_once('}') {
            let new_color: u8 = new_color
                .get(1..)
                .and_then(|num| num.parse().ok())
                .unwrap_or_else(|| panic!("Invalid color: {new_color}"));
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

    Some((
        pattern == "[Ll]inux*",
        Logo {
            primary_color: Color(Some(primary_color)),
            secondary_color: Color(Some(secondary_color)),
            pattern: pattern.to_owned().into(),
            logo_parts: logo_parts.into(),
        },
    ))
}
