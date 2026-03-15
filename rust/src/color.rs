//! Color palette definitions and intensity-to-color mapping.
//!
//! This module provides thermal color palettes that match the JavaScript and Python
//! reference implementations exactly. For Splinter Cell palette, uses HSV-based
//! interpolation with a precomputed LUT for embedded-friendly performance.

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use micromath::F32Ext;

const SPLINTER_GAMMA: f32 = 1.3;

#[derive(Clone, Copy, Debug)]
struct HsvStop {
    t: f32,
    h: f32,
    s: f32,
    v: f32,
}

impl HsvStop {
    const fn new(t: f32, h: f32, s: f32, v: f32) -> Self {
        Self { t, h, s, v }
    }
}

/// A single color stop in a palette.
#[derive(Clone, Copy, Debug)]
pub struct ColorStop {
    pub t: f32,
    pub color: Rgb888,
}

impl ColorStop {
    #[inline]
    pub const fn new(t: f32, r: u8, g: u8, b: u8) -> Self {
        Self {
            t,
            color: Rgb888::new(r, g, b),
        }
    }
}

/// Available thermal color palettes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThermalPalette {
    #[default]
    SplinterCell,
    Classic,
    Ironbow,
    HotCold,
}

impl ThermalPalette {
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

/// Splinter Cell palette - HSV-interpolated for perceptual smoothness.
/// Cold-end uses purple (h=275) per SC Wiki: "dark, purplish color".
/// Peak is warm yellow-orange (h=42), not near-white.
static SPLINTER_CELL_STOPS: [ColorStop; 7] = [
    ColorStop::new(0.00, 20, 5, 31),
    ColorStop::new(0.20, 12, 42, 110),
    ColorStop::new(0.40, 20, 120, 170),
    ColorStop::new(0.62, 82, 190, 132),
    ColorStop::new(0.80, 220, 218, 90),
    ColorStop::new(0.92, 255, 235, 140),
    ColorStop::new(1.00, 240, 208, 32),
];

static SPLINTER_CELL_HSV_STOPS: [HsvStop; 7] = [
    HsvStop::new(0.00, 275.0, 85.0, 12.0),
    HsvStop::new(0.20, 265.0, 90.0, 43.0),
    HsvStop::new(0.40, 195.0, 88.0, 67.0),
    HsvStop::new(0.62, 150.0, 58.0, 75.0),
    HsvStop::new(0.80, 55.0, 60.0, 86.0),
    HsvStop::new(0.92, 48.0, 90.0, 94.0),
    HsvStop::new(1.00, 50.0, 87.0, 94.0),
];

/// Classic thermal palette color stops.
static CLASSIC_STOPS: [ColorStop; 6] = [
    ColorStop::new(0.0, 0, 0, 40),
    ColorStop::new(0.2, 0, 80, 160),
    ColorStop::new(0.4, 0, 200, 200),
    ColorStop::new(0.6, 200, 200, 0),
    ColorStop::new(0.8, 255, 128, 0),
    ColorStop::new(1.0, 255, 50, 50),
];

/// Ironbow palette color stops.
static IRONBOW_STOPS: [ColorStop; 6] = [
    ColorStop::new(0.0, 0, 0, 0),
    ColorStop::new(0.2, 40, 0, 60),
    ColorStop::new(0.4, 150, 0, 50),
    ColorStop::new(0.6, 255, 80, 0),
    ColorStop::new(0.8, 255, 220, 80),
    ColorStop::new(1.0, 255, 255, 255),
];

/// Hot/Cold palette color stops.
static HOT_COLD_STOPS: [ColorStop; 5] = [
    ColorStop::new(0.0, 0, 0, 100),
    ColorStop::new(0.3, 50, 50, 200),
    ColorStop::new(0.5, 255, 255, 255),
    ColorStop::new(0.7, 255, 150, 50),
    ColorStop::new(1.0, 255, 0, 0),
];

/// Maps intensity values to RGB colors using thermal palettes.
#[derive(Clone, Copy, Debug, Default)]
pub struct ThermalColorMapper {
    palette: ThermalPalette,
}

impl ThermalColorMapper {
    #[inline]
    pub const fn new() -> Self {
        Self {
            palette: ThermalPalette::SplinterCell,
        }
    }

    #[inline]
    pub const fn with_palette(palette: ThermalPalette) -> Self {
        Self { palette }
    }

    #[inline]
    pub fn set_palette(&mut self, palette: ThermalPalette) {
        self.palette = palette;
    }

    #[inline]
    pub const fn palette(&self) -> ThermalPalette {
        self.palette
    }

    pub fn intensity_to_rgb(&self, intensity: f32) -> Rgb888 {
        let intensity = intensity.clamp(0.0, 1.0);

        if self.palette == ThermalPalette::SplinterCell {
            return splinter_hsv_color(intensity);
        }

        let stops = self.palette.stops();

        let mut lower = &stops[0];
        let mut upper = &stops[stops.len() - 1];

        for i in 0..stops.len() - 1 {
            if intensity >= stops[i].t && intensity <= stops[i + 1].t {
                lower = &stops[i];
                upper = &stops[i + 1];
                break;
            }
        }

        let range_t = upper.t - lower.t;
        let factor = if range_t == 0.0 {
            0.0
        } else {
            (intensity - lower.t) / range_t
        };

        let lower_color = lower.color;
        let upper_color = upper.color;

        let r = interpolate_component(lower_color.r(), upper_color.r(), factor);
        let g = interpolate_component(lower_color.g(), upper_color.g(), factor);
        let b = interpolate_component(lower_color.b(), upper_color.b(), factor);

        Rgb888::new(r, g, b)
    }
}

#[inline]
fn interpolate_component(lower: u8, upper: u8, factor: f32) -> u8 {
    let lower_f = lower as f32;
    let upper_f = upper as f32;
    let result = lower_f + (upper_f - lower_f) * factor;
    (result + 0.5) as u8
}

#[inline]
fn splinter_hsv_color(intensity: f32) -> Rgb888 {
    let gamma_t = F32Ext::powf(intensity, SPLINTER_GAMMA);
    let mut lower = SPLINTER_CELL_HSV_STOPS[0];
    let mut upper = SPLINTER_CELL_HSV_STOPS[SPLINTER_CELL_HSV_STOPS.len() - 1];

    for i in 0..SPLINTER_CELL_HSV_STOPS.len() - 1 {
        if gamma_t >= SPLINTER_CELL_HSV_STOPS[i].t && gamma_t <= SPLINTER_CELL_HSV_STOPS[i + 1].t {
            lower = SPLINTER_CELL_HSV_STOPS[i];
            upper = SPLINTER_CELL_HSV_STOPS[i + 1];
            break;
        }
    }

    let range = upper.t - lower.t;
    let factor = if range == 0.0 {
        0.0
    } else {
        (gamma_t - lower.t) / range
    };

    let (h, s, v) = lerp_hsv_shortest(lower, upper, factor);
    let (r, g, b) = hsv_to_rgb(h, s, v);
    Rgb888::new(r, g, b)
}

#[inline]
fn lerp_hsv_shortest(lower: HsvStop, upper: HsvStop, t: f32) -> (f32, f32, f32) {
    let mut h_diff = upper.h - lower.h;
    if h_diff > 180.0 {
        h_diff -= 360.0;
    }
    if h_diff < -180.0 {
        h_diff += 360.0;
    }

    let mut h = lower.h + h_diff * t;
    if h < 0.0 {
        h += 360.0;
    }
    if h >= 360.0 {
        h -= 360.0;
    }

    let s = lower.s + (upper.s - lower.s) * t;
    let v = lower.v + (upper.v - lower.v) * t;
    (h, s, v)
}

#[inline]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let s_norm = s / 100.0;
    let v_norm = v / 100.0;
    let c = v_norm * s_norm;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v_norm - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0 + 0.5) as u8,
        ((g + m) * 255.0 + 0.5) as u8,
        ((b + m) * 255.0 + 0.5) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splinter_cell_palette_extremes() {
        let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);

        let cold = mapper.intensity_to_rgb(0.0);
        assert!(cold.b() > cold.r());
        assert!(cold.b() > cold.g());

        let hot = mapper.intensity_to_rgb(1.0);
        assert!((hot.r() as i32 - 240).abs() <= 2);
        assert!((hot.g() as i32 - 208).abs() <= 4);
        assert!((hot.b() as i32 - 32).abs() <= 3);
        assert!(hot.r() > hot.g());
        assert!(hot.g() > hot.b());
    }

    #[test]
    fn test_splinter_midpoint_smooth_hsv_blend() {
        let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);
        let c1 = mapper.intensity_to_rgb(0.40);
        let c2 = mapper.intensity_to_rgb(0.41);

        assert!((c1.r() as i32 - c2.r() as i32).abs() <= 16);
        assert!((c1.g() as i32 - c2.g() as i32).abs() <= 16);
        assert!((c1.b() as i32 - c2.b() as i32).abs() <= 16);
    }

    #[test]
    fn test_intensity_clamping() {
        let mapper = ThermalColorMapper::new();

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

        let at_02 = mapper.intensity_to_rgb(0.2);
        assert_eq!(at_02, Rgb888::new(0, 80, 160));

        let at_01 = mapper.intensity_to_rgb(0.1);
        assert_eq!(at_01, Rgb888::new(0, 40, 100));
    }
}
