/// ANSI Text styling
///
/// # Examples
///
/// ```rust
/// let style = anstyle::Style::new().bold();
/// ```
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}

/// # Core
impl Style {
    /// No effects enabled
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            underline: None,
            effects: crate::Effects::new(),
        }
    }

    /// Set foreground color
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));
    /// ```
    #[must_use]
    #[inline]
    pub const fn fg_color(mut self, fg: Option<crate::Color>) -> Self {
        self.fg = fg;
        self
    }

    /// Set background color
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().bg_color(Some(anstyle::AnsiColor::Red.into()));
    /// ```
    #[must_use]
    #[inline]
    pub const fn bg_color(mut self, bg: Option<crate::Color>) -> Self {
        self.bg = bg;
        self
    }

    /// Set underline color
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().underline_color(Some(anstyle::AnsiColor::Red.into()));
    /// ```
    #[must_use]
    #[inline]
    pub const fn underline_color(mut self, underline: Option<crate::Color>) -> Self {
        self.underline = underline;
        self
    }

    /// Set text effects
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE);
    /// ```
    #[must_use]
    #[inline]
    pub const fn effects(mut self, effects: crate::Effects) -> Self {
        self.effects = effects;
        self
    }

    /// Render the ANSI code
    #[inline]
    pub fn render(self) -> impl core::fmt::Display {
        StyleDisplay(self)
    }

    /// Renders the relevant [`Reset`][crate::Reset] code
    ///
    /// Unlike [`Reset::render`][crate::Reset::render], this will elide the code if there is nothing to reset.
    #[inline]
    pub fn render_reset(self) -> impl core::fmt::Display {
        ResetDisplay(self != Self::new())
    }
}

/// # Convenience
impl Style {
    /// Apply `bold` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().bold();
    /// ```
    #[must_use]
    #[inline]
    pub const fn bold(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::BOLD);
        self
    }

    /// Apply `dimmed` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().dimmed();
    /// ```
    #[must_use]
    #[inline]
    pub const fn dimmed(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::DIMMED);
        self
    }

    /// Apply `italic` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().italic();
    /// ```
    #[must_use]
    #[inline]
    pub const fn italic(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::ITALIC);
        self
    }

    /// Apply `underline` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().underline();
    /// ```
    #[must_use]
    #[inline]
    pub const fn underline(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::UNDERLINE);
        self
    }

    /// Apply `blink` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().blink();
    /// ```
    #[must_use]
    #[inline]
    pub const fn blink(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::BLINK);
        self
    }

    /// Apply `invert` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().invert();
    /// ```
    #[must_use]
    #[inline]
    pub const fn invert(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::INVERT);
        self
    }

    /// Apply `hidden` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().hidden();
    /// ```
    #[must_use]
    #[inline]
    pub const fn hidden(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::HIDDEN);
        self
    }

    /// Apply `strikethrough` effect
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().strikethrough();
    /// ```
    #[must_use]
    #[inline]
    pub const fn strikethrough(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::STRIKETHROUGH);
        self
    }
}

/// # Reflection
impl Style {
    #[inline]
    pub const fn get_fg_color(self) -> Option<crate::Color> {
        self.fg
    }

    #[inline]
    pub const fn get_bg_color(self) -> Option<crate::Color> {
        self.bg
    }

    #[inline]
    pub const fn get_underline_color(self) -> Option<crate::Color> {
        self.underline
    }

    #[inline]
    pub const fn get_effects(self) -> crate::Effects {
        self.effects
    }

    /// Check if no effects are enabled
    #[inline]
    pub const fn is_plain(self) -> bool {
        self.fg.is_none()
            && self.bg.is_none()
            && self.underline.is_none()
            && self.effects.is_plain()
    }
}

/// Define style with specified foreground color
///
/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::Color::from((0, 0, 0)).into();
/// ```
impl From<crate::Color> for Style {
    #[inline]
    fn from(color: crate::Color) -> Self {
        Self::new().fg_color(Some(color))
    }
}

/// Define style with specified foreground color
///
/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::AnsiColor::Black.into();
/// ```
impl From<crate::AnsiColor> for Style {
    #[inline]
    fn from(color: crate::AnsiColor) -> Self {
        Self::new().fg_color(Some(color.into()))
    }
}

/// Define style with specified foreground color
///
/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::Ansi256Color(0).into();
/// ```
impl From<crate::Ansi256Color> for Style {
    #[inline]
    fn from(color: crate::Ansi256Color) -> Self {
        Self::new().fg_color(Some(color.into()))
    }
}

/// Define style with specified foreground color
///
/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::RgbColor(0, 0, 0).into();
/// ```
impl From<crate::RgbColor> for Style {
    #[inline]
    fn from(color: crate::RgbColor) -> Self {
        Self::new().fg_color(Some(color.into()))
    }
}

/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::Effects::BOLD.into();
/// ```
impl From<crate::Effects> for Style {
    #[inline]
    fn from(effects: crate::Effects) -> Self {
        Self::new().effects(effects)
    }
}

/// # Examples
///
/// ```rust
/// let style = anstyle::Style::new() | anstyle::Effects::BOLD.into();
/// ```
impl core::ops::BitOr<crate::Effects> for Style {
    type Output = Self;

    #[inline(always)]
    fn bitor(mut self, rhs: crate::Effects) -> Self {
        self.effects |= rhs;
        self
    }
}

/// # Examples
///
/// ```rust
/// let mut style = anstyle::Style::new();
/// style |= anstyle::Effects::BOLD.into();
/// ```
impl core::ops::BitOrAssign<crate::Effects> for Style {
    #[inline]
    fn bitor_assign(&mut self, other: crate::Effects) {
        self.effects |= other;
    }
}

/// # Examples
///
/// ```rust
/// let style = anstyle::Style::new().bold().underline() - anstyle::Effects::BOLD.into();
/// ```
impl core::ops::Sub<crate::Effects> for Style {
    type Output = Self;

    #[inline]
    fn sub(mut self, other: crate::Effects) -> Self {
        self.effects -= other;
        self
    }
}

/// # Examples
///
/// ```rust
/// let mut style = anstyle::Style::new().bold().underline();
/// style -= anstyle::Effects::BOLD.into();
/// ```
impl core::ops::SubAssign<crate::Effects> for Style {
    #[inline]
    fn sub_assign(&mut self, other: crate::Effects) {
        self.effects -= other;
    }
}

/// # Examples
///
/// ```rust
/// let color = anstyle::RgbColor(0, 0, 0);
/// assert_eq!(anstyle::Style::new().fg_color(Some(color.into())), color);
/// assert_ne!(color | anstyle::Effects::BOLD, color);
/// ```
impl<C: Into<crate::Color> + Clone> core::cmp::PartialEq<C> for Style {
    #[inline]
    fn eq(&self, other: &C) -> bool {
        let other = other.clone().into();
        let other = Self::from(other);
        *self == other
    }
}

/// # Examples
///
/// ```rust
/// let effects = anstyle::Effects::BOLD;
/// assert_eq!(anstyle::Style::new().effects(effects), effects);
/// assert_ne!(anstyle::Effects::UNDERLINE | effects, effects);
/// assert_ne!(anstyle::RgbColor(0, 0, 0) | effects, effects);
/// ```
impl core::cmp::PartialEq<crate::Effects> for Style {
    #[inline]
    fn eq(&self, other: &crate::Effects) -> bool {
        let other = Self::from(*other);
        *self == other
    }
}

struct StyleDisplay(Style);

impl core::fmt::Display for StyleDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_plain() {
            return Ok(());
        }

        write!(f, "\x1B[")?;

        let mut first = true;

        for index in self.0.effects.index_iter() {
            separator(f, &mut first)?;
            write!(f, "{}", crate::effect::METADATA[index].primary)?;
            if let Some(secondary) = crate::effect::METADATA[index].secondary {
                write!(f, ":{}", secondary)?;
            }
        }

        if let Some(fg) = self.0.fg {
            separator(f, &mut first)?;
            fg.ansi_fmt(f, crate::ColorContext::Foreground)?;
        }

        if let Some(bg) = self.0.bg {
            separator(f, &mut first)?;
            bg.ansi_fmt(f, crate::ColorContext::Background)?;
        }

        if let Some(underline) = self.0.underline {
            separator(f, &mut first)?;
            underline.ansi_fmt(f, crate::ColorContext::Underline)?;
        }

        write!(f, "m")?;
        Ok(())
    }
}

#[inline]
fn separator(f: &mut core::fmt::Formatter<'_>, first: &mut bool) -> core::fmt::Result {
    if *first {
        *first = false;
        Ok(())
    } else {
        write!(f, ";")
    }
}

struct ResetDisplay(bool);

impl core::fmt::Display for ResetDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 {
            write!(f, "\x1B[0m")
        } else {
            Ok(())
        }
    }
}
