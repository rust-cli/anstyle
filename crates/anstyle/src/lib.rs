//! ANSI Text Styling
//!
//! `anstyle` provides core types describing [ANSI styling escape
//! codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for interoperability
//! between crates.
//!
//! Example use cases:
//! - An argument parser allowing callers to define the colors used in the help-output without
//!   putting the text formatting crate in the public API
//! - A style description parser that can work with any text formatting crate
//!
//! Priorities:
//! 1. API stability
//! 2. Low compile-time and binary-size overhead
//! 3. `const` friendly API for callers to statically define their stylesheet
//!
//! For integration with text styling crate, see:
//! - [anstyle-ansi-term](https://docs.rs/anstyle-ansi-term)
//! - [anstyle-crossterm](https://docs.rs/anstyle-crossterm)
//! - [anstyle-termcolor](https://docs.rs/anstyle-termcolor)
//! - [anstyle-owo-colors](https://docs.rs/anstyle-owo-colors)
//! - [anstyle-yansi](https://docs.rs/anstyle-yansi)
//!
//! General utilities:
//! - [anstyle-git](https://docs.s/anstyle-git): Parse Git style descriptions
//! - [anstyle-ls](https://docs.s/anstyle-ls): Parse LS_COLORS style descriptions
//! - [anstyle-lossy](https://docs.s/anstyle-lossy): Convert between `anstyle::Color` types
//! - [anstyle-roff](https://docs.s/anstyle-roff): For converting to ROFF
//!
//! # Examples
//!
//! The core type is [`Style`]:
//! ```rust
//! let style = anstyle::Style::new().bold();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod color;
mod effect;
mod reset;
mod style;

pub use color::*;
pub use effect::*;
pub use reset::*;
pub use style::*;
