//! ANSI escape code hyperlink (OSC 8)
//!
//! For details on the protocol, see
//! [Hyperlinks (a.k.a. HTML-like anchors) in terminal emulators](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda)
//!
//! To detect support, see [supports-hyperlinks](https://crates.io/crates/supports-hyperlinks)

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(feature = "file")]
mod file;
#[cfg(feature = "file")]
mod hostname;
mod hyperlink;

#[cfg(feature = "file")]
pub use file::dir_to_url;
#[cfg(feature = "file")]
pub use file::file_to_url;
#[cfg(feature = "file")]
pub use file::path_to_url;
#[cfg(feature = "file")]
pub use file::Editor;
#[cfg(feature = "file")]
pub use file::ParseEditorError;
#[cfg(feature = "file")]
pub use hostname::hostname;
pub use hyperlink::Hyperlink;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
