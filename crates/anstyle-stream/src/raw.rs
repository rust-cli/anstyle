#[cfg(not(any(feature = "auto", feature = "wincon")))]
pub trait RawStream: std::io::Write + private::Sealed {}

#[cfg(all(feature = "auto", not(feature = "wincon")))]
pub trait RawStream: std::io::Write + is_terminal::IsTerminal + private::Sealed {}

#[cfg(all(not(feature = "auto"), feature = "wincon"))]
pub trait RawStream: std::io::Write + anstyle_wincon::WinconStream + private::Sealed {}

#[cfg(all(feature = "auto", feature = "wincon"))]
pub trait RawStream:
    std::io::Write + is_terminal::IsTerminal + anstyle_wincon::WinconStream + private::Sealed
{
}

impl RawStream for std::io::Stdout {}

impl RawStream for std::io::StdoutLock<'static> {}

impl RawStream for std::io::Stderr {}

impl RawStream for std::io::StderrLock<'static> {}

impl RawStream for crate::Buffer {}

mod private {
    pub trait Sealed {}

    impl Sealed for std::io::Stdout {}

    impl Sealed for std::io::StdoutLock<'static> {}

    impl Sealed for std::io::Stderr {}

    impl Sealed for std::io::StderrLock<'static> {}

    impl Sealed for crate::Buffer {}
}
