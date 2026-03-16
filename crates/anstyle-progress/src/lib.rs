//! ANSI escape code progress reporting (OSC 9;4)
//!
//! For details on the protocol, see
//! [ConEmu's docs](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC),
//! [Tutorial: Set the progress bar in the Windows Terminal](https://learn.microsoft.com/en-us/windows/terminal/tutorials/progress-bar-sequences)

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod progress;
#[cfg(feature = "std")]
mod query;

pub use progress::TermProgress;
pub use progress::TermProgressStatus;
#[cfg(feature = "std")]
pub use query::supports_term_progress;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
