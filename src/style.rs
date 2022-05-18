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
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            effects: crate::Effects::new(),
        }
    }

    /// Set foreground color
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().fg_color(anstyle::AnsiColor::Red.into());
    /// ```
    pub const fn fg_color(mut self, fg: crate::Color) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Set background color
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().bg_color(anstyle::AnsiColor::Red.into());
    /// ```
    pub const fn bg_color(mut self, bg: crate::Color) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Set text effects
    ///
    /// # Examples
    ///
    /// ```rust
    /// let style = anstyle::Style::new().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE);
    /// ```
    pub const fn effects(mut self, effects: crate::Effects) -> Self {
        self.effects = effects;
        self
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
    pub const fn strikethrough(mut self) -> Self {
        self.effects = self.effects.insert(crate::Effects::STRIKETHROUGH);
        self
    }
}

/// # Reflection
impl Style {
    pub const fn get_fg_color(self) -> Option<crate::Color> {
        self.fg
    }

    pub const fn get_bg_color(self) -> Option<crate::Color> {
        self.bg
    }

    pub const fn get_effects(self) -> crate::Effects {
        self.effects
    }

    /// Check if no effects are enabled
    pub const fn is_plain(self) -> bool {
        self.fg.is_none() && self.bg.is_none() && self.effects.is_plain()
    }
}

/// # Examples
///
/// ```rust
/// let style: anstyle::Style = anstyle::Effects::BOLD.into();
/// ```
impl From<crate::Effects> for Style {
    fn from(effects: crate::Effects) -> Self {
        Self::new().effects(effects)
    }
}

/// # Examples
///
/// ```rust
/// let style = anstyle::Style::new() | anstyle::Effects::BOLD.into();
/// ```
impl std::ops::BitOr<crate::Effects> for Style {
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
impl std::ops::BitOrAssign<crate::Effects> for Style {
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
impl std::ops::Sub<crate::Effects> for Style {
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
impl std::ops::SubAssign<crate::Effects> for Style {
    #[inline]
    fn sub_assign(&mut self, other: crate::Effects) {
        self.effects -= other;
    }
}
