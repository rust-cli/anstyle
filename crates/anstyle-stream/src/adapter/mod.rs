//! Gracefully degrade styled output

mod strip;

pub use strip::strip_bytes;
pub use strip::strip_str;
pub use strip::StripBytes;
pub use strip::StripBytesIter;
pub use strip::StripStr;
pub use strip::StripStrIter;
pub use strip::StrippedBytes;
pub use strip::StrippedStr;
