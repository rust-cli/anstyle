//! Higher-level traits to describe writeable streams

/// Required functionality for underlying [`std::io::Write`] for adaptation
#[cfg(not(all(windows, feature = "wincon")))]
pub trait RawStream: std::io::Write + IsTerminal + private::Sealed {}

/// Required functionality for underlying [`std::io::Write`] for adaptation
#[cfg(all(windows, feature = "wincon"))]
pub trait RawStream:
    std::io::Write + IsTerminal + anstyle_wincon::WinconStream + private::Sealed
{
}

impl RawStream for std::io::Stdout {}

impl RawStream for std::io::StdoutLock<'_> {}

impl RawStream for std::io::Stderr {}

impl RawStream for std::io::StderrLock<'_> {}

impl RawStream for std::fs::File {}

impl RawStream for crate::Buffer {}

pub trait IsTerminal: private::Sealed {
    fn is_terminal(&self) -> bool;
}

impl IsTerminal for std::io::Stdout {
    #[inline]
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}

impl IsTerminal for std::io::StdoutLock<'_> {
    #[inline]
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}

impl IsTerminal for std::io::Stderr {
    #[inline]
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}

impl IsTerminal for std::io::StderrLock<'_> {
    #[inline]
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}

impl IsTerminal for std::fs::File {
    #[inline]
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}

impl IsTerminal for crate::Buffer {
    #[inline]
    fn is_terminal(&self) -> bool {
        false
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for std::io::Stdout {}

    impl Sealed for std::io::StdoutLock<'_> {}

    impl Sealed for std::io::Stderr {}

    impl Sealed for std::io::StderrLock<'_> {}

    impl Sealed for std::fs::File {}

    impl Sealed for crate::Buffer {}
}
