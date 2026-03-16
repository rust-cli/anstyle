//! ANSI escape code progress reporting (OSC 9;4)
//!
//! For details on the protocol, see
//! [ConEmu's docs](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC)

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod progress;

pub use progress::TermProgress;
pub use progress::TermProgressStatus;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
