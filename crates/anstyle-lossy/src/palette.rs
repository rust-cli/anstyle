//! Popular color palettes for [`anstyle::AnsiColor`]
//!
//! Based on [wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit)

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Palette([anstyle::RgbColor; 16]);

impl Palette {
    pub(crate) const fn rgb_from_ansi(&self, color: anstyle::AnsiColor) -> anstyle::RgbColor {
        let color = anstyle::Ansi256Color::from_ansi(color);
        self.0[color.index() as usize]
    }

    pub(crate) const fn rgb_from_index(&self, index: u8) -> Option<anstyle::RgbColor> {
        let index = index as usize;
        if index <= self.0.len() {
            Some(self.0[index])
        } else {
            None
        }
    }

    pub(crate) const fn find_match(&self, color: anstyle::RgbColor) -> anstyle::AnsiColor {
        let mut best_index = 0;
        let mut best_distance = crate::distance(color, self.0[best_index]);

        let mut index = best_index + 1;
        while index < self.0.len() {
            let distance = crate::distance(color, self.0[index]);
            if distance < best_distance {
                best_index = index;
                best_distance = distance;
            }

            index += 1;
        }

        match anstyle::Ansi256Color(best_index as u8).into_ansi() {
            Some(color) => color,
            None => {
                // Panic
                #[allow(clippy::no_effect)]
                ["best_index is out of bounds"][best_index];
                // Make compiler happy
                anstyle::AnsiColor::Black
            }
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        DEFAULT
    }
}

#[cfg(not(windows))]
pub use VGA as DEFAULT;

#[cfg(windows)]
pub use WIN10_CONSOLE as DEFAULT;

pub const VGA: Palette = Palette([
    anstyle::RgbColor(0, 0, 0),
    anstyle::RgbColor(170, 0, 0),
    anstyle::RgbColor(0, 170, 0),
    anstyle::RgbColor(170, 85, 0),
    anstyle::RgbColor(0, 0, 170),
    anstyle::RgbColor(170, 0, 170),
    anstyle::RgbColor(0, 170, 170),
    anstyle::RgbColor(170, 170, 170),
    anstyle::RgbColor(85, 85, 85),
    anstyle::RgbColor(255, 85, 85),
    anstyle::RgbColor(85, 255, 85),
    anstyle::RgbColor(255, 255, 85),
    anstyle::RgbColor(85, 85, 255),
    anstyle::RgbColor(255, 85, 255),
    anstyle::RgbColor(85, 255, 255),
    anstyle::RgbColor(255, 255, 255),
]);

pub const WIN10_CONSOLE: Palette = Palette([
    anstyle::RgbColor(12, 12, 12),
    anstyle::RgbColor(197, 15, 31),
    anstyle::RgbColor(19, 161, 14),
    anstyle::RgbColor(193, 156, 0),
    anstyle::RgbColor(0, 55, 218),
    anstyle::RgbColor(136, 23, 152),
    anstyle::RgbColor(58, 150, 221),
    anstyle::RgbColor(204, 204, 204),
    anstyle::RgbColor(118, 118, 118),
    anstyle::RgbColor(231, 72, 86),
    anstyle::RgbColor(22, 198, 12),
    anstyle::RgbColor(249, 241, 165),
    anstyle::RgbColor(59, 120, 255),
    anstyle::RgbColor(180, 0, 158),
    anstyle::RgbColor(97, 214, 214),
    anstyle::RgbColor(242, 242, 242),
]);
