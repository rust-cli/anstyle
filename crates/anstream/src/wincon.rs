use crate::adapter::WinconBytes;
use crate::IsTerminal;
use crate::RawStream;

/// Only pass printable data to the inner `Write`
#[cfg(feature = "wincon")] // here mostly for documentation purposes
#[derive(Debug)]
pub struct WinconStream<S>
where
    S: RawStream,
{
    raw: S,
    // `WinconBytes` is especially large compared to other variants of `AutoStream`, so boxing it
    // here so `AutoStream` doesn't have to discard one allocation and create another one when
    // calling `AutoStream::lock`
    state: Box<WinconBytes>,
}

impl<S> WinconStream<S>
where
    S: RawStream,
{
    /// Only pass printable data to the inner `Write`
    #[inline]
    pub fn new(raw: S) -> Self {
        Self {
            raw,
            state: Box::default(),
        }
    }

    /// Get the wrapped [`RawStream`]
    #[inline]
    pub fn into_inner(self) -> S {
        self.raw
    }

    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.raw.is_terminal()
    }
}

impl WinconStream<std::io::Stdout> {
    /// Get exclusive access to the `WinconStream`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    #[inline]
    pub fn lock(self) -> WinconStream<std::io::StdoutLock<'static>> {
        WinconStream {
            raw: self.raw.lock(),
            state: self.state,
        }
    }
}

impl WinconStream<std::io::Stderr> {
    /// Get exclusive access to the `WinconStream`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    #[inline]
    pub fn lock(self) -> WinconStream<std::io::StderrLock<'static>> {
        WinconStream {
            raw: self.raw.lock(),
            state: self.state,
        }
    }
}

impl<S> IsTerminal for WinconStream<S>
where
    S: RawStream,
{
    #[inline]
    fn is_terminal(&self) -> bool {
        self.is_terminal()
    }
}

impl<S> std::io::Write for WinconStream<S>
where
    S: RawStream,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for (style, printable) in self.state.extract_next(buf) {
            let fg = style.get_fg_color().and_then(cap_wincon_color);
            let bg = style.get_bg_color().and_then(cap_wincon_color);
            let written = self.raw.write_colored(fg, bg, printable.as_bytes())?;
            let possible = printable.len();
            if possible != written {
                // HACK: Unsupported atm
                break;
            }
        }
        Ok(buf.len())
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.raw.flush()
    }
}

fn cap_wincon_color(color: anstyle::Color) -> Option<anstyle::AnsiColor> {
    match color {
        anstyle::Color::Ansi(c) => Some(c),
        anstyle::Color::Ansi256(c) => c.into_ansi(),
        anstyle::Color::Rgb(_) => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use std::io::Write as _;

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn write_all_no_escapes(s in "\\PC*") {
            let buffer = crate::Buffer::new();
            let mut stream = WinconStream::new(buffer);
            stream.write_all(s.as_bytes()).unwrap();
            let buffer = stream.into_inner();
            let actual = std::str::from_utf8(buffer.as_ref()).unwrap();
            assert_eq!(s, actual);
        }

        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn write_byte_no_escapes(s in "\\PC*") {
            let buffer = crate::Buffer::new();
            let mut stream = WinconStream::new(buffer);
            for byte in s.as_bytes() {
                stream.write_all(&[*byte]).unwrap();
            }
            let buffer = stream.into_inner();
            let actual = std::str::from_utf8(buffer.as_ref()).unwrap();
            assert_eq!(s, actual);
        }

        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn write_all_random(s in any::<Vec<u8>>()) {
            let buffer = crate::Buffer::new();
            let mut stream = WinconStream::new(buffer);
            stream.write_all(s.as_slice()).unwrap();
        }

        #[test]
        #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
        fn write_byte_random(s in any::<Vec<u8>>()) {
            let buffer = crate::Buffer::new();
            let mut stream = WinconStream::new(buffer);
            for byte in s.as_slice() {
                stream.write_all(&[*byte]).unwrap();
            }
        }
    }
}
