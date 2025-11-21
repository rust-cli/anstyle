//! Convert between [ratatui] and [generic styling types][anstyle]
//!
//! [ratatui]: https://docs.rs/ratatui
//! [anstyle]: https://docs.rs/anstyle

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

/// Adapt generic styling to [`ratatui`]
pub fn to_ratatui_style(style: anstyle::Style) -> ratatui::style::Style {
    let mut ratatui_style = ratatui::style::Style::default();

    if let Some(fg) = style.get_fg_color() {
        ratatui_style = ratatui_style.fg(to_ratatui_color(fg));
    }
    if let Some(bg) = style.get_bg_color() {
        ratatui_style = ratatui_style.bg(to_ratatui_color(bg));
    }
    let modifiers = to_ratatui_modifier(style.get_effects());
    if modifiers != ratatui::style::Modifier::empty() {
        ratatui_style = ratatui_style.add_modifier(modifiers);
    }

    ratatui_style
}

/// Adapt [`ratatui`] styling to generic [`anstyle::Style`]
pub fn from_ratatui_style(style: ratatui::style::Style) -> anstyle::Style {
    let mut anstyle = anstyle::Style::new();

    if let Some(fg) = style.fg.and_then(from_ratatui_color) {
        anstyle = anstyle.fg_color(Some(fg));
    }
    if let Some(bg) = style.bg.and_then(from_ratatui_color) {
        anstyle = anstyle.bg_color(Some(bg));
    }
    let effects = from_ratatui_modifier(style.add_modifier);
    if !effects.is_plain() {
        anstyle = anstyle.effects(effects);
    }

    anstyle
}

/// Adapt generic colors to [`ratatui`]
pub fn to_ratatui_color(color: anstyle::Color) -> ratatui::style::Color {
    match color {
        anstyle::Color::Ansi(ansi) => ansi_to_ratatui_color(ansi),
        anstyle::Color::Ansi256(xterm) => xterm_to_ratatui_color(xterm),
        anstyle::Color::Rgb(rgb) => rgb_to_ratatui_color(rgb),
    }
}

fn ansi_to_ratatui_color(color: anstyle::AnsiColor) -> ratatui::style::Color {
    match color {
        anstyle::AnsiColor::Black => ratatui::style::Color::Black,
        anstyle::AnsiColor::Red => ratatui::style::Color::Red,
        anstyle::AnsiColor::Green => ratatui::style::Color::Green,
        anstyle::AnsiColor::Yellow => ratatui::style::Color::Yellow,
        anstyle::AnsiColor::Blue => ratatui::style::Color::Blue,
        anstyle::AnsiColor::Magenta => ratatui::style::Color::Magenta,
        anstyle::AnsiColor::Cyan => ratatui::style::Color::Cyan,
        anstyle::AnsiColor::White => ratatui::style::Color::Gray,
        anstyle::AnsiColor::BrightBlack => ratatui::style::Color::DarkGray,
        anstyle::AnsiColor::BrightRed => ratatui::style::Color::LightRed,
        anstyle::AnsiColor::BrightGreen => ratatui::style::Color::LightGreen,
        anstyle::AnsiColor::BrightYellow => ratatui::style::Color::LightYellow,
        anstyle::AnsiColor::BrightBlue => ratatui::style::Color::LightBlue,
        anstyle::AnsiColor::BrightMagenta => ratatui::style::Color::LightMagenta,
        anstyle::AnsiColor::BrightCyan => ratatui::style::Color::LightCyan,
        anstyle::AnsiColor::BrightWhite => ratatui::style::Color::White,
    }
}

fn xterm_to_ratatui_color(color: anstyle::Ansi256Color) -> ratatui::style::Color {
    ratatui::style::Color::Indexed(color.0)
}

fn rgb_to_ratatui_color(color: anstyle::RgbColor) -> ratatui::style::Color {
    ratatui::style::Color::Rgb(color.0, color.1, color.2)
}

/// Adapt [`ratatui`] colors to generic [`anstyle::Color`]
pub fn from_ratatui_color(color: ratatui::style::Color) -> Option<anstyle::Color> {
    match color {
        ratatui::style::Color::Reset => None,
        ratatui::style::Color::Black => Some(anstyle::AnsiColor::Black.into()),
        ratatui::style::Color::Red => Some(anstyle::AnsiColor::Red.into()),
        ratatui::style::Color::Green => Some(anstyle::AnsiColor::Green.into()),
        ratatui::style::Color::Yellow => Some(anstyle::AnsiColor::Yellow.into()),
        ratatui::style::Color::Blue => Some(anstyle::AnsiColor::Blue.into()),
        ratatui::style::Color::Magenta => Some(anstyle::AnsiColor::Magenta.into()),
        ratatui::style::Color::Cyan => Some(anstyle::AnsiColor::Cyan.into()),
        ratatui::style::Color::Gray => Some(anstyle::AnsiColor::White.into()),
        ratatui::style::Color::DarkGray => Some(anstyle::AnsiColor::BrightBlack.into()),
        ratatui::style::Color::LightRed => Some(anstyle::AnsiColor::BrightRed.into()),
        ratatui::style::Color::LightGreen => Some(anstyle::AnsiColor::BrightGreen.into()),
        ratatui::style::Color::LightYellow => Some(anstyle::AnsiColor::BrightYellow.into()),
        ratatui::style::Color::LightBlue => Some(anstyle::AnsiColor::BrightBlue.into()),
        ratatui::style::Color::LightMagenta => Some(anstyle::AnsiColor::BrightMagenta.into()),
        ratatui::style::Color::LightCyan => Some(anstyle::AnsiColor::BrightCyan.into()),
        ratatui::style::Color::White => Some(anstyle::AnsiColor::BrightWhite.into()),
        ratatui::style::Color::Indexed(value) => Some(anstyle::Ansi256Color(value).into()),
        ratatui::style::Color::Rgb(r, g, b) => Some(anstyle::RgbColor(r, g, b).into()),
    }
}

/// Adapt generic effects to [`ratatui`]
pub fn to_ratatui_modifier(effects: anstyle::Effects) -> ratatui::style::Modifier {
    use ratatui::style::Modifier;

    let mut modifier = Modifier::empty();
    if effects.contains(anstyle::Effects::BOLD) {
        modifier |= Modifier::BOLD;
    }
    if effects.contains(anstyle::Effects::DIMMED) {
        modifier |= Modifier::DIM;
    }
    if effects.contains(anstyle::Effects::ITALIC) {
        modifier |= Modifier::ITALIC;
    }
    if has_underline(effects) {
        modifier |= Modifier::UNDERLINED;
    }
    if effects.contains(anstyle::Effects::BLINK) {
        modifier |= Modifier::SLOW_BLINK;
    }
    if effects.contains(anstyle::Effects::INVERT) {
        modifier |= Modifier::REVERSED;
    }
    if effects.contains(anstyle::Effects::HIDDEN) {
        modifier |= Modifier::HIDDEN;
    }
    if effects.contains(anstyle::Effects::STRIKETHROUGH) {
        modifier |= Modifier::CROSSED_OUT;
    }
    modifier
}

/// Adapt [`ratatui`] modifiers to [`anstyle::Effects`]
pub fn from_ratatui_modifier(modifier: ratatui::style::Modifier) -> anstyle::Effects {
    use ratatui::style::Modifier;

    let mut effects = anstyle::Effects::new();
    if modifier.contains(Modifier::BOLD) {
        effects = effects.insert(anstyle::Effects::BOLD);
    }
    if modifier.contains(Modifier::DIM) {
        effects = effects.insert(anstyle::Effects::DIMMED);
    }
    if modifier.contains(Modifier::ITALIC) {
        effects = effects.insert(anstyle::Effects::ITALIC);
    }
    if modifier.contains(Modifier::UNDERLINED) {
        effects = effects.insert(anstyle::Effects::UNDERLINE);
    }
    if modifier.intersects(Modifier::SLOW_BLINK | Modifier::RAPID_BLINK) {
        effects = effects.insert(anstyle::Effects::BLINK);
    }
    if modifier.contains(Modifier::REVERSED) {
        effects = effects.insert(anstyle::Effects::INVERT);
    }
    if modifier.contains(Modifier::HIDDEN) {
        effects = effects.insert(anstyle::Effects::HIDDEN);
    }
    if modifier.contains(Modifier::CROSSED_OUT) {
        effects = effects.insert(anstyle::Effects::STRIKETHROUGH);
    }
    effects
}

fn has_underline(effects: anstyle::Effects) -> bool {
    effects.contains(anstyle::Effects::UNDERLINE)
        || effects.contains(anstyle::Effects::DOUBLE_UNDERLINE)
        || effects.contains(anstyle::Effects::CURLY_UNDERLINE)
        || effects.contains(anstyle::Effects::DOTTED_UNDERLINE)
        || effects.contains(anstyle::Effects::DASHED_UNDERLINE)
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
