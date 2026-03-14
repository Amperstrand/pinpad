pub mod colors {
    pub const PRIMARY_CYAN: (u8, u8, u8) = (0, 255, 255);
    pub const DEEP_BLUE: (u8, u8, u8) = (0, 71, 171);
    pub const HEALTH_TEAL: (u8, u8, u8) = (0, 255, 204);
    pub const STASIS_BLUE: (u8, u8, u8) = (51, 153, 255);
    pub const ALERT_RED: (u8, u8, u8) = (255, 51, 0);
    pub const TEXT_BLUE: (u8, u8, u8) = (160, 230, 255);
}

#[derive(Clone, Copy, Debug)]
pub struct DeadSpaceConfig {
    pub hologram_opacity: f32,
    pub chromatic_offset_px: u8,
    pub jitter_range_px: f32,
    pub scanline_darkness: f32,
    pub grid_rows: u8,
    pub grid_cols: u8,
}

impl Default for DeadSpaceConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DeadSpaceConfig {
    #[inline]
    pub const fn new() -> Self {
        Self {
            hologram_opacity: 0.72,
            chromatic_offset_px: 2,
            jitter_range_px: 2.0,
            scanline_darkness: 0.08,
            grid_rows: 4,
            grid_cols: 4,
        }
    }

    #[inline]
    pub const fn hologram_opacity(mut self, value: f32) -> Self {
        self.hologram_opacity = value;
        self
    }

    #[inline]
    pub const fn chromatic_offset_px(mut self, value: u8) -> Self {
        self.chromatic_offset_px = value;
        self
    }

    #[inline]
    pub const fn jitter_range_px(mut self, value: f32) -> Self {
        self.jitter_range_px = value;
        self
    }

    #[inline]
    pub const fn scanline_darkness(mut self, value: f32) -> Self {
        self.scanline_darkness = value;
        self
    }

    #[inline]
    pub const fn grid_rows(mut self, rows: u8) -> Self {
        self.grid_rows = rows;
        self
    }

    #[inline]
    pub const fn grid_cols(mut self, cols: u8) -> Self {
        self.grid_cols = cols;
        self
    }

    #[inline]
    pub const fn slot_count(&self) -> u16 {
        self.grid_rows as u16 * self.grid_cols as u16
    }

    #[inline]
    pub const fn hologram_alpha_u8(&self) -> u8 {
        (self.hologram_opacity.clamp(0.0, 1.0) * 255.0) as u8
    }

    #[inline]
    pub const fn scanline_alpha_u8(&self) -> u8 {
        (self.scanline_darkness.clamp(0.0, 1.0) * 255.0) as u8
    }

    #[inline]
    pub const fn validate(&self) -> bool {
        self.hologram_opacity >= 0.60
            && self.hologram_opacity <= 0.80
            && self.scanline_darkness >= 0.05
            && self.scanline_darkness <= 0.10
            && self.chromatic_offset_px >= 1
            && self.chromatic_offset_px <= 2
            && self.jitter_range_px >= 1.0
            && self.jitter_range_px <= 2.0
            && self.grid_rows > 0
            && self.grid_cols > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_within_spec() {
        let cfg = DeadSpaceConfig::new();
        assert!(cfg.validate());
        assert_eq!(cfg.slot_count(), 16);
        assert_eq!(cfg.hologram_alpha_u8(), 183);
        assert_eq!(cfg.scanline_alpha_u8(), 20);
    }

    #[test]
    fn builder_updates_values() {
        let cfg = DeadSpaceConfig::new()
            .hologram_opacity(0.75)
            .chromatic_offset_px(1)
            .jitter_range_px(1.5)
            .scanline_darkness(0.09)
            .grid_rows(3)
            .grid_cols(3);

        assert!(cfg.validate());
        assert_eq!(cfg.slot_count(), 9);
        assert_eq!(cfg.chromatic_offset_px, 1);
    }
}
