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
pub(crate) enum ConsoleState {
    Wincon(StdioAdapter),
    Pass(PassThroughAdapter),
}

impl ConsoleState {
    pub(crate) fn new<S: crate::WinconStream + std::io::Write>(
        stream: &S,
    ) -> std::io::Result<Self> {
        let adapter = match stream.get_colors() {
            Ok((Some(initial_fg), Some(initial_bg))) => {
                Self::Wincon(StdioAdapter::with_initial(initial_fg, initial_bg))
            }
            // Can only happen on non-wincon systems
            Ok(_) => Self::Pass(PassThroughAdapter::new()),
            Err(err) => {
                return Err(err);
            }
        };
        Ok(adapter)
    }

    pub(crate) fn write<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        let non_default = fg.is_some() || bg.is_some();

        if non_default {
            self.apply(stream, fg, bg)?;
        }
        let written = stream.write(data)?;
        if non_default {
            self.apply(stream, None, None)?;
        }
        Ok(written)
    }

    pub(crate) fn reset<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
    ) -> std::io::Result<()> {
        self.apply(stream, None, None)
    }

    fn apply<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        match self {
            Self::Wincon(adapter) => adapter.apply(stream, fg, bg),
            Self::Pass(adapter) => adapter.apply(stream, fg, bg),
        }
    }
}

#[derive(Default, Clone, Debug)]
#[non_exhaustive]
pub(crate) struct PassThroughAdapter {}

impl PassThroughAdapter {
    fn new() -> Self {
        Self {}
    }

    fn with_initial(_initial_fg: anstyle::AnsiColor, _initial_bg: anstyle::AnsiColor) -> Self {
        // Should be fine to ignore the initial as that only happens on windows when this shouldn't
        // actually be called
        Self::new()
    }

    fn apply<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        stream.set_colors(fg, bg)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WinconAdapter {
    initial_fg: anstyle::AnsiColor,
    initial_bg: anstyle::AnsiColor,
}

impl WinconAdapter {
    fn with_initial(initial_fg: anstyle::AnsiColor, initial_bg: anstyle::AnsiColor) -> Self {
        Self {
            initial_fg,
            initial_bg,
        }
    }

    fn apply<S: crate::WinconStream + std::io::Write>(
        &mut self,
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        let fg = fg.unwrap_or(self.initial_fg);
        let bg = bg.unwrap_or(self.initial_bg);

        // Ensure everything is written with the last set of colors before applying the next set
        stream.flush()?;
        stream.set_colors(Some(fg), Some(bg))?;

        Ok(())
    }
}

#[cfg(not(windows))]
use PassThroughAdapter as StdioAdapter;
#[cfg(windows)]
use WinconAdapter as StdioAdapter;

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
        // No idea what state the stream was left in, so just assume default
        Ok((None, None))
    }
}
