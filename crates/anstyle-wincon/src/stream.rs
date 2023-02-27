#[cfg(windows)]
use std::os::windows::io::{AsHandle, AsRawHandle};

#[cfg(windows)]
use crate::windows::*;

/// Extend `std::io::Write` with wincon styling
///
/// Generally, you will want to use [`Console`][crate::Console] instead
pub trait WinconStream {
    /// Change the foreground/background
    ///
    /// A common pitfall is to forget to flush writes to
    /// stdout before setting new text attributes.
    fn set_colors(&mut self, fg: anstyle::AnsiColor, bg: anstyle::AnsiColor)
        -> std::io::Result<()>;
    fn get_colors(&self) -> std::io::Result<(anstyle::AnsiColor, anstyle::AnsiColor)>;
}

#[cfg(windows)]
impl WinconStream for std::io::Stdout {
    fn set_colors(
        &mut self,
        fg: anstyle::AnsiColor,
        bg: anstyle::AnsiColor,
    ) -> std::io::Result<()> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let attributes = set_colors(fg, bg);
        set_console_text_attributes(handle, attributes)
    }

    fn get_colors(&self) -> std::io::Result<(anstyle::AnsiColor, anstyle::AnsiColor)> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let info = get_screen_buffer_info(handle)?;
        Ok(get_colors(&info))
    }
}

#[cfg(windows)]
impl<'s> WinconStream for std::io::StdoutLock<'s> {
    fn set_colors(
        &mut self,
        fg: anstyle::AnsiColor,
        bg: anstyle::AnsiColor,
    ) -> std::io::Result<()> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let attributes = set_colors(fg, bg);
        set_console_text_attributes(handle, attributes)
    }

    fn get_colors(&self) -> std::io::Result<(anstyle::AnsiColor, anstyle::AnsiColor)> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let info = get_screen_buffer_info(handle)?;
        Ok(get_colors(&info))
    }
}

#[cfg(windows)]
impl WinconStream for std::io::Stderr {
    fn set_colors(
        &mut self,
        fg: anstyle::AnsiColor,
        bg: anstyle::AnsiColor,
    ) -> std::io::Result<()> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let attributes = set_colors(fg, bg);
        set_console_text_attributes(handle, attributes)
    }

    fn get_colors(&self) -> std::io::Result<(anstyle::AnsiColor, anstyle::AnsiColor)> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let info = get_screen_buffer_info(handle)?;
        Ok(get_colors(&info))
    }
}

#[cfg(windows)]
impl<'s> WinconStream for std::io::StderrLock<'s> {
    fn set_colors(
        &mut self,
        fg: anstyle::AnsiColor,
        bg: anstyle::AnsiColor,
    ) -> std::io::Result<()> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let attributes = set_colors(fg, bg);
        set_console_text_attributes(handle, attributes)
    }

    fn get_colors(&self) -> std::io::Result<(anstyle::AnsiColor, anstyle::AnsiColor)> {
        let handle = self.as_handle();
        let handle = handle.as_raw_handle();
        let info = get_screen_buffer_info(handle)?;
        Ok(get_colors(&info))
    }
}
