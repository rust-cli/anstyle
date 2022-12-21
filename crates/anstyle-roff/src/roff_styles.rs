#[derive(Debug, Clone)]
pub(crate) enum Color {
    Rgb(RgbColor),
    AnsiColor(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct RgbColor(pub u8, pub u8, pub u8);

/// Constants for setting Control Requests to ROFF
pub(crate) struct ControlRequests {}

impl ControlRequests {
    /// Control to Create a Color definition
    pub const CREATE_COLOR: &'static str = "defcolor";
    /// Roff control request to set background color (fill color)
    pub const BACKGROUND: &'static str = "fcolor";
    /// Roff control request to set foreground color (glyph color)
    pub const FOREGROUND: &'static str = "gcolor";
}

impl RgbColor {
    pub const DEFAULT: &'static str = "default";

    pub fn as_hex(&self) -> String {
        let val: usize = ((self.0 as usize) << 16) + ((self.1 as usize) << 8) + (self.2 as usize);
        format!("#{:06x}", val)
    }
}

/// Default AsciiColors supported by roff
#[cfg(test)]
mod tests {
    use super::RgbColor;

    #[test]
    fn to_hex() {
        assert_eq!(RgbColor(0, 0, 0).as_hex().as_str(), "#000000");
        assert_eq!(RgbColor(255, 0, 0).as_hex().as_str(), "#ff0000");
        assert_eq!(RgbColor(0, 255, 0).as_hex().as_str(), "#00ff00");
        assert_eq!(RgbColor(0, 0, 255).as_hex().as_str(), "#0000ff");
    }
}
