use std::fmt::{Result, Write};

use anstyle::{Ansi256Color, AnsiColor};

#[test]
fn no_leading_zero() -> Result {
    let mut actual = String::new();
    let ansi_colors = vec![
        AnsiColor::Black,
        AnsiColor::Red,
        AnsiColor::Green,
        AnsiColor::Yellow,
        AnsiColor::Blue,
        AnsiColor::Magenta,
        AnsiColor::Cyan,
        AnsiColor::White,
        AnsiColor::BrightBlack,
        AnsiColor::BrightRed,
        AnsiColor::BrightGreen,
        AnsiColor::BrightYellow,
        AnsiColor::BrightBlue,
        AnsiColor::BrightMagenta,
        AnsiColor::BrightCyan,
        AnsiColor::BrightWhite,
    ];

    for c in ansi_colors {
        let c = Ansi256Color::from_ansi(c).on_default();
        writeln!(actual, "{c}{c:?}{c:#}")?;
    }

    snapbox::assert_data_eq!(actual, snapbox::file!["no_leading_zero.vte": Text].raw());

    Ok(())
}
