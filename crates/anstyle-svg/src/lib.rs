pub use anstyle_lossy::palette::Palette;
pub use anstyle_lossy::palette::VGA;
pub use anstyle_lossy::palette::WIN10_CONSOLE;

#[derive(Copy, Clone, Debug)]
pub struct Term {
    palette: Palette,
    fg_color: anstyle::Color,
    bg_color: anstyle::Color,
    background: bool,
    font_family: &'static str,
}

impl Term {
    pub const fn new() -> Self {
        Self {
            palette: VGA,
            fg_color: FG_COLOR,
            bg_color: BG_COLOR,
            background: true,
            font_family: "SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace",
        }
    }

    pub const fn palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }

    pub const fn fg_color(mut self, color: anstyle::Color) -> Self {
        self.fg_color = color;
        self
    }

    pub const fn bg_color(mut self, color: anstyle::Color) -> Self {
        self.bg_color = color;
        self
    }

    pub const fn background(mut self, yes: bool) -> Self {
        self.background = yes;
        self
    }

    pub const fn font_family(mut self, font: &'static str) -> Self {
        self.font_family = font;
        self
    }

    pub fn render_svg(&self, ansi: &str) -> String {
        use unicode_width::UnicodeWidthStr as _;

        let mut styled = anstream::adapter::WinconBytes::new();
        let mut styled = styled.extract_next(ansi.as_bytes()).collect::<Vec<_>>();
        // Pre-process INVERT to make fg/bg calculations easier
        for (style, _) in &mut styled {
            if style.get_effects().contains(anstyle::Effects::INVERT) {
                *style = style
                    .fg_color(Some(style.get_bg_color().unwrap_or(self.bg_color)))
                    .bg_color(Some(style.get_fg_color().unwrap_or(self.fg_color)))
                    .effects(style.get_effects().remove(anstyle::Effects::INVERT));
            }
        }

        const FG: &str = "fg";
        let fg_color = rgb_value(self.fg_color, self.palette);
        const BG: &str = "bg";
        let bg_color = rgb_value(self.bg_color, self.palette);
        let font_family = self.font_family;

        let line_height = 18;
        let height = (ansi.lines().count() + 1) * line_height;
        let stripped = anstream::adapter::strip_str(ansi).to_string();
        let max_width = stripped.lines().map(|s| s.width()).max().unwrap_or(20);

        use std::fmt::Write as _;
        let mut buffer = String::new();
        writeln!(
            &mut buffer,
            r#"<svg width="{max_width}em" height="{height}px" xmlns="http://www.w3.org/2000/svg">"#
        )
        .unwrap();
        writeln!(&mut buffer, r#"  <style>"#).unwrap();
        writeln!(&mut buffer, r#"    .{FG} {{ fill: {fg_color} }}"#).unwrap();
        writeln!(&mut buffer, r#"    .{BG} {{ background: {bg_color} }}"#).unwrap();
        for (name, rgb) in color_styles(&styled, self.palette) {
            if name.starts_with(FG_PREFIX) {
                writeln!(&mut buffer, r#"    .{name} {{ fill: {rgb} }}"#).unwrap();
            }
            if name.starts_with(BG_PREFIX) {
                writeln!(&mut buffer, r#"    .{name} {{ background: {rgb} }}"#).unwrap();
            }
            if name.starts_with(UNDERLINE_PREFIX) {
                writeln!(
                    &mut buffer,
                    r#"    .{name} {{ text-decoration-line: underline; text-decoration-color: {rgb} }}"#
                )
                .unwrap();
            }
        }
        writeln!(&mut buffer, r#"    .container {{"#).unwrap();
        writeln!(&mut buffer, r#"      padding: 0 10px;"#).unwrap();
        writeln!(&mut buffer, r#"      line-height: {line_height}px;"#).unwrap();
        writeln!(&mut buffer, r#"    }}"#).unwrap();
        writeln!(&mut buffer, r#"    .bold {{ font-weight: bold; }}"#).unwrap();
        writeln!(&mut buffer, r#"    .italic {{ font-style: italic; }}"#).unwrap();
        writeln!(
            &mut buffer,
            r#"    .underline {{ text-decoration-line: underline; }}"#
        )
        .unwrap();
        writeln!(
            &mut buffer,
            r#"    .double-underline {{ text-decoration-line: underline; text-decoration-style: double; }}"#
        )
        .unwrap();
        writeln!(
            &mut buffer,
            r#"    .curly-underline {{ text-decoration-line: underline; text-decoration-style: wavy; }}"#
        )
        .unwrap();
        writeln!(
            &mut buffer,
            r#"    .dotted-underline {{ text-decoration-line: underline; text-decoration-style: dotted; }}"#
        )
        .unwrap();
        writeln!(
            &mut buffer,
            r#"    .dashed-underline {{ text-decoration-line: underline; text-decoration-style: dashed; }}"#
        )
        .unwrap();
        writeln!(
            &mut buffer,
            r#"    .strikethrough {{ text-decoration-line: line-through; }}"#
        )
        .unwrap();
        writeln!(&mut buffer, r#"    .dimmed {{ opacity: 0.7; }}"#).unwrap();
        writeln!(&mut buffer, r#"    tspan {{"#).unwrap();
        writeln!(&mut buffer, r#"      font: 14px {font_family};"#).unwrap();
        writeln!(&mut buffer, r#"      white-space: pre;"#).unwrap();
        writeln!(&mut buffer, r#"      line-height: {line_height}px;"#).unwrap();
        writeln!(&mut buffer, r#"    }}"#).unwrap();
        writeln!(&mut buffer, r#"  </style>"#).unwrap();
        if self.background {
            writeln!(
                &mut buffer,
                r#"  <rect width="100%" height="100%" y="0" rx="4.5" class="{BG}" />"#
            )
            .unwrap();
        }
        let mut text_y = line_height;
        write!(&mut buffer, r#"  <text class="container {FG} {BG}">"#).unwrap();
        write!(&mut buffer, r#"    <tspan x="0px" y="{text_y}px">"#).unwrap();
        for (style, string) in &styled {
            if string.is_empty() {
                continue;
            }
            let mut remaining = string.as_str();
            while let Some((fragment, remains)) = remaining.split_once('\n') {
                write_span(&mut buffer, style, fragment);
                text_y += line_height;
                // HACK: must close tspan on newline to include them in copy/paste
                writeln!(&mut buffer).unwrap();
                writeln!(&mut buffer, r#"</tspan>"#).unwrap();
                write!(&mut buffer, r#"    <tspan x="0px" y="{text_y}px">"#).unwrap();
                remaining = remains;
            }
            write_span(&mut buffer, style, remaining)
        }
        writeln!(&mut buffer, r#"    </tspan>"#).unwrap();
        writeln!(&mut buffer, r#"  </text>"#).unwrap();
        writeln!(&mut buffer, r#"</svg>"#).unwrap();
        buffer
    }
}

const FG_COLOR: anstyle::Color = anstyle::Color::Ansi(anstyle::AnsiColor::White);
const BG_COLOR: anstyle::Color = anstyle::Color::Ansi(anstyle::AnsiColor::Black);

fn write_span(buffer: &mut String, style: &anstyle::Style, fragment: &str) {
    let fg_color = style.get_fg_color().map(|c| color_name(FG_PREFIX, c));
    let bg_color = style.get_bg_color().map(|c| color_name(BG_PREFIX, c));
    let underline_color = style
        .get_underline_color()
        .map(|c| color_name(UNDERLINE_PREFIX, c));
    let effects = style.get_effects();
    let underline = effects.contains(anstyle::Effects::UNDERLINE);
    let double_underline = effects.contains(anstyle::Effects::DOUBLE_UNDERLINE);
    let curly_underline = effects.contains(anstyle::Effects::CURLY_UNDERLINE);
    let dotted_underline = effects.contains(anstyle::Effects::DOTTED_UNDERLINE);
    let dashed_underline = effects.contains(anstyle::Effects::DASHED_UNDERLINE);
    let strikethrough = effects.contains(anstyle::Effects::STRIKETHROUGH);
    // skipping INVERT as that was handled earlier
    let bold = effects.contains(anstyle::Effects::BOLD);
    let italic = effects.contains(anstyle::Effects::ITALIC);
    let dimmed = effects.contains(anstyle::Effects::DIMMED);

    let fragment = html_escape::encode_text(fragment);
    let mut classes = Vec::new();
    if let Some(class) = fg_color.as_deref() {
        classes.push(class);
    }
    if let Some(class) = bg_color.as_deref() {
        classes.push(class);
    }
    if let Some(class) = underline_color.as_deref() {
        classes.push(class);
    }
    if underline {
        classes.push("underline");
    }
    if double_underline {
        classes.push("double-underline");
    }
    if curly_underline {
        classes.push("curly-underline");
    }
    if dotted_underline {
        classes.push("dotted-underline");
    }
    if dashed_underline {
        classes.push("dashed-underline");
    }
    if strikethrough {
        classes.push("strikethrough");
    }
    if bold {
        classes.push("bold");
    }
    if italic {
        classes.push("italic");
    }
    if dimmed {
        classes.push("dimmed");
    }

    use std::fmt::Write as _;
    write!(buffer, r#"<tspan xml:space="preserve""#).unwrap();
    if !classes.is_empty() {
        let classes = classes.join(" ");
        write!(buffer, r#" class="{classes}""#).unwrap();
    }
    write!(buffer, r#">"#).unwrap();
    write!(buffer, "{fragment}").unwrap();
    write!(buffer, r#"</tspan>"#).unwrap();
}

impl Default for Term {
    fn default() -> Self {
        Self::new()
    }
}

const ANSI_NAMES: [&str; 16] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright-black",
    "bright-red",
    "bright-green",
    "bright-yellow",
    "bright-blue",
    "bright-magenta",
    "bright-cyan",
    "bright-white",
];

fn rgb_value(color: anstyle::Color, palette: anstyle_lossy::palette::Palette) -> String {
    let color = anstyle_lossy::color_to_rgb(color, palette);
    let anstyle::RgbColor(r, g, b) = color;
    format!("#{r:02X}{g:02X}{b:02X}")
}

const FG_PREFIX: &str = "fg";
const BG_PREFIX: &str = "bg";
const UNDERLINE_PREFIX: &str = "underline";

fn color_name(prefix: &str, color: anstyle::Color) -> String {
    match color {
        anstyle::Color::Ansi(color) => {
            let color = anstyle::Ansi256Color::from_ansi(color);
            let index = color.index() as usize;
            let name = ANSI_NAMES[index];
            format!("{prefix}-{name}")
        }
        anstyle::Color::Ansi256(color) => {
            let index = color.index();
            format!("{prefix}-ansi256-{index:03}")
        }
        anstyle::Color::Rgb(color) => {
            let anstyle::RgbColor(r, g, b) = color;
            format!("{prefix}-rgb-{r:02X}{g:02X}{b:02X}")
        }
    }
}

fn color_styles(
    styled: &[(anstyle::Style, String)],
    palette: anstyle_lossy::palette::Palette,
) -> impl Iterator<Item = (String, String)> {
    let mut colors = std::collections::BTreeMap::new();
    for (style, _) in styled {
        if let Some(color) = style.get_fg_color() {
            colors.insert(color_name(FG_PREFIX, color), rgb_value(color, palette));
        }
        if let Some(color) = style.get_bg_color() {
            colors.insert(color_name(BG_PREFIX, color), rgb_value(color, palette));
        }
        if let Some(color) = style.get_underline_color() {
            colors.insert(
                color_name(UNDERLINE_PREFIX, color),
                rgb_value(color, palette),
            );
        }
    }

    colors.into_iter()
}
