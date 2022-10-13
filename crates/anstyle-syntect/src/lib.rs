mod sealed {
    pub(crate) trait Sealed {}
}

trait Ext: sealed::Sealed {
    fn syntect_to_anstyle(self) -> anstyle::Style;
}

impl sealed::Sealed for syntect::highlighting::Style {}

impl Ext for syntect::highlighting::Style {
    fn syntect_to_anstyle(self) -> anstyle::Style {
        to_anstyle(self)
    }
}

pub fn to_anstyle(style: syntect::highlighting::Style) -> anstyle::Style {
    anstyle::Style::new()
        .fg_color(Some(to_anstyle_color(style.foreground)))
        .bg_color(Some(to_anstyle_color(style.background)))
        .effects(to_anstyle_effects(style.font_style))
}

fn to_anstyle_color(color: syntect::highlighting::Color) -> anstyle::Color {
    anstyle::RgbColor(color.r, color.g, color.b).into()
}

fn to_anstyle_effects(style: syntect::highlighting::FontStyle) -> anstyle::Effects {
    let mut effects = anstyle::Effects::new();

    if style.contains(syntect::highlighting::FontStyle::BOLD) {
        effects |= anstyle::Effects::BOLD;
    }
    if style.contains(syntect::highlighting::FontStyle::ITALIC) {
        effects |= anstyle::Effects::ITALIC;
    }
    if style.contains(syntect::highlighting::FontStyle::UNDERLINE) {
        effects |= anstyle::Effects::UNDERLINE;
    }

    effects
}
