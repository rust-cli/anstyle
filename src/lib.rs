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
//! - [anstyle-termcolor](https:://docs.rs/anstyle-termcolor)
//! - [anstyle-owo-colors](https:://docs.rs/anstyle-owo-colors)
//! - [anstyle-yansi](https:://docs.rs/anstyle-yansi)
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
