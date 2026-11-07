/// Determines whether the terminal supports ANSI OSC 9;4.
pub fn supports_term_progress(is_terminal: bool) -> bool {
    if !is_terminal {
        return false;
    }

    // Terminal feature detection, includes
    // - iTerm support in v3.6.6
    let term_features = std::env::var("TERM_FEATURES")
        .ok()
        .map(|v| term_features_has_progress(&v))
        .unwrap_or(false);
    if term_features {
        return true;
    }

    // https://github.com/wezterm/wezterm/issues/6581
    // https://ghostty.org/docs/install/release-notes/1-2-0#graphical-progress-bars
    // cmux also identifies as Ghostty through TERM_PROGRAM.
    let term_program = std::env::var("TERM_PROGRAM").ok();
    if matches!(term_program.as_deref(), Some("WezTerm") | Some("ghostty")) {
        return true;
    }

    // Ghostty is detected through TERM_PROGRAM locally and over SSH when the
    // client requests forwarding and the server's AcceptEnv permits it.
    // SSH also carries TERM in PTY requests independently of AcceptEnv, so
    // xterm-ghostty identifies sessions without TERM_PROGRAM forwarding.
    // This fallback does not apply when SSH integration or user configuration
    // substitutes xterm-256color.
    if std::env::var("TERM").as_deref() == Ok("xterm-ghostty") {
        return true;
    }

    // https://github.com/microsoft/terminal/pull/8055
    if std::env::var("WT_SESSION").is_ok() {
        return true;
    }

    // https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC
    if std::env::var("ConEmuANSI").ok() == Some("ON".into()) {
        return true;
    }

    // Ptyxis added OSC 9;4 support in 48.0.
    // See https://gitlab.gnome.org/chergert/ptyxis/-/issues/305
    let ptyxis = std::env::var("PTYXIS_VERSION")
        .ok()
        .and_then(|version| version.split(".").next()?.parse::<i32>().ok())
        .map(|major_version| major_version >= 48)
        .unwrap_or(false);
    if ptyxis {
        return true;
    }

    // Konsole added support in 26.04.0.
    // https://invent.kde.org/utilities/konsole/-/merge_requests/1054
    let konsole = std::env::var("KONSOLE_VERSION")
        .ok()
        .and_then(|version| version.parse::<u32>().ok())
        .map(|version| version >= 260400)
        .unwrap_or(false);
    if konsole {
        return true;
    }

    // Herdr 0.6.9 added OSC 9;4 capture in
    // https://github.com/herdrdev/herdr/commit/82468a0e03360219898ebf3735bdf559182009f8
    if term_program.as_deref() == Some("herdr") {
        return true;
    }

    // tmux 3.7 added OSC 9;4 handling and forwarding to capable client terminals.
    // https://github.com/tmux/tmux/blob/3.7/CHANGES
    if term_program.as_deref() == Some("tmux")
        && std::env::var("TERM_PROGRAM_VERSION")
            .ok()
            .map(|version| tmux_version_has_progress(&version))
            .unwrap_or(false)
    {
        return true;
    }

    false
}

fn tmux_version_has_progress(version: &str) -> bool {
    let Some((major, minor)) = version.split_once('.') else {
        return false;
    };
    // tmux patch releases append a lowercase letter, for example "3.7b".
    let minor = match minor.as_bytes().last() {
        Some(ch) if ch.is_ascii_lowercase() => &minor[..minor.len() - 1],
        _ => minor,
    };
    if major.is_empty()
        || minor.is_empty()
        || !major
            .bytes()
            .chain(minor.bytes())
            .all(|ch| ch.is_ascii_digit())
    {
        return false;
    }
    match (major.parse::<u32>(), minor.parse::<u32>()) {
        (Ok(major), Ok(minor)) => (major, minor) >= (3, 7),
        _ => false,
    }
}

// For iTerm, the TERM_FEATURES value "P" indicates OSC 9;4 support.
//
// Context: https://iterm2.com/feature-reporting/
fn term_features_has_progress(value: &str) -> bool {
    let mut current = String::new();

    for ch in value.chars() {
        if !ch.is_ascii_alphanumeric() {
            break;
        }
        if ch.is_ascii_uppercase() {
            if current == "P" {
                return true;
            }
            current.clear();
            current.push(ch);
        } else {
            current.push(ch);
        }
    }
    current == "P"
}

#[cfg(test)]
mod tests {
    use super::{term_features_has_progress, tmux_version_has_progress};

    #[test]
    fn term_features_progress_detection() {
        // With PROGRESS feature ("P")
        assert!(term_features_has_progress("MBT2ScP"));

        // Without PROGRESS feature
        assert!(!term_features_has_progress("MBT2Sc"));
    }

    #[test]
    fn tmux_progress_detection() {
        for (version, expected) in [
            ("3.6b", false),
            ("3.7", true),
            ("3.7b", true),
            ("3.8", true),
            ("3.10", true),
            ("4.0", true),
            ("2.10", false),
            ("", false),
            ("next-3.7", false),
            ("3.7junk", false),
            ("3.7.0", false),
            ("3.", false),
            ("3.a", false),
            ("+3.7", false),
            ("3.+7", false),
            ("4294967296.7", false),
        ] {
            assert_eq!(tmux_version_has_progress(version), expected, "{version:?}");
        }
    }
}
