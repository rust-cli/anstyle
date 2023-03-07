use crate::adapter::StripBytes;
use crate::Lockable;
use crate::RawStream;

/// Only pass printable data to the inner `Write`
pub struct StripStream<S> {
    raw: S,
    state: StripBytes,
}

impl<S> StripStream<S>
where
    S: RawStream,
{
    /// Only pass printable data to the inner `Write`
    #[inline]
    pub fn new(raw: S) -> Self {
        Self {
            raw,
            state: Default::default(),
        }
    }
}

impl<S> std::io::Write for StripStream<S>
where
    S: RawStream,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let initial_state = self.state.clone();

        let mut written = 0;
        let mut possible = 0;
        for printable in self.state.strip_next(buf) {
            possible += printable.len();
            written += self.raw.write(printable)?;
            if possible != written {
                let divergence = &printable[written..];
                let offset = offset_to(buf, divergence);
                let consumed = &buf[offset..];
                self.state = initial_state;
                self.state.strip_next(consumed).last();
                break;
            }
        }
        Ok(written)
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.raw.flush()
    }

    // Provide explicit implementations of trait methods
    // - To reduce bookkeeping
    // - Avoid acquiring / releasing locks in a loop

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        for printable in self.state.strip_next(buf) {
            self.raw.write_all(printable)?;
        }
        Ok(())
    }

    // Not bothering with `write_fmt` as it just calls `write_all`
}

#[inline]
fn offset_to(total: &[u8], subslice: &[u8]) -> usize {
    let total = total.as_ptr();
    let subslice = subslice.as_ptr();

    debug_assert!(
        total <= subslice,
        "`Offset::offset_to` only accepts slices of `self`"
    );
    subslice as usize - total as usize
}

impl<S> Lockable for StripStream<S>
where
    S: Lockable,
{
    type Locked = StripStream<<S as Lockable>::Locked>;

    #[inline]
    fn lock(self) -> Self::Locked {
        Self::Locked {
            raw: self.raw.lock(),
            state: self.state,
        }
    }
}
