use crate::Lockable;
use crate::RawStream;
use crate::StripStream;

/// [`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities
pub struct AutoStream<S> {
    inner: StreamInner<S>,
}

enum StreamInner<S> {
    PassThrough(S),
    Strip(StripStream<S>),
}

impl<S> AutoStream<S>
where
    S: RawStream,
{
    /// Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`
    /// supports.
    #[inline]
    pub fn always_ansi(raw: S) -> Self {
        #[cfg(feature = "auto")]
        {
            if raw.is_terminal() {
                let _ = concolor_query::windows::enable_ansi_colors();
            }
        }
        Self::always_ansi_(raw)
    }

    fn always_ansi_(raw: S) -> Self {
        let inner = StreamInner::PassThrough(raw);
        AutoStream { inner }
    }

    /// Only pass printable data to the inner `Write`.
    #[inline]
    pub fn never(raw: S) -> Self {
        let inner = StreamInner::Strip(StripStream::new(raw));
        AutoStream { inner }
    }
}

impl<S> AutoStream<S>
where
    S: Lockable,
{
    /// Get exclusive access to the `AutoStream`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    #[inline]
    pub fn lock(self) -> <Self as Lockable>::Locked {
        let inner = match self.inner {
            StreamInner::PassThrough(w) => StreamInner::PassThrough(w.lock()),
            StreamInner::Strip(w) => StreamInner::Strip(w.lock()),
        };
        AutoStream { inner }
    }
}

#[cfg(feature = "auto")]
impl<S> AutoStream<S>
where
    S: RawStream,
{
    #[cfg(feature = "auto")]
    #[inline]
    pub(crate) fn auto(raw: S) -> Self {
        if raw.is_terminal() {
            if concolor_query::windows::enable_ansi_colors().unwrap_or(true) {
                Self::always_ansi_(raw)
            } else {
                Self::never(raw)
            }
        } else {
            Self::never(raw)
        }
    }
}

impl<S> std::io::Write for AutoStream<S>
where
    S: RawStream,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.inner {
            StreamInner::PassThrough(w) => w.write(buf),
            StreamInner::Strip(w) => w.write(buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.inner {
            StreamInner::PassThrough(w) => w.flush(),
            StreamInner::Strip(w) => w.flush(),
        }
    }

    // Provide explicit implementations of trait methods
    // - To reduce bookkeeping
    // - Avoid acquiring / releasing locks in a loop

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match &mut self.inner {
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
