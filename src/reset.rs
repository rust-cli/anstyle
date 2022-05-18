/// Reset terminal formatting
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reset;

impl Reset {
    /// Render the ANSI code
    pub fn render(self) -> impl std::fmt::Display {
        ResetDisplay
    }
}

struct ResetDisplay;

impl std::fmt::Display for ResetDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[0m")
    }
}
