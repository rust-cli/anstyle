use anstyle_parse::state::state_change;
use anstyle_parse::state::Action;
use anstyle_parse::state::State;

/// Strip ANSI escapes from a `&str`, returning the printable content
///
/// This can be used to take output from a program that includes escape sequences and write it
/// somewhere that does not easily support them, such as a log file.
///
/// For non-contiguous data, see [`StripStr`].
///
/// # Example
///
/// ```rust
/// use std::io::Write as _;
///
/// let styled_text = "\x1b[32mfoo\x1b[m bar";
/// let plain_str = anstyle_stream::adapter::strip_str(&styled_text).to_string();
/// assert_eq!(plain_str, "foo bar");
/// ```
#[inline]
pub fn strip_str(data: &str) -> StrippedStr<'_> {
    StrippedStr::new(data)
}

/// See [`strip_str`]
#[derive(Default, Debug)]
pub struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: State,
}

impl<'s> StrippedStr<'s> {
    #[inline]
    fn new(data: &'s str) -> Self {
        Self {
            bytes: data.as_bytes(),
            state: State::Ground,
        }
    }

    /// Create a [`String`] of the printable content
    #[inline]
    #[allow(clippy::inherent_to_string_shadow_display)] // Single-allocation implementation
    pub fn to_string(&self) -> String {
        use std::fmt::Write as _;
        let mut stripped = String::with_capacity(self.bytes.len());
        let _ = write!(&mut stripped, "{}", self);
        stripped
    }
}

impl<'s> std::fmt::Display for StrippedStr<'s> {
    /// **Note:** this does *not* exhaust the [`Iterator`]
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = Self {
            bytes: self.bytes,
            state: self.state,
        };
        for printable in iter {
            printable.fmt(f)?;
        }
        Ok(())
    }
}

impl<'s> Iterator for StrippedStr<'s> {
    type Item = &'s str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        next_str(&mut self.bytes, &mut self.state)
    }
}

/// Incrementally strip non-contiguous data
#[derive(Clone, Default, Debug)]
pub struct StripStr {
    state: State,
}

impl StripStr {
    /// Initial state
    pub fn new() -> Self {
        Default::default()
    }

    /// Strip the next segment of data
    pub fn strip_next<'s>(&'s mut self, data: &'s str) -> StripStrIter<'s> {
        StripStrIter {
            bytes: data.as_bytes(),
            state: &mut self.state,
        }
    }
}

/// See [`StripStr`]
#[derive(Debug)]
pub struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut State,
}

impl<'s> Iterator for StripStrIter<'s> {
    type Item = &'s str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        next_str(&mut self.bytes, self.state)
    }
}

#[inline]
fn next_str<'s>(bytes: &mut &'s [u8], state: &mut State) -> Option<&'s str> {
    let offset = bytes.iter().copied().position(|b| {
        let (next_state, action) = state_change(*state, b);
        if next_state != State::Anywhere {
            *state = next_state;
        }
        is_printable_str(action, b)
    });
    let (_, next) = bytes.split_at(offset.unwrap_or(bytes.len()));
    *bytes = next;
    *state = State::Ground;

    let offset = bytes.iter().copied().position(|b| {
        let (_next_state, action) = state_change(State::Ground, b);
        !is_printable_str(action, b)
    });
    let (printable, next) = bytes.split_at(offset.unwrap_or(bytes.len()));
    *bytes = next;
    if printable.is_empty() {
        None
    } else {
        let printable = unsafe {
            from_utf8_unchecked(
                printable,
                "`bytes` was validated as UTF-8, the parser preserves UTF-8 continuations",
            )
        };
        Some(printable)
    }
}

#[inline]
unsafe fn from_utf8_unchecked<'b>(bytes: &'b [u8], safety_justification: &'static str) -> &'b str {
    if cfg!(debug_assertions) {
        // Catch problems more quickly when testing
        std::str::from_utf8(bytes).expect(safety_justification)
    } else {
        std::str::from_utf8_unchecked(bytes)
    }
}

#[inline]
fn is_printable_str(action: Action, byte: u8) -> bool {
    action == Action::Print
        || action == Action::BeginUtf8
        // since we know the input is valid UTF-8, the only thing  we can do with
        // continuations is to print them
        || is_utf8_continuation(byte)
        || (action == Action::Execute && byte.is_ascii_whitespace())
}

#[inline]
fn is_utf8_continuation(b: u8) -> bool {
    matches!(b, 0x80..=0xbf)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    /// Model based off full parser
    fn parser_strip(bytes: &[u8]) -> String {
        #[derive(Default)]
        struct Strip(String);
        impl Strip {
            fn with_capacity(capacity: usize) -> Self {
                Self(String::with_capacity(capacity))
            }
        }
        impl anstyle_parse::Perform for Strip {
            fn print(&mut self, c: char) {
                self.0.push(c);
            }

            fn execute(&mut self, byte: u8) {
                if byte.is_ascii_whitespace() {
                    self.0.push(byte as char);
                }
            }
        }

        let mut stripped = Strip::with_capacity(bytes.len());
        let mut parser = anstyle_parse::Parser::<anstyle_parse::DefaultCharAccumulator>::new();
        for byte in bytes {
            parser.advance(&mut stripped, *byte);
        }
        stripped.0
    }

    /// Model verifying incremental parsing
    fn strip_chars(mut s: &str) -> String {
        let mut result = String::new();
        let mut state = StripStr::new();
        while !s.is_empty() {
            let mut indices = s.char_indices();
            indices.next(); // current
            let offset = indices.next().map(|(i, _)| i).unwrap_or_else(|| s.len());
            let (current, remainder) = s.split_at(offset);
            for printable in state.strip_next(current) {
                result.push_str(printable);
            }
            s = remainder;
        }
        result
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn strip_str_no_escapes(s in "\\PC*") {
            let expected = parser_strip(s.as_bytes());
            let actual = strip_str(&s).to_string();
            assert_eq!(expected, actual);
        }

        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn strip_chars_no_escapes(s in "\\PC*") {
            let expected = parser_strip(s.as_bytes());
            let actual = strip_chars(&s);
            assert_eq!(expected, actual);
        }
    }
}
