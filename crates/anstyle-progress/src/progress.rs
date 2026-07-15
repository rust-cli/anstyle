/// Terminal progress formatter
///
/// # Example
///
/// ```rust
/// # use anstyle_progress::TermProgress;
/// # use anstyle_progress::TermProgressStatus;
/// let mut progress = TermProgress::start();
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
/// let progress = TermProgress::remove();
/// println!("{progress}");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct TermProgress {
    status: Option<TermProgressStatus>,
    percent: Option<u8>,
}

impl TermProgress {
    /// No progress to display
    pub const fn none() -> Self {
        Self {
            status: None,
            percent: None,
        }
    }

    /// Start a progress indicator
    ///
    /// This starts in an indeterminate state
    pub const fn start() -> Self {
        Self::none().status(TermProgressStatus::Normal)
    }

    /// Start an error indicator
    pub const fn error() -> Self {
        Self::none().status(TermProgressStatus::Error)
    }

    /// Remove the indicator
    pub const fn remove() -> Self {
        Self::none().status(TermProgressStatus::Removed)
    }

    /// Set progress percentage (between `0..=100`)
    ///
    /// Without setting this, progress will be indeterminate
    pub const fn percent(mut self, percent: u8) -> Self {
        assert!(matches!(percent, 0..=100));
        self.percent = Some(percent);
        self
    }

    /// Change the reported status
    pub const fn status(mut self, status: TermProgressStatus) -> Self {
        self.status = Some(status);
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
#[derive(Copy, Clone, Debug)]
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
        write!(f, "\x1b]9;4;{st};")?;
        if let Some(pr) = pr {
            write!(f, "{pr}")?;
        }
        write!(f, "\x1b\\")
    }
}
