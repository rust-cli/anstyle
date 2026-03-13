/// Hyperlink formatter
///
/// # Example
///
/// ```
/// let link = anstyle_hyperlink::Hyperlink::with_url("https://docs.rs/anstyle/latest/anstyle/");
/// format!("Go to {link}anstyle's documentation{link:#}!");
/// ```
pub struct Hyperlink<D: core::fmt::Display> {
    url: Option<D>,
}

impl<D: core::fmt::Display> Hyperlink<D> {
    /// Directly create a hyperlink for a URL
    ///
    /// # Example
    ///
    /// ```
    /// let link = anstyle_hyperlink::Hyperlink::with_url("https://docs.rs/anstyle/latest/anstyle/");
    /// format!("Go to {link}anstyle's documentation{link:#}!");
    /// ```
    pub fn with_url(url: D) -> Self {
        Self { url: Some(url) }
    }
}

#[cfg(feature = "std")]
impl Hyperlink<String> {
    /// Create a hyperlink for a path
    ///
    /// # Example
    ///
    /// ```
    /// let path = std::env::current_dir().unwrap();
    /// let link = anstyle_hyperlink::Hyperlink::with_path(&path);
    /// format!("Go to {link}CWD{link:#}!");
    /// ```
    #[cfg(feature = "file")]
    pub fn with_path(path: &std::path::Path) -> Self {
        let url = crate::path_to_url(path);
        Self { url }
    }
}

impl<D: core::fmt::Display> Default for Hyperlink<D> {
    fn default() -> Self {
        Self { url: None }
    }
}

impl<D: core::fmt::Display> core::fmt::Display for Hyperlink<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Some(url) = self.url.as_ref() else {
            return Ok(());
        };
        if f.alternate() {
            write!(f, "\x1B]8;;\x1B\\")
        } else {
            write!(f, "\x1B]8;;{url}\x1B\\")
        }
    }
}
