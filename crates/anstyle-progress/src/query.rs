/// Determines whether the terminal supports ANSI OSC 9;4.
pub fn supports_term_progress(is_terminal: bool) -> bool {
    let windows_terminal = std::env::var("WT_SESSION").is_ok();
    let conemu = std::env::var("ConEmuANSI").ok() == Some("ON".into());
    let term_program = std::env::var("TERM_PROGRAM").ok();
    let wezterm = term_program == Some("WezTerm".into());
    let ghostty = term_program == Some("ghostty".into());
    // iTerm added OSC 9;4 support in v3.6.6, which we can check for.
    // For context: https://github.com/rust-lang/cargo/pull/16506#discussion_r2706584034
    let iterm = term_program == Some("iTerm.app".into())
        && std::env::var("TERM_FEATURES")
            .ok()
            .map(|v| term_features_has_progress(&v))
            .unwrap_or(false);
    // Ptyxis added OSC 9;4 support in 48.0.
    // See https://gitlab.gnome.org/chergert/ptyxis/-/issues/305
    let ptyxis = std::env::var("PTYXIS_VERSION")
        .ok()
        .and_then(|version| version.split(".").next()?.parse::<i32>().ok())
        .map(|major_version| major_version >= 48)
        .unwrap_or(false);

    (windows_terminal || conemu || wezterm || ghostty || iterm || ptyxis) && is_terminal
}

// For iTerm, the TERM_FEATURES value "P" indicates OSC 9;4 support.
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
    use super::term_features_has_progress;

    #[test]
    fn term_features_progress_detection() {
        // With PROGRESS feature ("P")
        assert!(term_features_has_progress("MBT2ScP"));

        // Without PROGRESS feature
        assert!(!term_features_has_progress("MBT2Sc"));
    }
}
