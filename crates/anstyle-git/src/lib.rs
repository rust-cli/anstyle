//! `anstyle_git::parse` parses a color configuration string (in Git syntax)
//! into an `anstyle::Style`:
//!
//! # Examples
//!
//! ```rust
//! let style = anstyle_git::parse("bold red blue").unwrap();
//! assert_eq!(style, anstyle::AnsiColor::Red.on(anstyle::AnsiColor::Blue) | anstyle::Effects::BOLD);
//!
//! let hyperlink_style = anstyle_git::parse("#0000ee ul").unwrap();
//! assert_eq!(hyperlink_style, anstyle::RgbColor(0x00, 0x00, 0xee).on_default() | anstyle::Effects::UNDERLINE);
//! ```

mod sealed {
    pub(crate) trait Sealed {}
}

trait Ext: sealed::Sealed + Sized {
    fn parse_git(s: &str) -> Result<Self, Error>;
}

impl sealed::Sealed for anstyle::Style {}

impl Ext for anstyle::Style {
    fn parse_git(s: &str) -> Result<Self, Error> {
        parse(s)
    }
}

/// Parse a string in Git's color configuration syntax into an
/// `anstyle::Style`.
pub fn parse(s: &str) -> Result<anstyle::Style, Error> {
    let mut style = anstyle::Style::new();
    let mut num_colors = 0;
    let mut effects = anstyle::Effects::new();
    for word in s.split_whitespace() {
        match word.to_lowercase().as_ref() {
            "nobold" | "no-bold" => {
                effects = effects.remove(anstyle::Effects::BOLD);
            }
            "bold" => {
                effects = effects.insert(anstyle::Effects::BOLD);
            }
            "nodim" | "no-dim" => {
                effects = effects.remove(anstyle::Effects::DIMMED);
            }
            "dim" => {
                effects = effects.insert(anstyle::Effects::DIMMED);
            }
            "noul" | "no-ul" => {
                effects = effects.remove(anstyle::Effects::UNDERLINE);
            }
            "ul" => {
                effects = effects.insert(anstyle::Effects::UNDERLINE);
            }
            "noblink" | "no-blink" => {
                effects = effects.remove(anstyle::Effects::BLINK);
            }
            "blink" => {
                effects = effects.insert(anstyle::Effects::BLINK);
            }
            "noreverse" | "no-reverse" => {
                effects = effects.remove(anstyle::Effects::INVERT);
            }
            "reverse" => {
                effects = effects.insert(anstyle::Effects::INVERT);
            }
            "noitalic" | "no-italic" => {
                effects = effects.remove(anstyle::Effects::ITALIC);
            }
            "italic" => {
                effects = effects.insert(anstyle::Effects::ITALIC);
            }
            "nostrike" | "no-strike" => {
                effects = effects.remove(anstyle::Effects::STRIKETHROUGH);
            }
            "strike" => {
                effects = effects.insert(anstyle::Effects::STRIKETHROUGH);
            }
            w => {
                if let Ok(color) = parse_color(w) {
                    match num_colors {
                        0 => {
                            style = style.fg_color(color);
                            num_colors += 1;
                        }
                        1 => {
                            style = style.bg_color(color);
                            num_colors += 1;
                        }
                        _ => {
                            return Err(Error::ExtraColor {
                                style: s.to_string(),
                                word: word.to_string(),
                            });
                        }
                    }
                } else {
                    return Err(Error::UnknownWord {
                        style: s.to_string(),
                        word: word.to_string(),
                    });
                }
            }
        }
    }
    style |= effects;
    Ok(style)
}

fn parse_color(word: &str) -> Result<Option<anstyle::Color>, ()> {
    let color = match word {
        "normal" => None,
        "-1" => None,
        "black" => Some(anstyle::AnsiColor::Black.into()),
        "red" => Some(anstyle::AnsiColor::Red.into()),
        "green" => Some(anstyle::AnsiColor::Green.into()),
        "yellow" => Some(anstyle::AnsiColor::Yellow.into()),
        "blue" => Some(anstyle::AnsiColor::Blue.into()),
        "magenta" => Some(anstyle::AnsiColor::Magenta.into()),
        "cyan" => Some(anstyle::AnsiColor::Cyan.into()),
        "white" => Some(anstyle::AnsiColor::White.into()),
        _ => {
            if word.starts_with('#') && word.len() == 7 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&word[1..3], 16),
                    u8::from_str_radix(&word[3..5], 16),
                    u8::from_str_radix(&word[5..7], 16),
                ) {
                    Some(anstyle::Color::from((r, g, b)))
                } else {
                    return Err(());
                }
            } else if let Ok(n) = word.parse::<u8>() {
                Some(anstyle::Color::from(n))
            } else {
                return Err(());
            }
        }
    };
    Ok(color)
}

/// Type for errors returned by the parser.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// An extra color appeared after the foreground and background colors.
    ExtraColor { style: String, word: String },
    /// An unknown word appeared.
    UnknownWord { style: String, word: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ExtraColor { style, word } => {
                write!(
                    fmt,
                    "Error parsing style \"{}\": extra color \"{}\"",
                    style, word
                )
            }
            Self::UnknownWord { style, word } => {
                write!(
                    fmt,
                    "Error parsing style \"{}\": unknown word: \"{}\"",
                    style, word
                )
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::Error::*;
    use super::*;

    use anstyle::AnsiColor::*;
    use anstyle::*;

    #[test]
    fn test_parse_style() {
        macro_rules! test {
            ($s:expr => $style:expr) => {
                assert_eq!(parse($s).unwrap(), $style);
            };
        }

        test!("" => Style::new());
        test!("  " => Style::new());
        test!("normal" => Style::new());
        test!("normal normal" => Style::new());
        test!("-1 normal" => Style::new());
        test!("red" => Red.on_default());
        test!("red blue" => Red.on(Blue));
        test!("   red blue   " => Red.on(Blue));
        test!("red\tblue" => Red.on(Blue));
        test!("red\n blue" => Red.on(Blue));
        test!("red\r\n blue" => Red.on(Blue));
        test!("blue red" => Blue.on(Red));
        test!("yellow green" => Yellow.on(Green));
        test!("white magenta" => White.on(Magenta));
        test!("black cyan" => Black.on(Cyan));
        test!("red normal" => Red.on_default());
        test!("normal red" => Style::new().bg_color(Some(Red.into())));
        test!("0" => Ansi256Color(0).on_default());
        test!("8 3" => Ansi256Color(8).on(Ansi256Color(3)));
        test!("255" => Ansi256Color(255).on_default());
        test!("255 -1" => Ansi256Color(255).on_default());
        test!("#000000" => RgbColor(0,0,0).on_default());
        test!("#204060" => RgbColor(0x20,0x40,0x60).on_default());

        test!("bold cyan white" => Cyan.on(White).bold());
        test!("bold cyan nobold white" => Cyan.on(White));
        test!("bold cyan reverse white nobold" => Cyan.on(White).invert());
        test!("bold cyan ul white dim" => Cyan.on(White).bold().underline().dimmed());
        test!("ul cyan white no-ul" => Cyan.on(White));
        test!("italic cyan white" => Cyan.on(White).italic());
        test!("strike cyan white" => Cyan.on(White).strikethrough());
        test!("blink #050505 white" => RgbColor(5,5,5).on(White).blink());
    }

    #[test]
    fn test_parse_style_err() {
        macro_rules! test {
            ($s:expr => $err:ident $word:expr) => {
                assert_eq!(
                    parse($s),
                    Err($err {
                        style: $s.to_string(),
                        word: $word.to_string()
                    })
                );
            };
        }

        test!("red blue green" => ExtraColor "green");
        test!("red blue 123" => ExtraColor "123");
        test!("123 red blue" => ExtraColor "blue");
        test!("red blue normal" => ExtraColor "normal");
        test!("red blue -1" => ExtraColor "-1");
        test!("yellow green #abcdef" => ExtraColor "#abcdef");
        test!("#123456 #654321 #abcdef" => ExtraColor "#abcdef");
        test!("bold red blue green" => ExtraColor "green");
        test!("red bold blue green" => ExtraColor "green");
        test!("red blue bold green" => ExtraColor "green");
        test!("red blue green bold" => ExtraColor "green");

        test!("256" => UnknownWord "256");
        test!("-2" => UnknownWord "-2");
        test!("-" => UnknownWord "-");
        test!("- 1" => UnknownWord "-");
        test!("123-1" => UnknownWord "123-1");
        test!("blue1" => UnknownWord "blue1");
        test!("blue-1" => UnknownWord "blue-1");
        test!("no" => UnknownWord "no");
        test!("nou" => UnknownWord "nou");
        test!("noblue" => UnknownWord "noblue");
        test!("no#123456" => UnknownWord "no#123456");
        test!("no-" => UnknownWord "no-");
        test!("no-u" => UnknownWord "no-u");
        test!("no-green" => UnknownWord "no-green");
        test!("no-#123456" => UnknownWord "no-#123456");
        test!("#" => UnknownWord "#");
        test!("#12345" => UnknownWord "#12345");
        test!("#1234567" => UnknownWord "#1234567");
        test!("#bcdefg" => UnknownWord "#bcdefg");
        test!("#blue" => UnknownWord "#blue");
        test!("blue#123456" => UnknownWord "blue#123456");
    }

    #[test]
    fn test_extension_trait() {
        let style = anstyle::Style::parse_git("red blue");
        assert_eq!(style.unwrap(), Red.on(Blue));
    }
}
