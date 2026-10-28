use crate::config::Config;
use pfetch_logo_parser::{Color, Logo};
use std::fmt::Write as _;
use unicode_width::UnicodeWidthStr;

pub enum InfoLine {
    Blank,
    Standalone {
        color: Color,
        text: String,
    },
    KeyValue {
        color: Color,
        key: String,
        value: String,
    },
}

fn label_width(info: &[InfoLine]) -> usize {
    info.iter()
        .filter_map(|line| match line {
            InfoLine::KeyValue { key, value, .. } if !value.is_empty() => Some(key.len()),
            InfoLine::KeyValue { .. } | InfoLine::Blank | InfoLine::Standalone { .. } => None,
        })
        .max()
        .unwrap_or(0)
}

pub fn pfetch(info: &[InfoLine], logo: Logo, logo_enabled: bool, config: &Config) {
    let output = render(info, &logo, logo_enabled, config);

    if config.stdout_is_terminal {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap);
    }

    print!("{output}");

    if config.stdout_is_terminal {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap);
    }
}

fn render(info: &[InfoLine], logo: &Logo, logo_enabled: bool, config: &Config) -> String {
    let (display_lines, width_per_line): (Vec<String>, Vec<usize>) = if !logo_enabled {
        (Vec::new(), Vec::new())
    } else if let Some(raw) = &config.raw_logo {
        let display: Vec<String> = raw.lines().map(|s| s.to_string()).collect();
        let widths: Vec<usize> = display
            .iter()
            .map(|line| strip_ansi(line).width())
            .collect();
        (display, widths)
    } else {
        let plain_logo = format!("{:#}", logo);
        let widths: Vec<usize> = plain_logo.lines().map(|s| s.width()).collect();
        let display: Vec<String> = if config.color {
            logo.to_string()
        } else {
            plain_logo
        }
        .lines()
        .map(|s| s.to_string())
        .collect();
        (display, widths)
    };

    let logo_width = *width_per_line.iter().max().unwrap_or(&0);
    let line_amount = if logo_enabled {
        usize::max(display_lines.len(), info.len())
    } else {
        info.len()
    };

    let max_label_width = label_width(info);

    let bold = if config.color { "\x1b[1m" } else { "" };
    let reset = if config.color { "\x1b[0m" } else { "" };
    let col2_str = if config.color {
        config.col2.map(|c| c.to_string()).unwrap_or_default()
    } else {
        String::new()
    };

    let mut output = String::new();
    let pad1 = " ".repeat(config.pad1);

    for l in 0..line_amount {
        let (line_color, text, key_value) = match info.get(l) {
            Some(InfoLine::KeyValue { color, key, value }) => {
                (Some(*color), key.as_str(), Some(value.as_str()))
            }
            Some(InfoLine::Standalone { color, text }) => (Some(*color), text.as_str(), None),
            Some(InfoLine::Blank) => (None, "", None),
            None => (Some(Color(None)), "", None),
        };

        output.push_str(&pad1);
        output.push_str(bold);
        if logo_enabled {
            output.push_str(display_lines.get(l).map_or("", |s| s.as_str()));
        }

        let padding2 = if logo_enabled {
            let current_width = width_per_line.get(l).copied().unwrap_or(0);
            logo_width.saturating_sub(current_width) + config.pad2
        } else {
            0
        };
        push_spaces(&mut output, padding2);

        if config.color {
            if let Some(color) = line_color {
                let _ = write!(output, "{color}");
            }
        }
        output.push_str(text);
        output.push_str(reset);

        if let Some(value) = key_value {
            if !value.is_empty() {
                output.push_str(&config.sep);

                let padding3 = max_label_width.saturating_sub(text.len()) + config.pad3;
                push_spaces(&mut output, padding3);

                if config.color {
                    output.push_str(&col2_str);
                }
                output.push_str(value);
            }
        }
        output.push('\n');
    }

    output
}

fn push_spaces(output: &mut String, amount: usize) {
    for _ in 0..amount {
        output.push(' ');
    }
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(&next) = chars.peek() {
                chars.next();
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(strip_ansi("no ansi"), "no ansi");
    }

    fn key_value(key: &str) -> InfoLine {
        InfoLine::KeyValue {
            color: Color(Some(1)),
            key: key.to_string(),
            value: "value".to_string(),
        }
    }

    fn key_value_with(key: &str, value: &str) -> InfoLine {
        InfoLine::KeyValue {
            color: Color(Some(1)),
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    fn empty_logo() -> Logo {
        Logo {
            primary_color: Color(None),
            secondary_color: Color(None),
            pattern: std::borrow::Cow::Borrowed(""),
            logo_parts: std::borrow::Cow::Borrowed(&[]),
        }
    }

    fn test_config() -> Config {
        Config::from_env(std::iter::empty(), crate::cli::Args::default(), false)
    }

    #[test]
    fn label_width_uses_key_value_rows_only() {
        let info = vec![key_value("os"), key_value("kernel")];
        assert_eq!(label_width(&info), 6);
    }

    #[test]
    fn label_width_ignores_empty_valued_key_value_rows() {
        let info = vec![key_value("os"), key_value_with("kernel", "")];
        assert_eq!(label_width(&info), 2);
    }

    #[test]
    fn empty_valued_key_value_has_no_trailing_padding() {
        let config = test_config();
        let info = vec![key_value("os"), key_value_with("kernel", "")];
        let output = render(&info, &empty_logo(), false, &config);
        assert_eq!(output, "os value\nkernel\n");
    }

    #[test]
    fn label_width_ignores_standalone_rows() {
        let info = vec![
            InfoLine::Standalone {
                color: Color(Some(2)),
                text: "user@long-hostname".to_string(),
            },
            key_value("kernel"),
            key_value("os"),
        ];
        assert_eq!(label_width(&info), 6);
    }

    #[test]
    fn label_width_ignores_blank_rows() {
        let info = vec![InfoLine::Blank, key_value("os")];
        assert_eq!(label_width(&info), 2);
    }

    #[test]
    fn label_width_without_key_value_rows_is_zero() {
        let info = vec![
            InfoLine::Blank,
            InfoLine::Standalone {
                color: Color(Some(3)),
                text: "title".to_string(),
            },
        ];
        assert_eq!(label_width(&info), 0);
    }

    #[test]
    fn label_width_of_empty_input_is_zero() {
        assert_eq!(label_width(&[]), 0);
    }
}
