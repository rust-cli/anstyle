/// Terminal progress formatter
///
/// # Example
///
/// ```rust
/// # use anstyle_progress::TermProgress;
/// # use anstyle_progress::TermProgressStatus;
/// let mut progress = anstyle_progress::TermProgress::none()
///   .status(TermProgressStatus::Normal);
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
        let st = match (f.alternate(), status, self.percent) {
            (true, _, _) => 0,
            (false, TermProgressStatus::Normal, Some(_)) => 1,
            (false, TermProgressStatus::Error, _) => 2,
            (false, TermProgressStatus::Normal, None) => 3,
            (false, TermProgressStatus::Paused, _) => 4,
        };
        let pr = self.percent.unwrap_or(0);
        write!(f, "\x1b]9;4;{st};{pr}\x1b\\")
    }
}
