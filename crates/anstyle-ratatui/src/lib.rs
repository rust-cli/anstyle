//! Convert between [`ratatui`](https://lib.rs/ratatui) and [generic styling types][anstyle]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

/// Adapt generic styling to [`ratatui`]
pub fn to_ratatui(astyle: anstyle::Style) -> ratatui::style::Style {
    let foreground_color = astyle.get_fg_color().map(to_ansi_color);
    let background_color = astyle.get_bg_color().map(to_ansi_color);
    let underline_color = astyle.get_underline_color().map(to_ansi_color);

    let mut modifiers = ratatui::style::Modifier::default();

    let effects = astyle.get_effects();
    if effects.contains(anstyle::Effects::BOLD) {
        modifiers.set(ratatui::style::Modifier::BOLD, true);
    }
    if effects.contains(anstyle::Effects::DIMMED) {
        modifiers.set(ratatui::style::Modifier::DIM, true);
    }
    if effects.contains(anstyle::Effects::ITALIC) {
        modifiers.set(ratatui::style::Modifier::ITALIC, true);
    }
    if effects.contains(anstyle::Effects::UNDERLINE) {
        modifiers.set(ratatui::style::Modifier::UNDERLINED, true);
    }
    if effects.contains(anstyle::Effects::BLINK) {
        modifiers.set(ratatui::style::Modifier::SLOW_BLINK, true);
    }
    if effects.contains(anstyle::Effects::INVERT) {
        modifiers.set(ratatui::style::Modifier::REVERSED, true);
    }
    if effects.contains(anstyle::Effects::HIDDEN) {
        modifiers.set(ratatui::style::Modifier::HIDDEN, true);
    }
    if effects.contains(anstyle::Effects::STRIKETHROUGH) {
        modifiers.set(ratatui::style::Modifier::CROSSED_OUT, true);
    }

    ratatui::style::Style {
        fg: foreground_color,
        bg: background_color,
        #[cfg(feature = "underline-color")]
        underline_color,
        add_modifier: modifiers,
        sub_modifier: ratatui::style::Modifier::default(),
    }
}

fn to_ansi_color(color: anstyle::Color) -> ratatui::style::Color {
    match color {
        anstyle::Color::Ansi(ansi) => ansi_to_ansi_color(ansi),
        anstyle::Color::Ansi256(xterm) => xterm_to_ansi_color(xterm),
        anstyle::Color::Rgb(rgb) => rgb_to_ansi_color(rgb),
    }
}

fn ansi_to_ansi_color(color: anstyle::AnsiColor) -> ratatui::style::Color {
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

fn xterm_to_ansi_color(color: anstyle::Ansi256Color) -> ratatui::style::Color {
    ratatui::style::Color::Indexed(color.index())
}

fn rgb_to_ansi_color(color: anstyle::RgbColor) -> ratatui::style::Color {
    let anstyle::RgbColor(r, g, b) = color;
    ratatui::style::Color::Rgb(r, g, b)
}
