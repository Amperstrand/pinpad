use core::cmp::min;
use micromath::F32Ext;

pub const BUTTON_LABELS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E'];

pub const NEON_BLUE: (u8, u8, u8) = (42, 210, 255);
pub const NEON_ORANGE: (u8, u8, u8) = (255, 157, 0);
pub const NEON_WHITE: (u8, u8, u8) = (224, 247, 255);
pub const DEEP_BLACK: (u8, u8, u8) = (3, 5, 4);
pub const GRID_CYAN: (u8, u8, u8) = (0, 140, 163);

pub const BLOOM_BASE_ALPHA: u8 = 105;
pub const BLOOM_FLASH_ALPHA: u8 = 220;
pub const CORE_BASE_ALPHA: u8 = 130;
pub const CORE_FLASH_ALPHA: u8 = 250;
pub const CIRCUIT_PULSE_PERIOD_MS: u64 = 2200;
pub const NODE_GLOW_INTENSITY: f32 = 0.75;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TronConfig {
    pub verify_ms: u32,
    pub key_flash_ms: u32,
    pub success_hold_ms: u32,
    pub error_hold_ms: u32,
    pub max_code_length: u8,
    pub bloom_radius_px: u8,
}

impl TronConfig {
    pub const fn new() -> Self {
        Self {
            verify_ms: 360,
            key_flash_ms: 110,
            success_hold_ms: 220,
            error_hold_ms: 220,
            max_code_length: 4,
            bloom_radius_px: 14,
        }
    }

    pub const fn max_code_length(mut self, length: u8) -> Self {
        self.max_code_length = if length > 8 { 8 } else { length };
        self
    }
}

impl Default for TronConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthState {
    Idle,
    Verifying,
    Success,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressResult {
    Ignored,
    Accepted,
    Cleared,
    Submitted,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircuitTraceSegment {
    pub start: (i16, i16),
    pub end: (i16, i16),
    pub width: u8,
}

pub const CIRCUIT_TRACES: [CircuitTraceSegment; 6] = [
    CircuitTraceSegment {
        start: (30, 186),
        end: (140, 186),
        width: 2,
    },
    CircuitTraceSegment {
        start: (140, 186),
        end: (168, 214),
        width: 2,
    },
    CircuitTraceSegment {
        start: (168, 214),
        end: (168, 244),
        width: 2,
    },
    CircuitTraceSegment {
        start: (168, 244),
        end: (386, 244),
        width: 2,
    },
    CircuitTraceSegment {
        start: (278, 244),
        end: (304, 218),
        width: 2,
    },
    CircuitTraceSegment {
        start: (304, 218),
        end: (304, 191),
        width: 2,
    },
];

pub const CIRCUIT_NODES: [(i16, i16); 3] = [(140, 186), (168, 244), (304, 191)];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TronFrame {
    pub panel_opacity: u8,
    pub state: AuthState,
    pub button_bloom_alpha: [u8; 12],
    pub button_core_alpha: [u8; 12],
}

#[derive(Clone, Copy, Debug)]
pub struct TronKeypad {
    config: TronConfig,
    auth_state: AuthState,
    state_started_at: u64,
    flash_button: Option<char>,
    flash_started_at: u64,
    code: [u8; 8],
    code_len: u8,
    correct_code: [u8; 8],
    correct_code_len: u8,
}

impl Default for TronKeypad {
    fn default() -> Self {
        Self::new()
    }
}

impl TronKeypad {
    pub fn new() -> Self {
        let mut keypad = Self {
            config: TronConfig::new(),
            auth_state: AuthState::Idle,
            state_started_at: 0,
            flash_button: None,
            flash_started_at: 0,
            code: [0; 8],
            code_len: 0,
            correct_code: [0; 8],
            correct_code_len: 0,
        };
        keypad.set_correct_code("1982");
        keypad
    }

    pub fn with_config(config: TronConfig) -> Self {
        let mut keypad = Self {
            config,
            ..Self::new()
        };
        keypad.config = config;
        keypad
    }

    pub const fn config(&self) -> &TronConfig {
        &self.config
    }

    pub const fn auth_state(&self) -> AuthState {
        self.auth_state
    }

    pub fn masked_code(&self) -> [char; 8] {
        let mut out = ['\0'; 8];
        let len = self.code_len as usize;
        let mut i = 0;
        while i < len {
            out[i] = '*';
            i += 1;
        }
        out
    }

    pub fn set_correct_code(&mut self, code: &str) {
        let bytes = code.as_bytes();
        let limit = min(8, bytes.len());
        self.correct_code = [0; 8];
        self.correct_code_len = limit as u8;

        let mut i = 0;
        while i < limit {
            self.correct_code[i] = bytes[i];
            i += 1;
        }
    }

    pub fn press(&mut self, button: char, now_ms: u64) -> PressResult {
        if self.auth_state == AuthState::Verifying {
            return PressResult::Ignored;
        }

        if !BUTTON_LABELS.contains(&button) {
            return PressResult::Ignored;
        }

        self.flash_button = Some(button);
        self.flash_started_at = now_ms;

        if button.is_ascii_digit() {
            let max_len = min(self.config.max_code_length as usize, 8);
            if self.code_len as usize >= max_len {
                return PressResult::Ignored;
            }
            self.code[self.code_len as usize] = button as u8;
            self.code_len += 1;
            return PressResult::Accepted;
        }

        if button == 'C' {
            self.clear_code();
            self.auth_state = AuthState::Idle;
            self.state_started_at = now_ms;
            return PressResult::Cleared;
        }

        if button == 'E' {
            self.auth_state = if self.code_len == 0 {
                AuthState::Error
            } else {
                AuthState::Verifying
            };
            self.state_started_at = now_ms;
            return PressResult::Submitted;
        }

        PressResult::Ignored
    }

    pub fn update(&mut self, now_ms: u64) {
        if self.auth_state == AuthState::Verifying
            && now_ms.saturating_sub(self.state_started_at) >= self.config.verify_ms as u64
        {
            if self.is_code_correct() {
                self.auth_state = AuthState::Success;
            } else {
                self.auth_state = AuthState::Error;
                self.clear_code();
            }
            self.state_started_at = now_ms;
        }

        if self.auth_state == AuthState::Success
            && now_ms.saturating_sub(self.state_started_at) >= self.config.success_hold_ms as u64
        {
            self.auth_state = AuthState::Idle;
            self.state_started_at = now_ms;
            self.clear_code();
        }

        if self.auth_state == AuthState::Error
            && now_ms.saturating_sub(self.state_started_at) >= self.config.error_hold_ms as u64
        {
            self.auth_state = AuthState::Idle;
            self.state_started_at = now_ms;
        }

        if let Some(_) = self.flash_button {
            if now_ms.saturating_sub(self.flash_started_at) >= self.config.key_flash_ms as u64 {
                self.flash_button = None;
            }
        }
    }

    pub fn render_frame(&self, now_ms: u64) -> TronFrame {
        let panel_opacity = flicker_opacity(now_ms);
        let mut bloom = [BLOOM_BASE_ALPHA; 12];
        let mut core = [CORE_BASE_ALPHA; 12];

        if let Some(button) = self.flash_button {
            if let Some(idx) = button_index(button) {
                let elapsed = now_ms.saturating_sub(self.flash_started_at);
                let intensity = if elapsed >= self.config.key_flash_ms as u64 {
                    0.0
                } else {
                    1.0 - (elapsed as f32 / self.config.key_flash_ms as f32)
                };
                bloom[idx] = (BLOOM_BASE_ALPHA as f32
                    + intensity * (BLOOM_FLASH_ALPHA - BLOOM_BASE_ALPHA) as f32)
                    as u8;
                core[idx] = (CORE_BASE_ALPHA as f32
                    + intensity * (CORE_FLASH_ALPHA - CORE_BASE_ALPHA) as f32)
                    as u8;
            }
        }

        if self.auth_state == AuthState::Success {
            let mut i = 0;
            while i < 12 {
                bloom[i] = bloom[i].saturating_add(25);
                core[i] = core[i].saturating_add(15);
                i += 1;
            }
        }

        TronFrame {
            panel_opacity,
            state: self.auth_state,
            button_bloom_alpha: bloom,
            button_core_alpha: core,
        }
    }

    fn clear_code(&mut self) {
        self.code = [0; 8];
        self.code_len = 0;
    }

    fn is_code_correct(&self) -> bool {
        if self.code_len != self.correct_code_len {
            return false;
        }

        let len = self.code_len as usize;
        let mut i = 0;
        while i < len {
            if self.code[i] != self.correct_code[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

pub const fn button_bevel_points(x: i16, y: i16, w: i16, h: i16, inset: i16) -> [(i16, i16); 8] {
    [
        (x + inset, y),
        (x + w - inset, y),
        (x + w, y + inset),
        (x + w, y + h - inset),
        (x + w - inset, y + h),
        (x + inset, y + h),
        (x, y + h - inset),
        (x, y + inset),
    ]
}

pub fn button_index(button: char) -> Option<usize> {
    BUTTON_LABELS.iter().position(|&b| b == button)
}

pub fn flicker_opacity(now_ms: u64) -> u8 {
    let period1 = 110u64;
    let period2 = 170u64;
    let phase1 = (now_ms % period1) as f32 / period1 as f32;
    let phase2 = (now_ms % period2) as f32 / period2 as f32;
    let wave1 = F32Ext::sin(phase1 * core::f32::consts::TAU) * 0.04;
    let wave2 = F32Ext::sin(phase2 * core::f32::consts::TAU) * 0.03;
    let combined: f32 = 0.94 + wave1 + wave2;
    (combined.clamp(0.9, 1.0) * 255.0) as u8
}

pub fn circuit_pulse_alpha(now_ms: u64) -> u8 {
    let phase = (now_ms % CIRCUIT_PULSE_PERIOD_MS) as f32 / CIRCUIT_PULSE_PERIOD_MS as f32;
    let triangle = 1.0 - (2.0 * phase - 1.0).abs();
    let alpha = 160.0 + 90.0 * triangle;
    alpha as u8
}

pub fn node_glow_factor(now_ms: u64) -> f32 {
    let phase = (now_ms % 1100) as f32 / 1100.0;
    NODE_GLOW_INTENSITY + 0.25 * (1.0 - (phase - 1.0).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_colors() {
        assert_eq!(NEON_BLUE, (42, 210, 255));
        assert_eq!(DEEP_BLACK, (3, 5, 4));
    }

    #[test]
    fn test_flicker_stays_in_bounds() {
        let a = flicker_opacity(0);
        let b = flicker_opacity(60);
        let c = flicker_opacity(119);
        assert!((230..=255).contains(&a));
        assert!((230..=255).contains(&b));
        assert!((230..=255).contains(&c));
    }

    #[test]
    fn test_circuit_pulse_alpha() {
        let alpha = circuit_pulse_alpha(0);
        assert!((160..=250).contains(&alpha));
    }

    #[test]
    fn test_node_glow_factor() {
        let factor = node_glow_factor(0);
        assert!((0.75..=1.0).contains(&factor));
    }

    #[test]
    fn test_verify_path() {
        let mut keypad = TronKeypad::new();
        keypad.press('1', 10);
        keypad.press('9', 20);
        keypad.press('8', 30);
        keypad.press('2', 40);
        assert_eq!(keypad.press('E', 50), PressResult::Submitted);
        assert_eq!(keypad.auth_state(), AuthState::Verifying);
        keypad.update(450);
        assert_eq!(keypad.auth_state(), AuthState::Success);
    }

    #[test]
    fn test_bevel_points() {
        let points = button_bevel_points(10, 20, 100, 70, 8);
        assert_eq!(points[0], (18, 20));
        assert_eq!(points[7], (10, 28));
    }
}
