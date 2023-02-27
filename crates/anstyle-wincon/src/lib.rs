#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod console;
mod stream;
#[cfg(windows)]
mod windows;

pub use console::Console;
pub use stream::WinconStream;
