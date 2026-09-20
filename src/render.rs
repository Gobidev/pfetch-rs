use crate::config::Config;
use pfetch_logo_parser::{Color, Logo, LogoPart};
use unicode_width::UnicodeWidthStr;

pub fn pfetch(
    info: Vec<(Color, String, String)>,
    logo: Logo,
    logo_enabled: bool,
    config: &Config,
) {
    let (display_lines, width_per_line): (Vec<String>, Vec<usize>) =
        if let Some(raw) = &config.raw_logo {
            let display: Vec<String> = raw.lines().map(|s| s.to_string()).collect();
            let widths: Vec<usize> = display
                .iter()
                .map(|line| strip_ansi(line).width())
                .collect();
            (display, widths)
        } else {
            let colored_logo = if config.color {
                logo.to_string()
            } else {
                format!("{:#}", logo)
            };
            let raw_text = logo
                .logo_parts
                .iter()
                .map(|LogoPart { content, .. }| content.as_ref())
                .collect::<String>();
            let display: Vec<String> = colored_logo.lines().map(|s| s.to_string()).collect();
            let widths: Vec<usize> = raw_text.lines().map(|s| s.width()).collect();
            (display, widths)
        };

    let logo_width = *width_per_line.iter().max().unwrap_or(&0);
    let line_amount = usize::max(display_lines.len(), info.len());

    let info1_width = info
        .iter()
        .skip(1)
        .map(|(_, line, _)| {
            if line.starts_with("\x1b[4") {
                0
            } else {
                line.len()
            }
        })
        .max()
        .unwrap_or(0);

    let bold = if config.color { "\x1b[1m" } else { "" };
    let reset = if config.color { "\x1b[0m" } else { "" };
    let col2_str = if config.color {
        config.col2.map(|c| c.to_string()).unwrap_or_default()
    } else {
        String::new()
    };

    let mut output = String::new();

    for l in 0..line_amount {
        let (line_color, info1, info2) = info
            .get(l)
            .map_or((Color(None), "", ""), |(c, i1, i2)| (*c, i1.as_str(), i2.as_str()));

        output.push_str(&" ".repeat(config.pad1));
        output.push_str(bold);
        if logo_enabled {
            output.push_str(display_lines.get(l).map_or("", |s| s.as_str()));
        }

        let current_width = width_per_line.get(l).copied().unwrap_or(0);
        let padding2 = logo_width.saturating_sub(current_width)
            + if logo_enabled { config.pad2 } else { 0 };
        output.push_str(&" ".repeat(padding2));

        if config.color {
            output.push_str(&line_color.to_string());
        }
        output.push_str(info1);
        output.push_str(reset);

        if !info2.is_empty() {
            output.push_str(&config.sep);
        }

        let padding3 = info1_width.saturating_sub(info1.len()) + config.pad3;
        output.push_str(&" ".repeat(padding3));

        if config.color {
            output.push_str(&col2_str);
        }
        output.push_str(info2);
        output.push('\n');
    }

    if config.stdout_is_terminal {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap);
    }

    print!("{output}");

    if config.stdout_is_terminal {
        let _ = crossterm::execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap);
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
}
