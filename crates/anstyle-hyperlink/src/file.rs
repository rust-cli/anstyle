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

fn encode_path(path: &std::path::Path, url: &mut String) {
    let mut is_path_empty = true;

    for component in path.components() {
        is_path_empty = false;
        match component {
            std::path::Component::Prefix(prefix) => {
                let component = prefix.as_os_str().to_string_lossy();
                url.push_str(&component);
            }
            std::path::Component::RootDir => {}
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
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
