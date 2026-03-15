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
    pub jitter_min_px: f32,
    pub jitter_max_px: f32,
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
            jitter_min_px: 1.0,
            jitter_max_px: 2.0,
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
    pub const fn jitter_range_px(mut self, min: f32, max: f32) -> Self {
        self.jitter_min_px = min;
        self.jitter_max_px = max;
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
            && self.jitter_min_px >= 1.0
            && self.jitter_min_px <= 2.0
            && self.jitter_max_px >= self.jitter_min_px
            && self.jitter_max_px <= 2.0
            && self.grid_rows > 0
            && self.grid_cols > 0
    }

    #[inline]
    pub fn instability_offsets(&self, frame_index: u32, unstable: bool) -> (f32, f32) {
        let min = self.jitter_min_px;
        let max = self.jitter_max_px;
        let span = (max - min).max(0.0);
        let cycle = (frame_index % 8) as f32 / 7.0;
        let magnitude = min + span * cycle;
        let factor = if unstable { 1.0 } else { 0.8 };
        let x = magnitude * factor * if (frame_index & 1) == 0 { 1.0 } else { -1.0 };
        let y = magnitude
            * factor
            * if ((frame_index + 1) & 1) == 0 {
                1.0
            } else {
                -1.0
            };
        (x, y)
    }

    #[inline]
    pub const fn step_selection(&self, selected: u16, dx: i8, dy: i8) -> u16 {
        let rows = self.grid_rows as i16;
        let cols = self.grid_cols as i16;
        let row = (selected / self.grid_cols as u16) as i16;
        let col = (selected % self.grid_cols as u16) as i16;

        let next_row = (row + dy as i16 + rows) % rows;
        let next_col = (col + dx as i16 + cols) % cols;
        (next_row * cols + next_col) as u16
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
            .jitter_range_px(1.0, 1.5)
            .scanline_darkness(0.09)
            .grid_rows(3)
            .grid_cols(3);

        assert!(cfg.validate());
        assert_eq!(cfg.slot_count(), 9);
        assert_eq!(cfg.chromatic_offset_px, 1);
    }

    #[test]
    fn selection_wraps_inventory_grid() {
        let cfg = DeadSpaceConfig::new();
        assert_eq!(cfg.step_selection(0, -1, 0), 3);
        assert_eq!(cfg.step_selection(0, 0, -1), 12);
        assert_eq!(cfg.step_selection(15, 1, 0), 12);
        assert_eq!(cfg.step_selection(15, 0, 1), 3);
    }

    #[test]
    fn stable_instability_jitter_stays_in_spec() {
        let cfg = DeadSpaceConfig::new();
        let (sx, sy) = cfg.instability_offsets(7, false);
        assert!(sx.abs() >= 0.8);
        assert!(sy.abs() >= 0.8);

        let (ux, uy) = cfg.instability_offsets(7, true);
        assert!(ux.abs() >= 1.0);
        assert!(uy.abs() >= 1.0);
        assert!(ux.abs() <= 2.0);
        assert!(uy.abs() <= 2.0);
    }
}
