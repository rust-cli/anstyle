//! Convert from ansi stylings to ROFF Control Lines
//! Currently uses [roff](https://docs.rs/roff/0.2.1/roff/) as the engine for generating
//! roff output.

mod roff_styles;
use roff::{bold, italic, Roff};

mod sealed {
    pub(crate) trait Sealed {}
}

trait Ext: sealed::Sealed {
    fn to_roff(self) -> RoffStyle;
}

impl sealed::Sealed for anstyle::Style {}

impl Ext for anstyle::Style {
    fn to_roff(self) -> RoffStyle {
        to_roff(self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct RoffStyle {
    fg: Option<roff_styles::Color>,
    bg: Option<roff_styles::Color>,
    effects: anstyle::Effects,
    text: Option<String>,
}

impl RoffStyle {
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            effects: anstyle::Effects::default(),
            text: None,
        }
    }

    /// Add text to be styled
    pub fn text(&mut self, txt: String) -> &mut Self {
        self.text = Some(txt);
        self
    }

    fn fg(&mut self, fg: Option<roff_styles::Color>) -> &mut Self {
        self.fg = fg;
        self
    }
    fn bg(&mut self, bg: Option<roff_styles::Color>) -> &mut Self {
        self.bg = bg;
        self
    }

    fn effects(&mut self, effects: anstyle::Effects) -> &mut Self {
        self.effects = effects;
        self
    }

    /// Render the style, with associated text as Roff Document
    pub fn render(&self) -> String {
        let mut doc = Roff::new();
        doc.extend([set_color((&self.fg, &self.bg)), self.set_effects()]);
        doc.to_roff()
    }

    fn set_effects(&self) -> Roff {
        // Roff (the crate) only supports these inline commands
        // Bold
        // Italic
        // Roman (plain text)
        // If we want more support, or even support combined formats, we will need
        // to push improvements to roff upstream or implement a more thorough roff crate
        // perhaps by spinning off some of this code
        let mut doc = Roff::new();
        if let Some(ref txt) = self.text {
            if self.effects.contains(anstyle::Effects::BOLD) {
                doc.text(vec![bold(txt)]);
                return doc;
            }
            if self.effects.contains(anstyle::Effects::ITALIC) {
                doc.text(vec![italic(txt)]);
                return doc;
            }
            if self.effects.is_plain() {
                doc.text(vec![roff::roman(txt)]);
                return doc;
            }
        }
        doc
    }
}

/// Generate A RoffStyle from Style
///
/// ```rust
/// use anstyle::{Color, RgbColor};
///
/// let style = anstyle::Style::new()
///     .fg_color(Some(Color::Rgb(RgbColor(255, 0, 170))));
///
/// let roff_style = anstyle_roff::to_roff(style);
/// let expected = r#".defcolor hex_#ff00aa rgb #ff00aa
/// .gcolor hex_#ff00aa
/// .fcolor default
/// "#;
/// assert_eq!(roff_style.render(), expected);
/// ```
pub fn to_roff(style: anstyle::Style) -> RoffStyle {
    let fg = ansi_color_to_roff(style.get_fg_color());
    let bg = ansi_color_to_roff(style.get_bg_color());
    let effect = style.get_effects();

    // doc.extend([set_color((fg, bg))]);
    let mut roff_style = RoffStyle::new();
    roff_style.fg(fg).bg(bg).effects(effect);
    roff_style
}

fn ansi_color_to_roff(color: Option<anstyle::Color>) -> Option<roff_styles::Color> {
    match color {
        Some(anstyle::Color::Rgb(rgb)) => Some(roff_styles::Color::Rgb(roff_styles::RgbColor(
            rgb.0, rgb.1, rgb.2,
        ))),
        Some(anstyle::Color::Ansi(color)) => {
            Some(roff_styles::Color::AnsiColor(ansi_color(color).to_string()))
        }
        Some(anstyle::Color::XTerm(color)) => {
            color.into_ansi().map(|c| roff_styles::Color::AnsiColor(ansi_color(c).to_string()))
        }
        _ => None,
    }
}

fn ansi_color(color: anstyle::AnsiColor) -> &'static str {
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

/// Set the foreground, background color
fn set_color(colors: (&Option<roff_styles::Color>, &Option<roff_styles::Color>)) -> Roff {
    let mut doc = Roff::new();
    // Set foreground
    add_color_to_roff(&mut doc, roff_styles::ControlRequests::FOREGROUND, colors.0);
    // Set background
    add_color_to_roff(&mut doc, roff_styles::ControlRequests::BACKGROUND, colors.1);
    doc
}

fn add_color_to_roff(doc: &mut Roff, control_request: &str, color: &Option<roff_styles::Color>) {
    match color {
        Some(roff_styles::Color::Rgb(c)) => {
            let name = format!("hex_{}", c.as_hex().as_str());
            doc.control(
                roff_styles::ControlRequests::CREATE_COLOR,
                vec![name.as_str(), "rgb", c.as_hex().as_str()],
            )
            .control(control_request, vec![name.as_str()]);
        }

        Some(roff_styles::Color::AnsiColor(c)) => {
            doc.control(control_request, vec![c.as_str()]);
        }
        None => {
            doc.control(control_request, vec![roff_styles::RgbColor::DEFAULT]);
        }
    }
}

#[cfg(test)]
mod tests {}
