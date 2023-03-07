use crate::Lockable;
use crate::StripStream;

/// [`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities
pub struct AutoStream<W> {
    write: StreamInner<W>,
}

enum StreamInner<W> {
    PassThrough(W),
    Strip(StripStream<W>),
}

impl<W> AutoStream<W>
where
    W: std::io::Write,
{
    /// Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`
    /// supports.
    #[inline]
    pub fn always_ansi(write: W) -> Self {
        #[cfg(feature = "auto")]
        let _ = concolor_query::windows::enable_ansi_colors();
        Self::always_ansi_(write)
    }

    fn always_ansi_(write: W) -> Self {
        let write = StreamInner::PassThrough(write);
        AutoStream { write }
    }

    /// Only pass printable data to the inner `Write`.
    #[inline]
    pub fn never(write: W) -> Self {
        let write = StreamInner::Strip(StripStream::new(write));
        AutoStream { write }
    }
}

impl<W> AutoStream<W>
where
    W: Lockable,
{
    /// Get exclusive access to the `AutoStream`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    #[inline]
    pub fn lock(self) -> <Self as Lockable>::Locked {
        let write = match self.write {
            StreamInner::PassThrough(w) => StreamInner::PassThrough(w.lock()),
            StreamInner::Strip(w) => StreamInner::Strip(w.lock()),
        };
        AutoStream { write }
    }
}

#[cfg(feature = "auto")]
impl<W> AutoStream<W>
where
    W: std::io::Write + is_terminal::IsTerminal,
{
    #[cfg(feature = "auto")]
    #[inline]
    pub(crate) fn auto(write: W) -> Self {
        if write.is_terminal() {
            if concolor_query::windows::enable_ansi_colors().unwrap_or(true) {
                Self::always_ansi_(write)
            } else {
                Self::never(write)
            }
        } else {
            Self::never(write)
        }
    }
}

impl<W> std::io::Write for AutoStream<W>
where
    W: std::io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.write {
            StreamInner::PassThrough(w) => w.write(buf),
            StreamInner::Strip(w) => w.write(buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.write {
            StreamInner::PassThrough(w) => w.flush(),
            StreamInner::Strip(w) => w.flush(),
        }
    }

    // Provide explicit implementations of trait methods
    // - To reduce bookkeeping
    // - Avoid acquiring / releasing locks in a loop

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match &mut self.write {
            StreamInner::PassThrough(w) => w.write_all(buf),
            StreamInner::Strip(w) => w.write_all(buf),
        }
    }

    // Not bothering with `write_fmt` as it just calls `write_all`
}

impl<W> Lockable for AutoStream<W>
where
    W: Lockable,
{
    type Locked = AutoStream<<W as Lockable>::Locked>;

    #[inline]
    fn lock(self) -> Self::Locked {
        self.lock()
    }
}
