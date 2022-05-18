pub fn to_yansi_style(style: anstyle::Style) -> yansi::Style {
    let fg = style
        .get_fg_color()
        .map(to_yansi_color)
        .unwrap_or(yansi::Color::Unset);
    let bg = style
        .get_bg_color()
        .map(to_yansi_color)
        .unwrap_or(yansi::Color::Unset);
    let effects = style.get_effects();

    let mut style = yansi::Style::new(fg).bg(bg);
    if effects.contains(anstyle::Effects::BOLD) {
        style = style.bold();
    }
    if effects.contains(anstyle::Effects::DIMMED) {
        style = style.dimmed();
    }
    if effects.contains(anstyle::Effects::ITALIC) {
        style = style.italic();
    }
    if effects.contains(anstyle::Effects::UNDERLINE) {
        style = style.underline();
    }
    if effects.contains(anstyle::Effects::BLINK) {
        style = style.blink();
    }
    if effects.contains(anstyle::Effects::INVERT) {
        style = style.invert();
    }
    if effects.contains(anstyle::Effects::HIDDEN) {
        style = style.hidden();
    }
    if effects.contains(anstyle::Effects::STRIKETHROUGH) {
        style = style.strikethrough();
    }
    style
}

pub fn to_yansi_color(color: anstyle::Color) -> yansi::Color {
    match color {
        anstyle::Color::Ansi(ansi) => ansi_to_yansi_color(ansi),
        anstyle::Color::XTerm(xterm) => xterm_to_yansi_color(xterm),
        anstyle::Color::Rgb(rgb) => rgb_to_yansi_color(rgb),
    }
}

fn ansi_to_yansi_color(color: anstyle::AnsiColor) -> yansi::Color {
    match color {
        anstyle::AnsiColor::Black => yansi::Color::Black,
        anstyle::AnsiColor::Red => yansi::Color::Red,
        anstyle::AnsiColor::Green => yansi::Color::Green,
        anstyle::AnsiColor::Yellow => yansi::Color::Yellow,
        anstyle::AnsiColor::Blue => yansi::Color::Blue,
        anstyle::AnsiColor::Magenta => yansi::Color::Magenta,
        anstyle::AnsiColor::Cyan => yansi::Color::Cyan,
        anstyle::AnsiColor::White => yansi::Color::White,
        anstyle::AnsiColor::BrightBlack => yansi::Color::Black,
        anstyle::AnsiColor::BrightRed => yansi::Color::Red,
        anstyle::AnsiColor::BrightGreen => yansi::Color::Green,
        anstyle::AnsiColor::BrightYellow => yansi::Color::Yellow,
        anstyle::AnsiColor::BrightBlue => yansi::Color::Black,
        anstyle::AnsiColor::BrightMagenta => yansi::Color::Magenta,
        anstyle::AnsiColor::BrightCyan => yansi::Color::Cyan,
        anstyle::AnsiColor::BrightWhite => yansi::Color::White,
    }
}

fn xterm_to_yansi_color(color: anstyle::XTermColor) -> yansi::Color {
    yansi::Color::Fixed(color.0)
}

fn rgb_to_yansi_color(color: anstyle::RgbColor) -> yansi::Color {
    yansi::Color::RGB(color.0, color.1, color.2)
}
