//! **Auto-adapting [`stdout`] / [`stderr`] streams**
//!
//! [`AutoStream`] always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
//! adapting to the user's terminal's capabilities.
//!
//! Benefits
//! - Allows the caller to not be concerned with the terminal's capabilities
//! - Semver safe way of passing styled text between crates as ANSI escape codes offer more
//!   compatibility than most crate APIs.
//!
//! # Example
//!
//! ```
//! #  #[cfg(feature = "auto")] {
//! use anstyle_stream::println;
//! use owo_colors::OwoColorize as _;
//!
//! // Foreground colors
//! println!("My number is {:#x}!", 10.green());
//! // Background colors
//! println!("My number is not {}!", 4.on_red());
//! # }
//! ```
//!
//! And this will correctly handle piping to a file, etc

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod adapter;
#[macro_use]
mod macros;

/// Create an ANSI escape code compatible stdout
///
/// **Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing
/// from the implicit locking in each [`std::io::Write`] call
#[cfg(feature = "auto")]
pub fn stdout() -> AutoStream<std::io::Stdout> {
    let stdout = std::io::stdout();
    AutoStream::auto(stdout)
}

/// Create an ANSI escape code compatible stderr
///
/// **Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing
/// from the implicit locking in each [`std::io::Write`] call
#[cfg(feature = "auto")]
pub fn stderr() -> AutoStream<std::io::Stderr> {
    let stderr = std::io::stderr();
    AutoStream::auto(stderr)
}

/// Explicitly lock a [`std::io::Write`]able
pub trait Lockable {
    type Locked;

    /// Get exclusive access to the `AutoStream`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    fn lock(self) -> Self::Locked;
}

impl Lockable for std::io::Stdout {
    type Locked = std::io::StdoutLock<'static>;

    #[inline]
    fn lock(self) -> Self::Locked {
        #[allow(clippy::needless_borrow)] // Its needed to avoid recursion
        (&self).lock()
    }
}

impl Lockable for std::io::Stderr {
    type Locked = std::io::StderrLock<'static>;

    #[inline]
    fn lock(self) -> Self::Locked {
        #[allow(clippy::needless_borrow)] // Its needed to avoid recursion
        (&self).lock()
    }
}

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
    fn auto(write: W) -> Self {
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

/// Only pass printable data to the inner `Write`
pub struct StripStream<W> {
    write: W,
    state: adapter::StripBytes,
}

impl<W> StripStream<W>
where
    W: std::io::Write,
{
    /// Only pass printable data to the inner `Write`
    #[inline]
    pub fn new(write: W) -> Self {
        Self {
            write,
            state: Default::default(),
        }
    }
}

impl<W> std::io::Write for StripStream<W>
where
    W: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let initial_state = self.state.clone();

        let mut written = 0;
        let mut possible = 0;
        for printable in self.state.strip_next(buf) {
            possible += printable.len();
            written += self.write.write(printable)?;
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
        self.write.flush()
    }

    // Provide explicit implementations of trait methods
    // - To reduce bookkeeping
    // - Avoid acquiring / releasing locks in a loop

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        for printable in self.state.strip_next(buf) {
            self.write.write_all(printable)?;
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

impl<W> Lockable for StripStream<W>
where
    W: Lockable,
{
    type Locked = StripStream<<W as Lockable>::Locked>;

    #[inline]
    fn lock(self) -> Self::Locked {
        Self::Locked {
            write: self.write.lock(),
            state: self.state,
        }
    }
}
