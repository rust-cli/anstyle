#[cfg(not(feature = "auto"))]
pub trait RawStream: std::io::Write + private::Sealed {}

#[cfg(feature = "auto")]
pub trait RawStream: std::io::Write + is_terminal::IsTerminal + private::Sealed {}

impl RawStream for std::io::Stdout {}

impl RawStream for std::io::StdoutLock<'static> {}

impl RawStream for std::io::Stderr {}

impl RawStream for std::io::StderrLock<'static> {}

mod private {
    pub trait Sealed {}

    impl Sealed for std::io::Stdout {}

    impl Sealed for std::io::StdoutLock<'static> {}

    impl Sealed for std::io::Stderr {}

    impl Sealed for std::io::StderrLock<'static> {}
}
