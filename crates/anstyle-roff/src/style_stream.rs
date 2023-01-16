use cansi::{v3::CategorisedSlice, Color, Intensity};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone)]
pub(crate) struct StyledStream<'text> {
    inner: Vec<StyledStr<'text>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct StyledStr<'text> {
    text: &'text str,
    style: anstyle::Style,
}

impl<'text> From<CategorisedSlice<'text>> for StyledStr<'text> {
    fn from(category: cansi::v3::CategorisedSlice<'text>) -> Self {
        let mut style = anstyle::Style::new();
        style = style
            .fg_color(cansi_to_anstyle_color(category.fg))
            .bg_color(cansi_to_anstyle_color(category.bg));

        if let Some(true) = category.underline {
            style = style.underline();
        }

        let effects = create_effects(&category);
        style = style.effects(effects);

        Self {
            text: category.text,
            style,
        }
    }
}

fn create_effects(category: &CategorisedSlice) -> anstyle::Effects {
    anstyle::Effects::new()
        .set(anstyle::Effects::ITALIC, category.italic.unwrap_or(false))
        .set(anstyle::Effects::BLINK, category.blink.unwrap_or(false))
        .set(anstyle::Effects::INVERT, category.reversed.unwrap_or(false))
        .set(anstyle::Effects::HIDDEN, category.hidden.unwrap_or(false))
        .set(
            anstyle::Effects::STRIKETHROUGH,
            category.strikethrough.unwrap_or(false),
        )
        .set(anstyle::Effects::BOLD, is_bold(category.intensity))
        .set(anstyle::Effects::DIMMED, is_faint(category.intensity))
}

fn is_bold(intensity: Option<Intensity>) -> bool {
    matches!(intensity, Some(Intensity::Bold))
}

fn is_faint(intensity: Option<Intensity>) -> bool {
    matches!(intensity, Some(Intensity::Faint))
}

impl StyledStr<'_> {
    pub(crate) fn text(&self) -> &str {
        self.text
    }

    pub(crate) fn style(&self) -> &anstyle::Style {
        &self.style
    }
}

impl<'a> IntoIterator for StyledStream<'a> {
    type Item = StyledStr<'a>;
    type IntoIter = <Vec<StyledStr<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> Deref for StyledStream<'a> {
    type Target = [StyledStr<'a>];

    fn deref(&self) -> &Self::Target {
        &self.inner[..]
    }
}

impl<'a> DerefMut for StyledStream<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner[..]
    }
}

fn cansi_to_anstyle_color(color: Option<Color>) -> Option<anstyle::Color> {
    match color {
        Some(Color::Black) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Black)),
        Some(Color::Red) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)),
        Some(Color::Green) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)),
        Some(Color::Yellow) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)),
        Some(Color::Blue) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue)),
        Some(Color::Magenta) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Magenta)),
        Some(Color::Cyan) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)),
        Some(Color::White) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::White)),
        Some(Color::BrightBlack) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack)),
        Some(Color::BrightRed) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed)),
        Some(Color::BrightGreen) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen)),
        Some(Color::BrightYellow) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow)),
        Some(Color::BrightBlue) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue)),
        Some(Color::BrightMagenta) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta)),
        Some(Color::BrightCyan) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan)),
        Some(Color::BrightWhite) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite)),
        None => None,
    }
}

impl<'text> StyledStream<'text> {
    pub(crate) fn new(s: &'text str) -> Self {
        let categorized = cansi::v3::categorise_text(s);
        Self {
            inner: categorized.into_iter().map(|x| dbg!(x.into())).collect(),
        }
    }
}
