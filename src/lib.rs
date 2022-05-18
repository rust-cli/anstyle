//! ANSI Text Styling
//!
//! The core type is [`Style`].
//!
//! # Examples
//!
//! ```rust
//! let style = anstyle::Style::new().bold();
//! ```

mod color;
mod effect;
mod reset;
mod style;

pub use color::*;
pub use effect::*;
pub use reset::*;
pub use style::*;
