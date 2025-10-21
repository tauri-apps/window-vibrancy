use crate::Error;
use objc2::rc::Retained;
use objc2_app_kit::NSColor;

/// Represents a tint color that can be specified either as RGBA components
/// or as a hex color string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintColor {
    /// RGBA color with components in range 0-255
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    /// Hex color string in format "#RRGGBB" or "#RRGGBBAA"
    Hex(String),
}

impl TintColor {
    /// Creates a new `TintColor` from RGBA components.
    ///
    /// # Example
    /// ```
    /// use window_vibrancy::TintColor;
    /// let color = TintColor::rgba(255, 0, 0, 128); // Semi-transparent red
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba { r, g, b, a }
    }

    /// Creates a new `TintColor` from a hex string.
    ///
    /// Accepts formats:
    /// - `#RRGGBB` (6 digits, alpha defaults to 255)
    /// - `#RRGGBBAA` (8 digits)
    /// - `RRGGBB` (6 digits without #)
    /// - `RRGGBBAA` (8 digits without #)
    ///
    /// # Example
    /// ```
    /// use window_vibrancy::TintColor;
    /// let color1 = TintColor::from_hex("#FF0000").unwrap();     // Red
    /// let color2 = TintColor::from_hex("#FF000080").unwrap();   // Semi-transparent red
    /// let color3 = TintColor::from_hex("0064C8").unwrap();      // Blue without #
    /// ```
    pub fn from_hex(hex: impl Into<String>) -> Result<Self, Error> {
        let hex_string = hex.into();
        // Validate the hex string can be parsed
        let _ = Self::parse_hex_to_rgba(&hex_string)?;
        Ok(Self::Hex(hex_string))
    }

    /// Converts the color to RGBA components.
    pub fn to_rgba(&self) -> Result<(u8, u8, u8, u8), Error> {
        match self {
            Self::Rgba { r, g, b, a } => Ok((*r, *g, *b, *a)),
            Self::Hex(hex) => Self::parse_hex_to_rgba(hex),
        }
    }

    /// Converts the color to an `NSColor` object.
    pub(crate) fn to_nscolor(&self) -> Result<Retained<NSColor>, Error> {
        let (r, g, b, a) = self.to_rgba()?;
        let rf = r as f64 / 255.0;
        let gf = g as f64 / 255.0;
        let bf = b as f64 / 255.0;
        let af = a as f64 / 255.0;

        Ok(NSColor::colorWithRed_green_blue_alpha(rf, gf, bf, af))
    }

    /// Parses a hex color string to RGBA components.
    fn parse_hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, u8), Error> {
        // Remove whitespace and convert to uppercase
        let cleaned = hex.trim().to_uppercase();

        // Remove # prefix if present
        let hex_str = cleaned.strip_prefix('#').unwrap_or(&cleaned);

        // Validate length
        if hex_str.len() != 6 && hex_str.len() != 8 {
            return Err(Error::InvalidHexColor(format!(
                "Hex color must be 6 or 8 characters (got {}): {}",
                hex_str.len(),
                hex
            )));
        }

        // Validate all characters are valid hex
        if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::InvalidHexColor(format!(
                "Invalid hex characters in: {}",
                hex
            )));
        }

        // Parse the hex value
        let rgba = u32::from_str_radix(hex_str, 16).map_err(|e| {
            Error::InvalidHexColor(format!("Failed to parse hex color '{}': {}", hex, e))
        })?;

        if hex_str.len() == 6 {
            // #RRGGBB format - alpha defaults to 255
            let r = ((rgba >> 16) & 0xFF) as u8;
            let g = ((rgba >> 8) & 0xFF) as u8;
            let b = (rgba & 0xFF) as u8;
            Ok((r, g, b, 255))
        } else {
            // #RRGGBBAA format
            let r = ((rgba >> 24) & 0xFF) as u8;
            let g = ((rgba >> 16) & 0xFF) as u8;
            let b = ((rgba >> 8) & 0xFF) as u8;
            let a = (rgba & 0xFF) as u8;
            Ok((r, g, b, a))
        }
    }
}

impl Default for TintColor {
    fn default() -> Self {
        Self::Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

// Convenient From implementations
impl From<(u8, u8, u8, u8)> for TintColor {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::Rgba { r, g, b, a }
    }
}

impl From<(u8, u8, u8)> for TintColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::Rgba { r, g, b, a: 255 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_creation() {
        let color = TintColor::rgba(255, 128, 64, 32);
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 32));
    }

    #[test]
    fn test_hex_6_digits_with_hash() {
        let color = TintColor::from_hex("#FF8040").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 255));
    }

    #[test]
    fn test_hex_6_digits_without_hash() {
        let color = TintColor::from_hex("FF8040").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 255));
    }

    #[test]
    fn test_hex_8_digits_with_hash() {
        let color = TintColor::from_hex("#FF804020").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 32));
    }

    #[test]
    fn test_hex_8_digits_without_hash() {
        let color = TintColor::from_hex("FF804020").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 32));
    }

    #[test]
    fn test_hex_lowercase() {
        let color = TintColor::from_hex("#ff8040").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 255));
    }

    #[test]
    fn test_hex_with_whitespace() {
        let color = TintColor::from_hex("  #FF8040  ").unwrap();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 255));
    }

    #[test]
    fn test_common_colors() {
        // Black
        let black = TintColor::from_hex("#000000").unwrap();
        assert_eq!(black.to_rgba().unwrap(), (0, 0, 0, 255));

        // White
        let white = TintColor::from_hex("#FFFFFF").unwrap();
        assert_eq!(white.to_rgba().unwrap(), (255, 255, 255, 255));

        // Red
        let red = TintColor::from_hex("#FF0000").unwrap();
        assert_eq!(red.to_rgba().unwrap(), (255, 0, 0, 255));

        // Green
        let green = TintColor::from_hex("#00FF00").unwrap();
        assert_eq!(green.to_rgba().unwrap(), (0, 255, 0, 255));

        // Blue
        let blue = TintColor::from_hex("#0000FF").unwrap();
        assert_eq!(blue.to_rgba().unwrap(), (0, 0, 255, 255));

        // Semi-transparent blue
        let blue_alpha = TintColor::from_hex("#0064C880").unwrap();
        assert_eq!(blue_alpha.to_rgba().unwrap(), (0, 100, 200, 128));
    }

    #[test]
    fn test_invalid_hex_length() {
        let result = TintColor::from_hex("#FFF");
        assert!(result.is_err());

        let result = TintColor::from_hex("#FFFFFFF");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_hex_characters() {
        let result = TintColor::from_hex("#GGGGGG");
        assert!(result.is_err());

        let result = TintColor::from_hex("#FF00ZZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_tuple_rgba() {
        let color: TintColor = (255, 128, 64, 32).into();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 32));
    }

    #[test]
    fn test_from_tuple_rgb() {
        let color: TintColor = (255, 128, 64).into();
        assert_eq!(color.to_rgba().unwrap(), (255, 128, 64, 255));
    }

    #[test]
    fn test_color_equality() {
        let color1 = TintColor::rgba(255, 0, 0, 255);
        let color2 = TintColor::from_hex("#FF0000").unwrap();

        // They should convert to the same RGBA values
        assert_eq!(color1.to_rgba().unwrap(), color2.to_rgba().unwrap());
    }

    #[test]
    fn test_default() {
        let color = TintColor::default();
        assert_eq!(color.to_rgba().unwrap(), (0, 0, 0, 0));
    }
}
