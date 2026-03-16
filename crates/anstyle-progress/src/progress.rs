/// Terminal progress formatter
///
/// # Example
///
/// ```rust
/// # use anstyle_progress::TermProgress;
/// # use anstyle_progress::TermProgressStatus;
/// let mut progress = anstyle_progress::TermProgress::start();
///
/// let progress = progress.percent(Some(0));
/// println!("{progress}");
///
/// let progress = progress.percent(Some(50));
/// println!("{progress}");
///
/// let progress = progress.percent(Some(100));
/// println!("{progress}");
///
/// println!("{progress:#}");
/// ```
#[derive(Copy, Clone)]
pub struct TermProgress {
    status: Option<TermProgressStatus>,
    percent: Option<u8>,
}

impl TermProgress {
    /// No progress to display
    pub fn none() -> Self {
        Self {
            status: None,
            percent: None,
        }
    }

    /// Start a progress indicator
    ///
    /// This starts in an indeterminate state
    pub fn start() -> Self {
        Self::none().status(TermProgressStatus::Normal)
    }

    /// Change the reported status
    pub fn status(mut self, status: TermProgressStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Between `0..=100`
    ///
    /// When `None`, will report an indeterminate status
    pub fn percent(mut self, percent: Option<u8>) -> Self {
        assert!(matches!(percent, Some(0..=100) | None));
        self.percent = percent;
        self
    }
}

impl Default for TermProgress {
    fn default() -> Self {
        Self::none()
    }
}

/// Reported status along with progress
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum TermProgressStatus {
    Normal,
    /// Some terminals treat this as a Warning
    Paused,
    Error,
}

impl core::fmt::Display for TermProgress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Some(status) = self.status else {
            return Ok(());
        };
        let (st, pr) = match (f.alternate(), status, self.percent) {
            (true, _, _) => (0, None),
            (false, TermProgressStatus::Normal, Some(_)) => (1, self.percent),
            (false, TermProgressStatus::Error, _) => (2, self.percent),
            (false, TermProgressStatus::Normal, None) => (3, None),
            (false, TermProgressStatus::Paused, _) => (4, self.percent),
        };
        write!(f, "\x1b]9;4;{st}")?;
        if let Some(pr) = pr {
            write!(f, ";{pr}")?;
        }
        write!(f, "\x1b\\")
    }
}
