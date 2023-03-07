use crate::adapter::WinconBytes;
use crate::Lockable;
use crate::RawStream;

/// Only pass printable data to the inner `Write`
#[cfg(feature = "wincon")] // here mostly for documentation purposes
pub struct WinconStream<S>
where
    S: RawStream,
{
    console: anstyle_wincon::Console<S>,
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
    pub fn new(console: anstyle_wincon::Console<S>) -> Self {
        Self {
            console,
            state: Box::default(),
        }
    }

    /// Get the wrapped [`RawStream`]
    #[inline]
    pub fn into_inner(self) -> anstyle_wincon::Console<S> {
        self.console
    }
}

impl<S> std::io::Write for WinconStream<S>
where
    S: RawStream,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut written = 0;
        let mut possible = 0;
        for (style, printable) in self.state.extract_next(buf) {
            let fg = style.get_fg_color().and_then(cap_wincon_color);
            let bg = style.get_bg_color().and_then(cap_wincon_color);
            written += self.console.write(fg, bg, printable.as_bytes())?;
            possible += printable.len();
            if possible != written {
                // HACK: Unsupported atm
                break;
            }
        }
        Ok(written)
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.console.flush()
    }
}

impl<S> Lockable for WinconStream<S>
where
    S: RawStream + Lockable,
    <S as Lockable>::Locked: RawStream,
{
    type Locked = WinconStream<<S as Lockable>::Locked>;

    #[inline]
    fn lock(self) -> Self::Locked {
        Self::Locked {
            console: self.console.lock(),
            state: self.state,
        }
    }
}

fn cap_wincon_color(color: anstyle::Color) -> Option<anstyle::AnsiColor> {
    match color {
        anstyle::Color::Ansi(c) => Some(c),
        anstyle::Color::XTerm(c) => c.into_ansi(),
        anstyle::Color::Rgb(_) => None,
    }
}
