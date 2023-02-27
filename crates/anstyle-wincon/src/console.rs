pub struct Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    stream: S,
    initial_fg: anstyle::AnsiColor,
    initial_bg: anstyle::AnsiColor,
    last_fg: anstyle::AnsiColor,
    last_bg: anstyle::AnsiColor,
}

impl<S> Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    pub fn new(stream: S) -> std::io::Result<Self> {
        Self::from_stream(stream)
    }

    fn from_stream(stream: S) -> std::io::Result<Self> {
        let (initial_fg, initial_bg) = stream.get_colors()?;
        Ok(Self {
            stream,
            initial_fg,
            initial_bg,
            last_fg: initial_fg,
            last_bg: initial_bg,
        })
    }

    pub fn write(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        self.apply(fg.unwrap_or(self.initial_fg), bg.unwrap_or(self.initial_bg))?;
        let written = self.stream.write(data)?;
        Ok(written)
    }

    pub fn reset(&mut self) -> std::io::Result<()> {
        self.apply(self.initial_fg, self.initial_bg)
    }

    pub fn close(mut self) -> std::io::Result<()> {
        self.reset()
    }

    fn apply(&mut self, fg: anstyle::AnsiColor, bg: anstyle::AnsiColor) -> std::io::Result<()> {
        if fg == self.last_fg && bg == self.last_bg {
            return Ok(());
        }

        // Ensure everything is written with the last set of colors before applying the next set
        self.stream.flush()?;

        self.stream.set_colors(fg, bg)?;
        self.last_fg = fg;
        self.last_bg = bg;

        Ok(())
    }
}

impl<S> Drop for Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    fn drop(&mut self) {
        let _ = self.reset();
    }
}
