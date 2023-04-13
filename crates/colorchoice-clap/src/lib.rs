//! Mixin a clap argument for colored output selection
//!
//! ## Examples
//!
//! ```rust
//! // ...
//! #[derive(Debug, clap::Parser)]
//! struct Cli {
//!     #[command(flatten)]
//!     color: colorchoice_clap::Color,
//! }
//! ```

/// Mixin a clap argument for colored output selection
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, clap::Args)]
#[command(about = None, long_about = None)]
pub struct Color {
    /// Controls when to use color.
    #[arg(long, default_value_t = ColorChoice::Auto, value_name = "WHEN", value_enum, global = true)]
    pub color: ColorChoice,
}

impl Color {
    /// Set the user selection on `colorchoice`
    pub fn write_global(&self) {
        self.as_choice().write_global();
    }

    /// Get the user's selection
    pub fn as_choice(&self) -> colorchoice::ColorChoice {
        match self.color {
            ColorChoice::Auto => colorchoice::ColorChoice::Auto,
            ColorChoice::Always => colorchoice::ColorChoice::Always,
            ColorChoice::Never => colorchoice::ColorChoice::Never,
        }
    }
}

/// Argument value for when to color output
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    /// Report all `possible_values`
    pub fn possible_values() -> impl Iterator<Item = clap::builder::PossibleValue> {
        use clap::ValueEnum;
        Self::value_variants()
            .iter()
            .filter_map(clap::ValueEnum::to_possible_value)
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => AUTO,
            Self::Always => ALWAYS,
            Self::Never => NEVER,
        }
    }
}

impl Default for ColorChoice {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::str::FromStr for ColorChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as clap::ValueEnum>::from_str(s, false)
    }
}

impl clap::ValueEnum for ColorChoice {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Auto, Self::Always, Self::Never]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_str()))
    }
}

const AUTO: &str = "auto";
const ALWAYS: &str = "always";
const NEVER: &str = "never";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[clap(flatten)]
            color: Color,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
