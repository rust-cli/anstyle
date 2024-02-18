#[test]
fn rainbow() {
    let input = std::fs::read_to_string("tests/rainbow.vte").unwrap();
    let actual = anstyle_svg::Term::new().render_svg(&input);
    snapbox::assert_eq(snapbox::file!["rainbow.svg": Text], actual);
}

#[test]
fn rg_linus() {
    let input = std::fs::read_to_string("tests/rg_linus.vte").unwrap();
    let actual = anstyle_svg::Term::new().render_svg(&input);
    snapbox::assert_eq(snapbox::file!["rg_linus.svg": Text], actual);
}
