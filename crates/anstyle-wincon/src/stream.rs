/// Extend `std::io::Write` with wincon styling
pub trait WinconStream {
    /// Write colored text to the stream
    fn write_colored(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize>;
}

impl<T: WinconStream + ?Sized> WinconStream for &mut T {
    #[inline]
    fn write_colored(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        T::write_colored(&mut **self, fg, bg, data)
    }
}

impl<T: WinconStream + ?Sized> WinconStream for Box<T> {
    #[inline]
    fn write_colored(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        T::write_colored(&mut **self, fg, bg, data)
    }
}

impl WinconStream for std::io::Stdout {
    #[inline]
    fn write_colored(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        // Ensure exclusive access
        self.lock().write_colored(fg, bg, data)
    }
}

impl WinconStream for std::io::Stderr {
    #[inline]
    fn write_colored(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        // Ensure exclusive access
        self.lock().write_colored(fg, bg, data)
    }
}

#[cfg(not(windows))]
mod platform {
    impl super::WinconStream for std::io::StdoutLock<'_> {
        fn write_colored(
            &mut self,
            fg: Option<anstyle::AnsiColor>,
            bg: Option<anstyle::AnsiColor>,
            data: &[u8],
        ) -> std::io::Result<usize> {
            crate::ansi::write_colored(self, fg, bg, data)
        }
    }

    impl super::WinconStream for std::io::StderrLock<'_> {
        fn write_colored(
            &mut self,
            fg: Option<anstyle::AnsiColor>,
            bg: Option<anstyle::AnsiColor>,
            data: &[u8],
        ) -> std::io::Result<usize> {
            crate::ansi::write_colored(self, fg, bg, data)
        }
    }
}

#[cfg(windows)]
mod platform {
    impl super::WinconStream for std::io::StdoutLock<'_> {
        fn write_colored(
            &mut self,
            fg: Option<anstyle::AnsiColor>,
            bg: Option<anstyle::AnsiColor>,
            data: &[u8],
        ) -> std::io::Result<usize> {
            let initial = crate::windows::stdout_initial_colors();
            crate::windows::write_colored(self, fg, bg, data, initial)
        }
    }

    impl super::WinconStream for std::io::StderrLock<'_> {
        fn write_colored(
            &mut self,
            fg: Option<anstyle::AnsiColor>,
            bg: Option<anstyle::AnsiColor>,
            data: &[u8],
        ) -> std::io::Result<usize> {
            let initial = crate::windows::stderr_initial_colors();
            crate::windows::write_colored(self, fg, bg, data, initial)
        }
    }
}

macro_rules! impl_default_wincon_stream {
    ($( $(#[$attr:meta])* $t:ty),* $(,)?) => {
        $(
            $(#[$attr])*
            impl WinconStream for $t {
                #[inline]
                fn write_colored(
                    &mut self,
                    fg: Option<anstyle::AnsiColor>,
                    bg: Option<anstyle::AnsiColor>,
                    data: &[u8],
                ) -> std::io::Result<usize> {
                    crate::ansi::write_colored(self, fg, bg, data)
                }
            }
        )*
    };
}

impl_default_wincon_stream!(
    dyn std::io::Write,
    dyn std::io::Write + Send,
    dyn std::io::Write + Send + Sync,
    Vec<u8>,
    std::fs::File,
);
