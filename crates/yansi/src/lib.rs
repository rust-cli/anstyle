pub fn to_yansi_style(style: anstyle::Style) -> yansi::Style {
    let (fg, fg_bold) = style
        .get_fg_color()
        .map(to_yansi_color_with_bold)
        .unwrap_or((yansi::Color::Unset, false));
    let bg = style
        .get_bg_color()
        .map(to_yansi_color)
        .unwrap_or(yansi::Color::Unset);
    let effects = style.get_effects();

    let mut style = yansi::Style::new(fg).bg(bg);
    if effects.contains(anstyle::Effects::BOLD) || fg_bold {
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
    to_yansi_color_with_bold(color).0
}

fn to_yansi_color_with_bold(color: anstyle::Color) -> (yansi::Color, bool) {
    match color {
        anstyle::Color::Ansi(ansi) => ansi_to_yansi_color(ansi),
        anstyle::Color::XTerm(xterm) => (xterm_to_yansi_color(xterm), false),
        anstyle::Color::Rgb(rgb) => (rgb_to_yansi_color(rgb), false),
    }
}

fn ansi_to_yansi_color(color: anstyle::AnsiColor) -> (yansi::Color, bool) {
    match color {
        anstyle::AnsiColor::Black => (yansi::Color::Black, false),
        anstyle::AnsiColor::Red => (yansi::Color::Red, false),
        anstyle::AnsiColor::Green => (yansi::Color::Green, false),
        anstyle::AnsiColor::Yellow => (yansi::Color::Yellow, false),
        anstyle::AnsiColor::Blue => (yansi::Color::Blue, false),
        anstyle::AnsiColor::Magenta => (yansi::Color::Magenta, false),
        anstyle::AnsiColor::Cyan => (yansi::Color::Cyan, false),
        anstyle::AnsiColor::White => (yansi::Color::White, false),
        anstyle::AnsiColor::BrightBlack => (yansi::Color::Black, true),
        anstyle::AnsiColor::BrightRed => (yansi::Color::Red, true),
        anstyle::AnsiColor::BrightGreen => (yansi::Color::Green, true),
        anstyle::AnsiColor::BrightYellow => (yansi::Color::Yellow, true),
        anstyle::AnsiColor::BrightBlue => (yansi::Color::Black, true),
        anstyle::AnsiColor::BrightMagenta => (yansi::Color::Magenta, true),
        anstyle::AnsiColor::BrightCyan => (yansi::Color::Cyan, true),
        anstyle::AnsiColor::BrightWhite => (yansi::Color::White, true),
    }
}

fn xterm_to_yansi_color(color: anstyle::XTermColor) -> yansi::Color {
    yansi::Color::Fixed(color.0)
}

fn rgb_to_yansi_color(color: anstyle::RgbColor) -> yansi::Color {
    yansi::Color::RGB(color.0, color.1, color.2)
}
