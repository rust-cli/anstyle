#[cfg(not(windows))]
use ansi as inner;
#[cfg(windows)]
use wincon as inner;

/// Extend `std::io::Write` with wincon styling
///
/// Generally, you will want to use [`Console`][crate::Console] instead
pub trait WinconStream {
    /// Change the foreground/background
    ///
    /// A common pitfall is to forget to flush writes to
    /// stdout before setting new text attributes.
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()>;

    /// Get the current foreground/background colors
    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)>;
}

impl WinconStream for std::io::Stdout {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

impl WinconStream for std::io::StdoutLock<'static> {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

impl WinconStream for std::io::Stderr {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

impl WinconStream for std::io::StderrLock<'static> {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

impl WinconStream for std::fs::File {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        ansi::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        ansi::get_colors(self)
    }
}

/// Write colored text to the screen
#[derive(Clone, Debug)]
pub(crate) struct ConsoleState {
    initial_fg: Option<anstyle::AnsiColor>,
    initial_bg: Option<anstyle::AnsiColor>,
    last_fg: Option<anstyle::AnsiColor>,
    last_bg: Option<anstyle::AnsiColor>,
}

impl ConsoleState {
    pub(crate) fn new<S: crate::WinconStream + std::io::Write>(
        stream: &S,
    ) -> std::io::Result<Self> {
        let (initial_fg, initial_bg) = match stream.get_colors() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(err);
            }
        };
        Ok(Self {
            initial_fg,
            initial_bg,
            last_fg: initial_fg,
            last_bg: initial_bg,
        })
    }

    pub(crate) fn write<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        self.apply(stream, fg, bg)?;
        let written = stream.write(data)?;
        Ok(written)
    }

    pub(crate) fn reset<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> std::io::Result<()> {
        self.apply(stream, self.initial_fg, self.initial_bg)
    }

    fn apply<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        let fg = fg.or(self.initial_fg);
        let bg = bg.or(self.initial_bg);
        if fg == self.last_fg && bg == self.last_bg {
            return Ok(());
        }

        // Ensure everything is written with the last set of colors before applying the next set
        stream.flush()?;

        stream.set_colors(fg, bg)?;
        self.last_fg = fg;
        self.last_bg = bg;

        Ok(())
    }
}

#[cfg(windows)]
mod wincon {
    use std::os::windows::io::AsHandle;

    pub(super) fn set_colors<S: AsHandle>(
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        if let (Some(fg), Some(bg)) = (fg, bg) {
            crate::windows::set_colors(stream, fg, bg)
        } else {
            Ok(())
        }
    }

    pub(super) fn get_colors<S: AsHandle>(
        stream: &S,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        crate::windows::get_colors(stream).map(|(fg, bg)| (Some(fg), Some(bg)))
    }
}

mod ansi {
    pub(super) fn set_colors<S: std::io::Write>(
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        if let Some(fg) = fg {
            write!(stream, "{}", fg.render_fg())?;
        }
        if let Some(bg) = bg {
            write!(stream, "{}", bg.render_bg())?;
        }
        if fg.is_none() && bg.is_none() {
            write!(stream, "{}", anstyle::Reset.render())?;
        }
        Ok(())
    }

    pub(super) fn get_colors<S>(
        _stream: &S,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        Ok((None, None))
    }
}
