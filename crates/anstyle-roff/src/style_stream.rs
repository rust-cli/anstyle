use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone)]
pub(crate) struct StyledStream<'text> {
    inner: Vec<StyledStr<'text>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct  StyledStr<'text> {
    text: &'text str,
    style: anstyle::Style,
}

impl<'text> From<cansi::v3::CategorisedSlice<'text>> for StyledStr<'text> {
    fn from(category: cansi::v3::CategorisedSlice<'text>) -> Self {
        let style = anstyle::Style::new();
        style
            .fg_color(cansi_to_anstyle_color(category.fg))
            .bg_color(cansi_to_anstyle_color(category.bg));
        if let Some(true) = category.underline {
            style.underline();
        }

        let effects = anstyle::Effects::new()
            .set(anstyle::Effects::ITALIC, category.italic.unwrap_or(false))
            .set(anstyle::Effects::BLINK, category.blink.unwrap_or(false))
            .set(anstyle::Effects::INVERT, category.reversed.unwrap_or(false))
            .set(anstyle::Effects::HIDDEN, category.hidden.unwrap_or(false))
            .set(
                anstyle::Effects::STRIKETHROUGH,
                category.strikethrough.unwrap_or(false),
            )
            .set(anstyle::Effects::BOLD, is_bold(category.intensity))
            .set(anstyle::Effects::DIMMED, is_faint(category.intensity));

        style.effects(effects);

        Self {
            text: category.text,
            style,
        }
    }
}

fn is_bold(intensity: Option<cansi::Intensity>) -> bool {
    matches!(intensity, Some(cansi::Intensity::Bold))
}

fn is_faint(intensity: Option<cansi::Intensity>) -> bool {
    matches!(intensity, Some(cansi::Intensity::Faint))
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

fn cansi_to_anstyle_color(color: Option<cansi::Color>) -> Option<anstyle::Color> {
    match color {
        Some(cansi::Color::Black) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Black)),
        Some(cansi::Color::Red) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)),
        Some(cansi::Color::Green) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)),
        Some(cansi::Color::Yellow) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)),
        Some(cansi::Color::Blue) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue)),
        Some(cansi::Color::Magenta) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Magenta)),
        Some(cansi::Color::Cyan) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)),
        Some(cansi::Color::White) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::White)),
        Some(cansi::Color::BrightBlack) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack))
        }
        Some(cansi::Color::BrightRed) => Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed)),
        Some(cansi::Color::BrightGreen) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen))
        }
        Some(cansi::Color::BrightYellow) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow))
        }
        Some(cansi::Color::BrightBlue) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue))
        }
        Some(cansi::Color::BrightMagenta) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta))
        }
        Some(cansi::Color::BrightCyan) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))
        }
        Some(cansi::Color::BrightWhite) => {
            Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite))
        }
        None => None,
    }
}

impl<'text> StyledStream<'text> {
    pub(crate) fn new(s: &'text str) -> Self {
        let categorized = cansi::v3::categorise_text(s);
        Self {
            inner: categorized.into_iter().map(|x| x.into()).collect(),
        }
    }
}
