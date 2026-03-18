/// Create a URL from a given path
pub fn path_to_url(path: &std::path::Path) -> Option<String> {
    // Do a best-effort for getting the host in the URL
    let hostname = if cfg!(windows) {
        // Not supported correctly on windows
        None
    } else {
        crate::hostname().ok().and_then(|os| os.into_string().ok())
    };
    if path.is_dir() {
        dir_to_url(hostname.as_deref(), path)
    } else {
        file_to_url(hostname.as_deref(), path)
    }
}

/// Create a URL from a given hostname and file path
///
/// For hyperlink escape codes, the hostname is used to avoid issues with opening a link scoped to
/// the computer you've SSH'ed into
/// ([reference](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#file-uris-and-the-hostname))
pub fn file_to_url(hostname: Option<&str>, path: &std::path::Path) -> Option<String> {
    let mut url = "file://".to_owned();
    if let Some(hostname) = hostname {
        url.push_str(hostname);
    }

    encode_path(path, &mut url);

    Some(url)
}

/// Create a URL from a given hostname and directory path
///
/// For hyperlink escape codes, the hostname is used to avoid issues with opening a link scoped to
/// the computer you've SSH'ed into
/// ([reference](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#file-uris-and-the-hostname))
pub fn dir_to_url(hostname: Option<&str>, path: &std::path::Path) -> Option<String> {
    let mut url = file_to_url(hostname, path)?;
    if !url.ends_with(URL_PATH_SEP) {
        url.push_str(URL_PATH_SEP);
    }
    Some(url)
}

const URL_PATH_SEP: &str = "/";

/// <https://url.spec.whatwg.org/#fragment-percent-encode-set>
const FRAGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`');

/// <https://url.spec.whatwg.org/#path-percent-encode-set>
const PATH: &percent_encoding::AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');

const PATH_SEGMENT: &percent_encoding::AsciiSet = &PATH.add(b'/').add(b'%');

// The backslash (\) character is treated as a path separator in special URLs
// so it needs to be additionally escaped in that case.
const SPECIAL_PATH_SEGMENT: &percent_encoding::AsciiSet = &PATH_SEGMENT.add(b'\\');

/// Editor-specific file URLs
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Editor {
    Cursor,
    Grepp,
    Kitty,
    MacVim,
    TextMate,
    VSCode,
    VSCodeInsiders,
    VSCodium,
}

impl Editor {
    /// Iterate over all supported editors.
    pub fn all() -> impl Iterator<Item = Self> {
        [
            Self::Cursor,
            Self::Grepp,
            Self::Kitty,
            Self::MacVim,
            Self::TextMate,
            Self::VSCode,
            Self::VSCodeInsiders,
            Self::VSCodium,
        ]
        .into_iter()
    }

    /// Create an editor-specific file URL
    pub fn to_url(
        &self,
        hostname: Option<&str>,
        file: &std::path::Path,
        line: usize,
        col: usize,
    ) -> Option<String> {
        let mut path = String::new();
        encode_path(file, &mut path);
        let url = match self {
            Self::Cursor => {
                format!("cursor://file{path}:{line}:{col}")
            }
            // https://github.com/misaki-web/grepp?tab=readme-ov-file#scheme-handler
            Self::Grepp => format!("grep+://{path}:{line}"),
            Self::Kitty => format!("file://{}{path}#{line}", hostname.unwrap_or_default()),
            // https://macvim.org/docs/gui_mac.txt.html#mvim%3A%2F%2F
            Self::MacVim => {
                format!("mvim://open?url=file://{path}&line={line}&column={col}")
            }
            // https://macromates.com/blog/2007/the-textmate-url-scheme/
            Self::TextMate => {
                format!("txmt://open?url=file://{path}&line={line}&column={col}")
            }
            // https://code.visualstudio.com/docs/editor/command-line#_opening-vs-code-with-urls
            Self::VSCode => format!("vscode://file{path}:{line}:{col}"),
            Self::VSCodeInsiders => {
                format!("vscode-insiders://file{path}:{line}:{col}")
            }
            Self::VSCodium => format!("vscodium://file{path}:{line}:{col}"),
        };
        Some(url)
    }
}

impl core::fmt::Display for Editor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Cursor => "cursor",
            Self::Grepp => "grepp",
            Self::Kitty => "kitty",
            Self::MacVim => "macvim",
            Self::TextMate => "textmate",
            Self::VSCode => "vscode",
            Self::VSCodeInsiders => "vscode-insiders",
            Self::VSCodium => "vscodium",
        };
        f.write_str(name)
    }
}

impl core::str::FromStr for Editor {
    type Err = ParseEditorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cursor" => Ok(Self::Cursor),
            "grepp" => Ok(Self::Grepp),
            "kitty" => Ok(Self::Kitty),
            "macvim" => Ok(Self::MacVim),
            "textmate" => Ok(Self::TextMate),
            "vscode" => Ok(Self::VSCode),
            "vscode-insiders" => Ok(Self::VSCodeInsiders),
            "vscodium" => Ok(Self::VSCodium),
            _ => Err(ParseEditorError),
        }
    }
}

/// Failed to parse an [`Editor`] from a string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseEditorError;

impl core::fmt::Display for ParseEditorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("unknown editor")
    }
}

fn encode_path(path: &std::path::Path, url: &mut String) {
    let mut is_path_empty = true;

    for component in path.components() {
        is_path_empty = false;
        match component {
            std::path::Component::Prefix(prefix) => {
                url.push_str(URL_PATH_SEP);
                let component = prefix.as_os_str().to_string_lossy();
                url.push_str(&component);
            }
            std::path::Component::RootDir => {}
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                url.push_str(URL_PATH_SEP);
                url.push_str("..");
            }
            std::path::Component::Normal(part) => {
                url.push_str(URL_PATH_SEP);
                let component = part.to_string_lossy();
                url.extend(percent_encoding::percent_encode(
                    component.as_bytes(),
                    SPECIAL_PATH_SEGMENT,
                ));
            }
        }
    }
    if is_path_empty {
        // An URL's path must not be empty
        url.push_str(URL_PATH_SEP);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funky_file_path() {
        let editor_urls = Editor::all()
            .map(|editor| editor.to_url(None, "/tmp/a b#c".as_ref(), 1, 1))
            .map(|editor| editor.unwrap_or_else(|| "-".to_owned()))
            .collect::<Vec<_>>()
            .join("\n");

        snapbox::assert_data_eq!(
            editor_urls,
            snapbox::str![[r#"
cursor://file/tmp/a%20b%23c:1:1
grep+:///tmp/a%20b%23c:1
file:///tmp/a%20b%23c#1
mvim://open?url=file:///tmp/a%20b%23c&line=1&column=1
txmt://open?url=file:///tmp/a%20b%23c&line=1&column=1
vscode://file/tmp/a%20b%23c:1:1
vscode-insiders://file/tmp/a%20b%23c:1:1
vscodium://file/tmp/a%20b%23c:1:1
"#]]
        );
    }

    #[test]
    fn with_hostname() {
        let editor_urls = Editor::all()
            .map(|editor| editor.to_url(Some("localhost"), "/home/foo/file.txt".as_ref(), 1, 1))
            .map(|editor| editor.unwrap_or_else(|| "-".to_owned()))
            .collect::<Vec<_>>()
            .join("\n");

        snapbox::assert_data_eq!(
            editor_urls,
            snapbox::str![[r#"
cursor://file/home/foo/file.txt:1:1
grep+:///home/foo/file.txt:1
file://localhost/home/foo/file.txt#1
mvim://open?url=file:///home/foo/file.txt&line=1&column=1
txmt://open?url=file:///home/foo/file.txt&line=1&column=1
vscode://file/home/foo/file.txt:1:1
vscode-insiders://file/home/foo/file.txt:1:1
vscodium://file/home/foo/file.txt:1:1
"#]]
        );
    }

    #[test]
    #[cfg(windows)]
    fn windows_file_path() {
        let editor_urls = Editor::all()
            .map(|editor| editor.to_url(None, "C:\\Users\\foo\\help.txt".as_ref(), 1, 1))
            .map(|editor| editor.unwrap_or_else(|| "-".to_owned()))
            .collect::<Vec<_>>()
            .join("\n");

        snapbox::assert_data_eq!(
            editor_urls,
            snapbox::str![[r#"
cursor://file/C:/Users/foo/help.txt:1:1
grep+:///C:/Users/foo/help.txt:1
file:///C:/Users/foo/help.txt#1
mvim://open?url=file:///C:/Users/foo/help.txt&line=1&column=1
txmt://open?url=file:///C:/Users/foo/help.txt&line=1&column=1
vscode://file/C:/Users/foo/help.txt:1:1
vscode-insiders://file/C:/Users/foo/help.txt:1:1
vscodium://file/C:/Users/foo/help.txt:1:1
"#]]
        );
    }

    #[test]
    fn editor_strings_round_trip() {
        let editors = Editor::all().collect::<Vec<_>>();
        let parsed = editors
            .iter()
            .map(|editor| editor.to_string().parse())
            .collect::<Result<Vec<_>, _>>();

        assert_eq!(parsed, Ok(editors));
    }

    #[test]
    fn invalid_editor_string_errors() {
        assert_eq!("code".parse::<Editor>(), Err(ParseEditorError));
    }
}
