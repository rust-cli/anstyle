use colored::Colorize;

#[test]
fn test_ansi_color_output() {
    let text = "test".red().on_blue().to_string();
    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq_path("tests/roffs/ansi_color.roff", roff_doc.to_roff());
}

#[test]
fn test_bold_output() {
    let text = "test".bold().to_string();
    let roff_doc = anstyle_roff::to_roff(&text);

    snapbox::assert_eq_path("tests/roffs/bold.roff", roff_doc.to_roff());
}

#[test]
fn test_italic_output() {
    let text = "test".italic().to_string();

    let roff_doc = anstyle_roff::to_roff(&text);
    snapbox::assert_eq_path("tests/roffs/italic.roff", roff_doc.to_roff());
}
