//! Convert from ansi stylings to ROFF Control Lines
//! Currently uses [roff](https://docs.rs/roff/0.2.1/roff/) as the engine for generating
//! roff output.

mod styled_str;
use anstyle::{Color, RgbColor};
use roff::{bold, italic, Roff};
use styled_str::StyledStr;

/// Generate A RoffStyle from Style
///
/// ```rust
/// use anstyle::{Color, RgbColor};
///
/// let text = "\u{1b}[44;31mtest\u{1b}[0m";
///
/// let roff_doc = anstyle_roff::to_roff(text);
/// let expected = r#".gcolor red
/// .fcolor blue
/// test
/// "#;
///
/// assert_eq!(roff_doc.to_roff(), expected);
/// ```
pub fn to_roff(styled_text: &str) -> Roff {
    let mut doc = Roff::new();
    for styled in styled_str::styled_stream(styled_text) {
        set_color((&styled.style.get_fg_color(), &styled.style.get_bg_color()), &mut doc);
        set_effects_and_text(&styled, &mut doc);
    }
    doc
}


fn set_effects_and_text(styled: &StyledStr, doc: &mut Roff) {
    // Roff (the crate) only supports these inline commands
    //  - Bold
    //  - Italic
    //  - Roman (plain text)
    // If we want more support, or even support combined formats, we will need
    // to push improvements to roff upstream or implement a more thorough roff crate
    // perhaps by spinning off some of this code
    let effects = styled.style.get_effects();
    if effects.contains(anstyle::Effects::BOLD) {
        doc.text(vec![bold(styled.text)]);
    }

    if effects.contains(anstyle::Effects::ITALIC) {
        doc.text(vec![italic(styled.text)]);
    }

    if effects.is_plain() {
        doc.text(vec![roff::roman(styled.text)]);
    }
}

type ColorSet<'a> = (&'a Option<Color>, &'a Option<Color>);

/// Set the foreground, background color
fn set_color(colors: ColorSet, doc: &mut Roff) {
    add_color_to_roff(doc, control_requests::FOREGROUND, colors.0);
    add_color_to_roff(doc, control_requests::BACKGROUND, colors.1);
}

fn add_color_to_roff(doc: &mut Roff, control_request: &str, color: &Option<Color>) {
    match color {
        Some(Color::Rgb(c)) => {
            let name = rgb_name(c);
            doc.control(
                control_requests::CREATE_COLOR,
                vec![name.as_str(), "rgb", as_hex(c).as_str()],
            )
            .control(control_request, vec![name.as_str()]);
        }

        Some(Color::Ansi(c)) => {
            doc.control(control_request, vec![ansi_color_to_roff(c)]);
        }
        _ => {
            // TODO: get rid of "default" hardcoded str?
            doc.control(control_request, vec!["default"]);
        }
    }
}

fn rgb_name(c: &RgbColor) -> String {
    format!("hex_{}", as_hex(c).as_str())
}

fn as_hex(rgb: &RgbColor) -> String {
    let val: usize = ((rgb.0 as usize) << 16) + ((rgb.1 as usize) << 8) + (rgb.2 as usize);
    format!("#{:06x}", val)
}

fn ansi_color_to_roff(color: &anstyle::AnsiColor) -> &'static str {
    match color {
        anstyle::AnsiColor::Black => "black",
        anstyle::AnsiColor::Red => "red",
        anstyle::AnsiColor::Green => "green",
        anstyle::AnsiColor::Yellow => "yellow",
        anstyle::AnsiColor::Blue => "blue",
        anstyle::AnsiColor::Magenta => "magenta",
        anstyle::AnsiColor::Cyan => "cyan",
        anstyle::AnsiColor::White => "white",
        _ => "default",
    }
}

/// Static Strings defining ROFF Control Requests
mod control_requests {
    /// Control to Create a Color definition
    pub const CREATE_COLOR: &'static str = "defcolor";
    /// Roff control request to set background color (fill color)
    pub const BACKGROUND: &'static str = "fcolor";
    /// Roff control request to set foreground color (glyph color)
    pub const FOREGROUND: &'static str = "gcolor";
}

/// Default AsciiColors supported by roff
#[cfg(test)]
mod tests {
    use super::*;
    use anstyle::RgbColor;

    #[test]
    fn to_hex() {
        assert_eq!(as_hex(&RgbColor(0, 0, 0)).as_str(), "#000000");
        assert_eq!(as_hex(&RgbColor(255, 0, 0)).as_str(), "#ff0000");
        assert_eq!(as_hex(&RgbColor(0, 255, 0)).as_str(), "#00ff00");
        assert_eq!(as_hex(&RgbColor(0, 0, 255)).as_str(), "#0000ff");
    }
}
