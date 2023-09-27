/// Write colored text to the screen
#[derive(Debug)]
pub struct Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    stream: Option<S>,
    state: crate::stream::ConsoleState,
}

impl<S> Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    pub fn new(stream: S) -> Result<Self, S> {
        match crate::stream::ConsoleState::new(&stream) {
            Ok(state) => Ok(Self {
                stream: Some(stream),
                state,
            }),
            Err(_err) => Err(stream),
        }
    }

    /// Write colored text to the screen
    pub fn write(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
        data: &[u8],
    ) -> std::io::Result<usize> {
        self.state
            .write(self.stream.as_mut().unwrap(), fg, bg, data)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stream.as_mut().unwrap().flush()
    }

    /// Change the terminal back to the initial colors
    pub fn reset(&mut self) -> std::io::Result<()> {
        self.state.reset(self.stream.as_mut().unwrap())
    }

    /// Close the stream, reporting any errors
    pub fn close(mut self) -> std::io::Result<()> {
        self.reset()
    }

    /// Get the inner writer
    #[inline]
    pub fn into_inner(mut self) -> S {
        let _ = self.reset();
        self.stream.take().unwrap()
    }
}

impl<S> Console<S>
where
    S: crate::WinconStream + std::io::Write + std::io::IsTerminal,
{
    pub fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self.stream.as_ref().unwrap())
    }
}

impl<S> Drop for Console<S>
where
    S: crate::WinconStream + std::io::Write,
{
    fn drop(&mut self) {
        // Otherwise `Console::lock` took it
        if self.stream.is_some() {
            let _ = self.reset();
        }
    }
}

impl<S> Console<S>
where
    S: crate::WinconStream + std::io::Write,
    S: crate::Lockable,
    <S as crate::Lockable>::Locked: crate::WinconStream + std::io::Write,
{
    /// Get exclusive access to the `Console`
    ///
    /// Why?
    /// - Faster performance when writing in a loop
    /// - Avoid other threads interleaving output with the current thread
    #[inline]
    pub fn lock(mut self) -> <Self as crate::Lockable>::Locked {
        Console {
            stream: Some(self.stream.take().unwrap().lock()),
            state: self.state.clone(),
        }
    }
}

impl<S> crate::Lockable for Console<S>
where
    S: crate::WinconStream + std::io::Write,
    S: crate::Lockable,
    <S as crate::Lockable>::Locked: crate::WinconStream + std::io::Write,
{
    type Locked = Console<<S as crate::Lockable>::Locked>;

    #[inline]
    fn lock(self) -> Self::Locked {
        self.lock()
    }
}
