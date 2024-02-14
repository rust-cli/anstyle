use anstyle::Style;
use snapbox::file;

#[test]
fn test_ansi_color_output() {
    // HACK: cansi doesn't properly merge separate sequences
    // let style = AnsiColor::Red.on(AnsiColor::Blue);
    // let text = format!("{}{}", style.render(), "test");
    let text = "\u{1b}[31;44mtest".to_owned();
    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq(file!["roffs/ansi_color.roff"], roff_doc.to_roff());
}

#[test]
fn test_bold_output() {
    let style = Style::new().bold();
    let text = format!("{}{}", style.render(), "test");
    dbg!(&text);
    let roff_doc = anstyle_roff::to_roff(&text);

    snapbox::assert_eq(file!["roffs/bold.roff"], roff_doc.to_roff());
}

#[test]
fn test_italic_output() {
    let style = Style::new().italic();
    let text = format!("{}{}", style.render(), "test");
    dbg!(&text);

    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq(file!["roffs/italic.roff"], roff_doc.to_roff());
}

#[test]
fn test_bright_color_output_as_bold() {
    // HACK: cansi doesn't properly merge separate sequences
    // let style = AnsiColor::BrightRed.on(AnsiColor::Blue);
    // let text = format!("{}{}", style.render(), "test");
    let text = "\u{1b}[91;44mtest".to_owned();
    dbg!(&text);
    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq(file!["roffs/bright_ansi_colors.roff"], roff_doc.to_roff());
}
