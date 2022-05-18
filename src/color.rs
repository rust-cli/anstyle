/// Any ANSI color code scheme
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Ansi(AnsiColor),
    XTerm(XTermColor),
    Rgb(RgbColor),
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

/// Index into the 8-bit ANSI color palette
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct XTermColor(pub u8);

impl From<u8> for XTermColor {
    fn from(inner: u8) -> Self {
        Self(inner)
    }
}

/// 24-bit ANSI RGB color codes
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbColor(pub u8, pub u8, pub u8);
