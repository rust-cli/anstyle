//! ANSI escape code hyperlink
//!
//! To detect support, see [supports-hyperlinks](https://crates.io/crates/supports-hyperlinks)

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod hyperlink;

pub use hyperlink::Hyperlink;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
