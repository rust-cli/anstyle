/// Terminal progress formatter
///
/// # Example
///
/// ```rust
/// # use anstyle_progress::TermProgress;
/// # use anstyle_progress::TermProgressStatus;
/// let mut progress = anstyle_progress::TermProgress::start();
///
/// let progress = progress.percent(0);
/// println!("{progress}");
///
/// let progress = progress.percent(50);
/// println!("{progress}");
///
/// let progress = progress.percent(100);
/// println!("{progress}");
///
/// let progress = progress.remove();
/// println!("{progress}");
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

    /// Start an error indicator
    pub fn error() -> Self {
        Self::none().status(TermProgressStatus::Error)
    }

    /// Remove the indicator
    pub fn remove(mut self) -> Self {
        self.status = Some(TermProgressStatus::Removed);
        self.percent = None;
        self
    }

    /// Change the reported status
    pub fn status(mut self, status: TermProgressStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set progress percentage (between `0..=100`)
    ///
    /// Without setting this, progress will be indeterminate
    pub fn percent(mut self, percent: u8) -> Self {
        assert!(matches!(percent, 0..=100));
        self.percent = Some(percent);
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
    Removed,
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
        let (st, pr) = match (status, self.percent) {
            (TermProgressStatus::Removed, _) => (0, None),
            (TermProgressStatus::Normal, Some(_)) => (1, self.percent),
            (TermProgressStatus::Error, _) => (2, self.percent),
            (TermProgressStatus::Normal, None) => (3, None),
            (TermProgressStatus::Paused, _) => (4, self.percent),
        };
        write!(f, "\x1b]9;4;{st}")?;
        if let Some(pr) = pr {
            write!(f, ";{pr}")?;
        }
        write!(f, "\x1b\\")
    }
}
