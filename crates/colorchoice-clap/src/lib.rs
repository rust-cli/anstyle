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

pub use clap::ColorChoice;

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
