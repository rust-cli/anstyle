/// Any ANSI color code scheme
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Ansi(AnsiColor),
    XTerm(XTermColor),
    Rgb(RgbColor),
}

impl Color {
    /// Render the ANSI code for a foreground color
    pub fn render_fg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Foreground,
        }
    }

    /// Render the ANSI code for a background color
    pub fn render_bg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Background,
        }
    }

    pub(crate) fn ansi_fmt(
        &self,
        f: &mut dyn core::fmt::Write,
        context: ColorContext,
    ) -> core::fmt::Result {
        match self {
            Self::Ansi(color) => color.ansi_fmt(f, context),
            Self::XTerm(color) => color.ansi_fmt(f, context),
            Self::Rgb(color) => color.ansi_fmt(f, context),
        }
    }
}

impl AnsiColorFmt for Color {
    fn ansi_fmt(&self, f: &mut dyn core::fmt::Write, context: ColorContext) -> core::fmt::Result {
        self.ansi_fmt(f, context)
    }
}

impl From<AnsiColor> for Color {
    fn from(inner: AnsiColor) -> Self {
        Self::Ansi(inner)
    }
}

impl From<XTermColor> for Color {
    fn from(inner: XTermColor) -> Self {
        Self::XTerm(inner)
    }
}

impl From<RgbColor> for Color {
    fn from(inner: RgbColor) -> Self {
        Self::Rgb(inner)
    }
}

impl From<u8> for Color {
    fn from(inner: u8) -> Self {
        Self::XTerm(inner.into())
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(inner: (u8, u8, u8)) -> Self {
        Self::Rgb(inner.into())
    }
}

/// Define style with specified foreground and background colors
///
/// # Examples
///
/// ```rust
/// let black = anstyle::Color::from((0, 0, 0));
/// let white = anstyle::Color::from((0xff, 0xff, 0xff));
/// let style = black | white;
/// ```
impl<C: Into<Color>> core::ops::BitOr<C> for Color {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: C) -> Self::Output {
        crate::Style::new()
            .fg_color(Some(self))
            .bg_color(Some(rhs.into()))
    }
}

/// Define style with specified foreground color and effects
///
/// # Examples
///
/// ```rust
/// let color = anstyle::Color::from((0, 0, 0));
/// let style = color | anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// ```
impl core::ops::BitOr<crate::Effects> for Color {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: crate::Effects) -> Self::Output {
        crate::Style::new().fg_color(Some(self)) | rhs
    }
}

/// Available 4-bit ANSI color codes
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnsiColor {
    /// Black: #0 (foreground code `30`, background code `40`).
    Black,

    /// Red: #1 (foreground code `31`, background code `41`).
    Red,

    /// Green: #2 (foreground code `32`, background code `42`).
    Green,

    /// Yellow: #3 (foreground code `33`, background code `43`).
    Yellow,

    /// Blue: #4 (foreground code `34`, background code `44`).
    Blue,

    /// Magenta: #5 (foreground code `35`, background code `45`).
    Magenta,

    /// Cyan: #6 (foreground code `36`, background code `46`).
    Cyan,

    /// White: #7 (foreground code `37`, background code `47`).
    White,

    /// Bright black: #0 (foreground code `90`, background code `100`).
    BrightBlack,

    /// Bright red: #1 (foreground code `91`, background code `101`).
    BrightRed,

    /// Bright green: #2 (foreground code `92`, background code `102`).
    BrightGreen,

    /// Bright yellow: #3 (foreground code `93`, background code `103`).
    BrightYellow,

    /// Bright blue: #4 (foreground code `94`, background code `104`).
    BrightBlue,

    /// Bright magenta: #5 (foreground code `95`, background code `105`).
    BrightMagenta,

    /// Bright cyan: #6 (foreground code `96`, background code `106`).
    BrightCyan,

    /// Bright white: #7 (foreground code `97`, background code `107`).
    BrightWhite,
}

impl AnsiColor {
    /// Render the ANSI code for a foreground color
    pub fn render_fg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Foreground,
        }
    }

    /// Render the ANSI code for a background color
    pub fn render_bg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Background,
        }
    }

    fn is_bright(self) -> bool {
        match self {
            Self::Black => false,
            Self::Red => false,
            Self::Green => false,
            Self::Yellow => false,
            Self::Blue => false,
            Self::Magenta => false,
            Self::Cyan => false,
            Self::White => false,
            Self::BrightBlack => true,
            Self::BrightRed => true,
            Self::BrightGreen => true,
            Self::BrightYellow => true,
            Self::BrightBlue => true,
            Self::BrightMagenta => true,
            Self::BrightCyan => true,
            Self::BrightWhite => true,
        }
    }
}

impl AnsiColorFmt for AnsiColor {
    fn ansi_fmt(&self, f: &mut dyn core::fmt::Write, context: ColorContext) -> core::fmt::Result {
        match (context, self.is_bright()) {
            (ColorContext::Foreground, true) => write!(f, "10"),
            (ColorContext::Background, true) => write!(f, "9"),
            (ColorContext::Foreground, false) => write!(f, "4"),
            (ColorContext::Background, false) => write!(f, "3"),
        }?;

        match self {
            Self::Black => write!(f, "0"),
            Self::Red => write!(f, "1"),
            Self::Green => write!(f, "2"),
            Self::Yellow => write!(f, "3"),
            Self::Blue => write!(f, "4"),
            Self::Magenta => write!(f, "5"),
            Self::Cyan => write!(f, "6"),
            Self::White => write!(f, "7"),
            Self::BrightBlack => write!(f, "0"),
            Self::BrightRed => write!(f, "1"),
            Self::BrightGreen => write!(f, "2"),
            Self::BrightYellow => write!(f, "3"),
            Self::BrightBlue => write!(f, "4"),
            Self::BrightMagenta => write!(f, "5"),
            Self::BrightCyan => write!(f, "6"),
            Self::BrightWhite => write!(f, "7"),
        }
    }
}

/// Define style with specified foreground and background colors
///
/// # Examples
///
/// ```rust
/// let black = anstyle::AnsiColor::Black;
/// let white = anstyle::AnsiColor::White;
/// let style = black | white;
/// ```
impl<C: Into<Color>> core::ops::BitOr<C> for AnsiColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: C) -> Self::Output {
        crate::Style::new()
            .fg_color(Some(self.into()))
            .bg_color(Some(rhs.into()))
    }
}

/// Define style with specified foreground color and effects
///
/// # Examples
///
/// ```rust
/// let color = anstyle::AnsiColor::Black;
/// let style = color | anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// ```
impl core::ops::BitOr<crate::Effects> for AnsiColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: crate::Effects) -> Self::Output {
        crate::Style::new().fg_color(Some(self.into())) | rhs
    }
}

/// Index into the 8-bit ANSI color palette
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XTermColor(pub u8);

impl XTermColor {
    pub fn index(self) -> u8 {
        self.0
    }

    /// Render the ANSI code for a foreground color
    pub fn render_fg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Foreground,
        }
    }

    /// Render the ANSI code for a background color
    pub fn render_bg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Background,
        }
    }
}

impl AnsiColorFmt for XTermColor {
    fn ansi_fmt(&self, f: &mut dyn core::fmt::Write, context: ColorContext) -> core::fmt::Result {
        match context {
            ColorContext::Foreground => {
                write!(f, "48;")?;
            }
            ColorContext::Background => {
                write!(f, "38;")?;
            }
        }
        write!(f, "5;{}", self.index())
    }
}

impl From<u8> for XTermColor {
    fn from(inner: u8) -> Self {
        Self(inner)
    }
}

impl From<AnsiColor> for XTermColor {
    fn from(inner: AnsiColor) -> Self {
        match inner {
            AnsiColor::Black => 0.into(),
            AnsiColor::Red => 1.into(),
            AnsiColor::Green => 2.into(),
            AnsiColor::Yellow => 3.into(),
            AnsiColor::Blue => 4.into(),
            AnsiColor::Magenta => 5.into(),
            AnsiColor::Cyan => 6.into(),
            AnsiColor::White => 7.into(),
            AnsiColor::BrightBlack => 8.into(),
            AnsiColor::BrightRed => 9.into(),
            AnsiColor::BrightGreen => 10.into(),
            AnsiColor::BrightYellow => 11.into(),
            AnsiColor::BrightBlue => 12.into(),
            AnsiColor::BrightMagenta => 13.into(),
            AnsiColor::BrightCyan => 14.into(),
            AnsiColor::BrightWhite => 15.into(),
        }
    }
}

/// Define style with specified foreground and background colors
///
/// # Examples
///
/// ```rust
/// let black = anstyle::XTermColor(16);
/// let white = anstyle::XTermColor(231);
/// let style = black | white;
/// ```
impl<C: Into<Color>> core::ops::BitOr<C> for XTermColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: C) -> Self::Output {
        crate::Style::new()
            .fg_color(Some(self.into()))
            .bg_color(Some(rhs.into()))
    }
}

/// Define style with specified foreground color and effects
///
/// # Examples
///
/// ```rust
/// let color = anstyle::XTermColor(0);
/// let style = color | anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// ```
impl core::ops::BitOr<crate::Effects> for XTermColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: crate::Effects) -> Self::Output {
        crate::Style::new().fg_color(Some(self.into())) | rhs
    }
}

/// 24-bit ANSI RGB color codes
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub fn r(self) -> u8 {
        self.0
    }

    pub fn g(self) -> u8 {
        self.1
    }

    pub fn b(self) -> u8 {
        self.2
    }

    /// Render the ANSI code for a foreground color
    pub fn render_fg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Foreground,
        }
    }

    /// Render the ANSI code for a background color
    pub fn render_bg(self) -> impl core::fmt::Display {
        DisplayColor {
            color: self,
            context: ColorContext::Background,
        }
    }
}

impl AnsiColorFmt for RgbColor {
    fn ansi_fmt(&self, f: &mut dyn core::fmt::Write, context: ColorContext) -> core::fmt::Result {
        match context {
            ColorContext::Foreground => {
                write!(f, "48;")?;
            }
            ColorContext::Background => {
                write!(f, "38;")?;
            }
        }
        write!(f, "2;{};{};{}", self.r(), self.g(), self.b())
    }
}

impl From<(u8, u8, u8)> for RgbColor {
    fn from(inner: (u8, u8, u8)) -> Self {
        let (r, g, b) = inner;
        Self(r, g, b)
    }
}

/// Define style with specified foreground and background colors
///
/// # Examples
///
/// ```rust
/// let black = anstyle::RgbColor(0, 0, 0);
/// let white = anstyle::RgbColor(0xff, 0xff, 0xff);
/// let style = black | white;
/// ```
impl<C: Into<Color>> core::ops::BitOr<C> for RgbColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: C) -> Self::Output {
        crate::Style::new()
            .fg_color(Some(self.into()))
            .bg_color(Some(rhs.into()))
    }
}

/// Define style with specified foreground color and effects
///
/// # Examples
///
/// ```rust
/// let color = anstyle::RgbColor(0, 0, 0);
/// let style = color | anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// ```
impl core::ops::BitOr<crate::Effects> for RgbColor {
    type Output = crate::Style;

    #[inline(always)]
    fn bitor(self, rhs: crate::Effects) -> Self::Output {
        crate::Style::new().fg_color(Some(self.into())) | rhs
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ColorContext {
    Background,
    Foreground,
}

trait AnsiColorFmt {
    fn ansi_fmt(&self, f: &mut dyn core::fmt::Write, context: ColorContext) -> core::fmt::Result;
}

struct DisplayColor<C: AnsiColorFmt> {
    color: C,
    context: ColorContext,
}

impl<C: AnsiColorFmt> core::fmt::Display for DisplayColor<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\x1B[")?;
        self.color.ansi_fmt(f, self.context)?;
        write!(f, "m")?;
        Ok(())
    }
}
