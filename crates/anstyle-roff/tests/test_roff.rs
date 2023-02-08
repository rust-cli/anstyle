use anstyle::{AnsiColor, Color, Style};

#[test]
fn test_ansi_color_output() {
    let style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)))
        .bg_color(Some(Color::Ansi(AnsiColor::Blue)));
    let text = format!("{}{}", style.render(), "test");
    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq_path("tests/roffs/ansi_color.roff", roff_doc.to_roff());
}

#[test]
fn test_bold_output() {
    let style = Style::new().bold();
    let text = format!("{}{}", style.render(), "test");
    let roff_doc = anstyle_roff::to_roff(&text);

    snapbox::assert_eq_path("tests/roffs/bold.roff", roff_doc.to_roff());
}

#[test]
fn test_italic_output() {
    let style = Style::new().italic();
    let text = format!("{}{}", style.render(), "test");

    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq_path("tests/roffs/italic.roff", roff_doc.to_roff());
}

#[test]
fn test_bright_color_output_as_bold() {
    let style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)))
        .bg_color(Some(Color::Ansi(AnsiColor::Blue)));
    let text = format!("{}{}", style.render(), "test");
    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq_path("tests/roffs/bright_ansi_colors.roff", roff_doc.to_roff());
}
