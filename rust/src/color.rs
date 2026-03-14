//! Color palette definitions and intensity-to-color mapping.
//!
//! This module provides thermal color palettes that match the JavaScript and Python
//! reference implementations exactly. Each palette defines color stops that are
//! linearly interpolated based on intensity values.

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

/// A single color stop in a palette.
///
/// Represents a point in the intensity range (0.0 to 1.0) with an associated RGB color.
#[derive(Clone, Copy, Debug)]
pub struct ColorStop {
    /// Intensity threshold (0.0 to 1.0)
    pub t: f32,
    /// RGB color at this threshold
    pub color: Rgb888,
}

impl ColorStop {
    /// Create a new color stop.
    ///
    /// # Arguments
    /// * `t` - Intensity threshold (0.0 to 1.0)
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    #[inline]
    pub const fn new(t: f32, r: u8, g: u8, b: u8) -> Self {
        Self {
            t,
            color: Rgb888::new(r, g, b),
        }
    }
}

/// Available thermal color palettes.
///
/// Each palette provides a different visual style for the thermal effect.
/// All palettes are designed to show heat intensity through color gradients.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThermalPalette {
    /// Splinter Cell style - deep blue through cyan, green, yellow to near-white.
    ///
    /// This is the classic Splinter Cell (2002) thermal vision look.
    /// Matches the in-game thermal goggle effect.
    #[default]
    SplinterCell,

    /// Classic thermal camera - blue through cyan, yellow, orange to red.
    ///
    /// Traditional thermal imaging color scheme used in many
    /// commercial thermal cameras.
    Classic,

    /// Ironbow palette - professional thermal camera style.
    ///
    /// Black through purple, red, orange, yellow to white.
    /// Commonly used in professional thermal imaging equipment.
    Ironbow,

    /// Hot/Cold palette - blue through white to red.
    ///
    /// Simple two-color gradient emphasizing hot vs cold.
    HotCold,
}

impl ThermalPalette {
    /// Get the color stops for this palette.
    ///
    /// Returns a slice of color stops sorted by intensity threshold.
    /// These stops are used for linear interpolation when mapping
    /// intensity values to colors.
    #[inline]
    pub fn stops(&self) -> &'static [ColorStop] {
        match self {
            ThermalPalette::SplinterCell => &SPLINTER_CELL_STOPS,
            ThermalPalette::Classic => &CLASSIC_STOPS,
            ThermalPalette::Ironbow => &IRONBOW_STOPS,
            ThermalPalette::HotCold => &HOT_COLD_STOPS,
        }
    }
}

/// Splinter Cell palette color stops.
///
/// Deep blue-black → cobalt → cyan → green → yellow → near-white
static SPLINTER_CELL_STOPS: [ColorStop; 7] = [
    ColorStop::new(0.00, 5, 8, 32),
    ColorStop::new(0.20, 12, 42, 110),
    ColorStop::new(0.40, 20, 120, 170),
    ColorStop::new(0.62, 82, 190, 132),
    ColorStop::new(0.80, 220, 218, 90),
    ColorStop::new(0.92, 255, 235, 140),
    ColorStop::new(1.00, 255, 250, 220),
];

/// Classic thermal palette color stops.
///
/// Blue → cyan → yellow → orange → red
static CLASSIC_STOPS: [ColorStop; 6] = [
    ColorStop::new(0.0, 0, 0, 40),
    ColorStop::new(0.2, 0, 80, 160),
    ColorStop::new(0.4, 0, 200, 200),
    ColorStop::new(0.6, 200, 200, 0),
    ColorStop::new(0.8, 255, 128, 0),
    ColorStop::new(1.0, 255, 50, 50),
];

/// Ironbow palette color stops.
///
/// Black → purple → red → orange → yellow → white
static IRONBOW_STOPS: [ColorStop; 6] = [
    ColorStop::new(0.0, 0, 0, 0),
    ColorStop::new(0.2, 40, 0, 60),
    ColorStop::new(0.4, 150, 0, 50),
    ColorStop::new(0.6, 255, 80, 0),
    ColorStop::new(0.8, 255, 220, 80),
    ColorStop::new(1.0, 255, 255, 255),
];

/// Hot/Cold palette color stops.
///
/// Blue → white → red
static HOT_COLD_STOPS: [ColorStop; 5] = [
    ColorStop::new(0.0, 0, 0, 100),
    ColorStop::new(0.3, 50, 50, 200),
    ColorStop::new(0.5, 255, 255, 255),
    ColorStop::new(0.7, 255, 150, 50),
    ColorStop::new(1.0, 255, 0, 0),
];

/// Maps intensity values to RGB colors using thermal palettes.
///
/// This struct provides methods to convert intensity values (0.0 to 1.0)
/// to RGB colors using linear interpolation between color stops in
/// the selected palette.
#[derive(Clone, Copy, Debug, Default)]
pub struct ThermalColorMapper {
    /// The currently selected palette.
    palette: ThermalPalette,
}

impl ThermalColorMapper {
    /// Create a new color mapper with the default palette (SplinterCell).
    #[inline]
    pub const fn new() -> Self {
        Self {
            palette: ThermalPalette::SplinterCell,
        }
    }

    /// Create a color mapper with a specific palette.
    ///
    /// # Arguments
    /// * `palette` - The thermal palette to use for color mapping
    #[inline]
    pub const fn with_palette(palette: ThermalPalette) -> Self {
        Self { palette }
    }

    /// Set the color palette.
    ///
    /// # Arguments
    /// * `palette` - The thermal palette to use
    #[inline]
    pub fn set_palette(&mut self, palette: ThermalPalette) {
        self.palette = palette;
    }

    /// Get the current palette.
    #[inline]
    pub const fn palette(&self) -> ThermalPalette {
        self.palette
    }

    /// Convert an intensity value to an RGB color.
    ///
    /// Uses linear interpolation between color stops in the current palette.
    /// Intensity is clamped to the valid range [0.0, 1.0].
    ///
    /// # Arguments
    /// * `intensity` - Intensity value (0.0 = cold, 1.0 = hot)
    ///
    /// # Returns
    /// The interpolated RGB color.
    ///
    /// # Example
    /// ```
    /// use thermal_pinpad::color::{ThermalColorMapper, ThermalPalette};
    /// use embedded_graphics::pixelcolor::Rgb888;
    ///
    /// let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);
    /// let color = mapper.intensity_to_rgb(0.8);
    /// ```
    pub fn intensity_to_rgb(&self, intensity: f32) -> Rgb888 {
        // Clamp intensity to valid range
        let intensity = intensity.clamp(0.0, 1.0);

        let stops = self.palette.stops();

        // Find the two stops to interpolate between
        let mut lower = &stops[0];
        let mut upper = &stops[stops.len() - 1];

        for i in 0..stops.len() - 1 {
            if intensity >= stops[i].t && intensity <= stops[i + 1].t {
                lower = &stops[i];
                upper = &stops[i + 1];
                break;
            }
        }

        // Calculate interpolation factor
        let range_t = upper.t - lower.t;
        let factor = if range_t == 0.0 {
            0.0
        } else {
            (intensity - lower.t) / range_t
        };

        // Interpolate RGB components
        let lower_color = lower.color;
        let upper_color = upper.color;

        let r = interpolate_component(lower_color.r(), upper_color.r(), factor);
        let g = interpolate_component(lower_color.g(), upper_color.g(), factor);
        let b = interpolate_component(lower_color.b(), upper_color.b(), factor);

        Rgb888::new(r, g, b)
    }
}

/// Interpolate between two color components.
///
/// # Arguments
/// * `lower` - Lower bound value (0-255)
/// * `upper` - Upper bound value (0-255)
/// * `factor` - Interpolation factor (0.0 to 1.0)
///
/// # Returns
/// Interpolated value (0-255)
#[inline]
fn interpolate_component(lower: u8, upper: u8, factor: f32) -> u8 {
    let lower_f = lower as f32;
    let upper_f = upper as f32;
    let result = lower_f + (upper_f - lower_f) * factor;
    (result + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splinter_cell_palette_extremes() {
        let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);

        // Test cold (0.0)
        let cold = mapper.intensity_to_rgb(0.0);
        assert_eq!(cold, Rgb888::new(5, 8, 32));

        // Test hot (1.0)
        let hot = mapper.intensity_to_rgb(1.0);
        assert_eq!(hot, Rgb888::new(255, 250, 220));
    }

    #[test]
    fn test_intensity_clamping() {
        let mapper = ThermalColorMapper::new();

        // Values outside range should be clamped
        let below = mapper.intensity_to_rgb(-0.5);
        let zero = mapper.intensity_to_rgb(0.0);
        assert_eq!(below, zero);

        let above = mapper.intensity_to_rgb(1.5);
        let one = mapper.intensity_to_rgb(1.0);
        assert_eq!(above, one);
    }

    #[test]
    fn test_classic_palette() {
        let mapper = ThermalColorMapper::with_palette(ThermalPalette::Classic);

        let cold = mapper.intensity_to_rgb(0.0);
        assert_eq!(cold, Rgb888::new(0, 0, 40));

        let hot = mapper.intensity_to_rgb(1.0);
        assert_eq!(hot, Rgb888::new(255, 50, 50));
    }

    #[test]
    fn test_interpolation() {
        let mapper = ThermalColorMapper::with_palette(ThermalPalette::Classic);

        // At 0.2, should be exactly the second stop
        let at_02 = mapper.intensity_to_rgb(0.2);
        assert_eq!(at_02, Rgb888::new(0, 80, 160));

        // At 0.1, should be between first and second stop
        let at_01 = mapper.intensity_to_rgb(0.1);
        assert_eq!(at_01, Rgb888::new(0, 40, 100));
    }
}
