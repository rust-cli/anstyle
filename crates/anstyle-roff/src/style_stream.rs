use anstyle::{AnsiColor, Color as AColor, Effects, Style};
use cansi::{v3::CategorisedSlice, Color, Intensity};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone)]
pub(crate) struct StyledStream<'text> {
    inner: Vec<StyledStr<'text>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct StyledStr<'text> {
    pub text: &'text str,
    pub style: Style,
}

impl<'text> From<CategorisedSlice<'text>> for StyledStr<'text> {
    fn from(category: CategorisedSlice<'text>) -> Self {
        let mut style = Style::new();
        style = style
            .fg_color(cansi_to_anstyle_color(category.fg))
            .bg_color(cansi_to_anstyle_color(category.bg));

        let effects = create_effects(&category);
        style = style.effects(effects);

        Self {
            text: category.text,
            style,
        }
    }
}

fn create_effects(category: &CategorisedSlice) -> Effects {
    Effects::new()
        .set(Effects::ITALIC, category.italic.unwrap_or(false))
        .set(Effects::BLINK, category.blink.unwrap_or(false))
        .set(Effects::INVERT, category.reversed.unwrap_or(false))
        .set(Effects::HIDDEN, category.hidden.unwrap_or(false))
        .set(
            Effects::STRIKETHROUGH,
            category.strikethrough.unwrap_or(false),
        )
        .set(Effects::UNDERLINE, category.underline.unwrap_or(false))
        .set(Effects::BOLD, is_bold(category.intensity))
        .set(Effects::DIMMED, is_faint(category.intensity))
}

fn is_bold(intensity: Option<Intensity>) -> bool {
    matches!(intensity, Some(Intensity::Bold))
}

fn is_faint(intensity: Option<Intensity>) -> bool {
    matches!(intensity, Some(Intensity::Faint))
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

fn cansi_to_anstyle_color(color: Option<Color>) -> Option<AColor> {
    match color {
        Some(Color::Black) => Some(AColor::Ansi(AnsiColor::Black)),
        Some(Color::Red) => Some(AColor::Ansi(AnsiColor::Red)),
        Some(Color::Green) => Some(AColor::Ansi(AnsiColor::Green)),
        Some(Color::Yellow) => Some(AColor::Ansi(AnsiColor::Yellow)),
        Some(Color::Blue) => Some(AColor::Ansi(AnsiColor::Blue)),
        Some(Color::Magenta) => Some(AColor::Ansi(AnsiColor::Magenta)),
        Some(Color::Cyan) => Some(AColor::Ansi(AnsiColor::Cyan)),
        Some(Color::White) => Some(AColor::Ansi(AnsiColor::White)),
        Some(Color::BrightBlack) => Some(AColor::Ansi(AnsiColor::BrightBlack)),
        Some(Color::BrightRed) => Some(AColor::Ansi(AnsiColor::BrightRed)),
        Some(Color::BrightGreen) => Some(AColor::Ansi(AnsiColor::BrightGreen)),
        Some(Color::BrightYellow) => Some(AColor::Ansi(AnsiColor::BrightYellow)),
        Some(Color::BrightBlue) => Some(AColor::Ansi(AnsiColor::BrightBlue)),
        Some(Color::BrightMagenta) => Some(AColor::Ansi(AnsiColor::BrightMagenta)),
        Some(Color::BrightCyan) => Some(AColor::Ansi(AnsiColor::BrightCyan)),
        Some(Color::BrightWhite) => Some(AColor::Ansi(AnsiColor::BrightWhite)),
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

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! styled_str {
        ($text: literal, $(Color:$color_key:literal:$color_val:expr;)* $(Intensity:$intensity:expr;)? $(Effects:$key:literal:$value:expr;)* ) => {
            {
            let mut cat_text = CategorisedSlice {
                text: $text,
                start: 0,
                end: 5,
                fg: None,
                bg: None,
                intensity: Some(Intensity::Normal),
                italic: None,
                underline: None,
                blink: None,
                reversed: None,
                strikethrough: None,
                hidden: None,
            };

            $(
            match $color_key {
                "fg" => cat_text.fg = $color_val,
                "bg" => cat_text.bg = $color_val,
                _ => panic!("Not A Valid key for color")
            };
            )*
            $(
                cat_text.intensity = $intensity;
            )?
            $(
            match $key {
                "underline" => cat_text.underline = $value,
                "italic" => cat_text.italic= $value,
                "blink" => cat_text.blink= $value,
                "reversed" => cat_text.reversed = $value,
                "strikethrough" => cat_text.strikethrough= $value,
                "hidden" => cat_text.hidden= $value,
                _ => panic!("Not A Valid key for effects")
            };
            )*
            cat_text
        }}
    }

    #[test]
    fn from_categorized_underlined() {
        let categorised = styled_str!("Hello", Effects:"underline":Some(true););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::UNDERLINE));
    }

    #[test]
    fn from_categorized_blink() {
        let categorised = styled_str!("Hello", Effects:"blink":Some(true););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::BLINK));
    }

    #[test]
    fn from_categorized_reversed() {
        let categorised = styled_str!("Hello", Effects:"reversed":Some(true););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::INVERT));
    }

    #[test]
    fn from_categorized_strikthrough() {
        let categorised = styled_str!("Hello", Effects:"strikethrough":Some(true););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str
            .style
            .get_effects()
            .contains(Effects::STRIKETHROUGH));
    }

    #[test]
    fn from_categorized_hidden() {
        let categorised = styled_str!("Hello", Effects:"hidden":Some(true););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::HIDDEN));
    }

    #[test]
    fn from_categorized_bg() {
        let categorised = styled_str!("Hello", Color:"bg":Some(Color::Blue););
        let styled_str: StyledStr = categorised.into();
        assert!(matches!(
            styled_str.style.get_bg_color(),
            Some(AColor::Ansi(AnsiColor::Blue))
        ));
    }

    #[test]
    fn from_categorized_fg() {
        let categorised = styled_str!("Hello", Color:"fg":Some(Color::Blue););
        let styled_str: StyledStr = categorised.into();
        assert!(matches!(
            styled_str.style.get_fg_color(),
            Some(AColor::Ansi(AnsiColor::Blue))
        ));
    }

    #[test]
    fn from_categorized_bold() {
        let categorised = styled_str!("Hello", Intensity:Some(Intensity::Bold););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::BOLD));
    }

    #[test]
    fn from_categorized_faint() {
        let categorised = styled_str!("Hello", Intensity:Some(Intensity::Faint););
        let styled_str: StyledStr = categorised.into();
        assert!(styled_str.style.get_effects().contains(Effects::DIMMED));
    }
}
