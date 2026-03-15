use core::cmp::min;
use micromath::F32Ext;

pub const BUTTON_LABELS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E'];

pub const BACKGROUND: (u8, u8, u8) = (19, 18, 0);
pub const PRIMARY_GOLD: (u8, u8, u8) = (255, 234, 33);
pub const AMBER: (u8, u8, u8) = (229, 175, 46);
pub const DARK_GOLD: (u8, u8, u8) = (180, 145, 37);
pub const CYAN: (u8, u8, u8) = (0, 255, 255);
pub const ALERT_RED: (u8, u8, u8) = (255, 0, 0);
pub const BOOT_MS: u32 = 400;
pub const KEYPRESS_FLASH_MS: u32 = 100;
pub const SCANLINE_CYCLE_MS: u32 = 2800;
pub const VERIFY_MS: u32 = 800;
pub const SUCCESS_FLASH_MS: u32 = 400;
pub const ERROR_FLASH_MS: u32 = 250;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeusExConfig {
    pub boot_ms: u32,
    pub keypress_flash_ms: u32,
    pub verify_ms: u32,
    pub success_flash_ms: u32,
    pub error_flash_ms: u32,
    pub max_code_length: u8,
}

impl Default for DeusExConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DeusExConfig {
    pub const fn new() -> Self {
        Self {
            boot_ms: BOOT_MS,
            keypress_flash_ms: KEYPRESS_FLASH_MS,
            verify_ms: VERIFY_MS,
            success_flash_ms: SUCCESS_FLASH_MS,
            error_flash_ms: ERROR_FLASH_MS,
            max_code_length: 4,
        }
    }

    pub const fn max_code_length(mut self, length: u8) -> Self {
        self.max_code_length = if length > 8 { 8 } else { length };
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthState {
    Booting,
    Idle,
    Verifying,
    Success,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationState {
    None,
    BootGlitch,
    KeypressFlash { button: char },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressResult {
    Ignored,
    Accepted,
    Cleared,
    Submitted,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyVisualPulse {
    pub button: char,
    pub outer_bloom: f32,
    pub inner_core: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DeusExKeypad {
    config: DeusExConfig,
    auth_state: AuthState,
    animation_state: AnimationState,
    state_started_at: u64,
    animation_started_at: u64,
    code: [u8; 8],
    code_len: u8,
    correct_code: [u8; 8],
    correct_code_len: u8,
}

impl Default for DeusExKeypad {
    fn default() -> Self {
        Self::new()
    }
}

impl DeusExKeypad {
    pub fn new() -> Self {
        Self::with_config(DeusExConfig::new())
    }

    pub fn with_config(config: DeusExConfig) -> Self {
        let mut keypad = Self {
            config,
            auth_state: AuthState::Booting,
            animation_state: AnimationState::BootGlitch,
            state_started_at: 0,
            animation_started_at: 0,
            code: [0; 8],
            code_len: 0,
            correct_code: [0; 8],
            correct_code_len: 0,
        };

        keypad.set_correct_code("0451");
        keypad
    }

    pub fn start_boot(&mut self, now_ms: u64) {
        self.auth_state = AuthState::Booting;
        self.animation_state = AnimationState::BootGlitch;
        self.state_started_at = now_ms;
        self.animation_started_at = now_ms;
        self.clear_code();
    }

    pub const fn config(&self) -> &DeusExConfig {
        &self.config
    }

    pub const fn auth_state(&self) -> AuthState {
        self.auth_state
    }

    pub const fn animation_state(&self) -> AnimationState {
        self.animation_state
    }

    pub fn boot_progress(&self, now_ms: u64) -> f32 {
        if self.auth_state != AuthState::Booting || self.config.boot_ms == 0 {
            return 1.0;
        }
        let elapsed = now_ms.saturating_sub(self.state_started_at) as f32;
        (elapsed / self.config.boot_ms as f32).clamp(0.0, 1.0)
    }

    pub fn code_len(&self) -> u8 {
        self.code_len
    }

    pub fn code_digits(&self) -> [char; 8] {
        let mut out = ['\0'; 8];
        let len = self.code_len as usize;
        let mut i = 0;
        while i < len {
            out[i] = self.code[i] as char;
            i += 1;
        }
        out
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
        if self.auth_state == AuthState::Booting || self.auth_state == AuthState::Verifying {
            return PressResult::Ignored;
        }

        if !BUTTON_LABELS.contains(&button) {
            return PressResult::Ignored;
        }

        self.animation_state = AnimationState::KeypressFlash { button };
        self.animation_started_at = now_ms;

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
            return PressResult::Cleared;
        }

        if button == 'E' {
            self.submit(now_ms);
            return PressResult::Submitted;
        }

        PressResult::Ignored
    }

    pub fn keypress_flash_intensity(&self, now_ms: u64) -> Option<(char, f32)> {
        if let AnimationState::KeypressFlash { button } = self.animation_state {
            let elapsed = now_ms.saturating_sub(self.animation_started_at);
            if elapsed >= self.config.keypress_flash_ms as u64 {
                return None;
            }

            let remaining = 1.0 - (elapsed as f32 / self.config.keypress_flash_ms as f32);
            return Some((button, remaining.clamp(0.0, 1.0)));
        }

        None
    }

    pub fn key_visual_pulse(&self, now_ms: u64) -> Option<KeyVisualPulse> {
        let (button, intensity) = self.keypress_flash_intensity(now_ms)?;
        Some(KeyVisualPulse {
            button,
            outer_bloom: 0.5 + (0.9 * intensity),
            inner_core: 0.6 + (0.4 * intensity),
        })
    }

    pub fn boot_glitch_intensity(&self, now_ms: u64) -> f32 {
        if self.auth_state != AuthState::Booting {
            return 0.0;
        }
        let progress = self.boot_progress(now_ms);
        F32Ext::powf(1.0 - progress, 0.65).clamp(0.0, 1.0)
    }

    pub fn scanline_phase(&self, now_ms: u64) -> f32 {
        let cycle = u64::from(SCANLINE_CYCLE_MS.max(1));
        ((now_ms % cycle) as f32 / cycle as f32).clamp(0.0, 1.0)
    }

    pub fn panel_glow_profile(&self, now_ms: u64) -> (f32, f32) {
        let glitch = self.boot_glitch_intensity(now_ms);
        match self.auth_state {
            AuthState::Success => (0.5, 0.45),
            AuthState::Error => (0.42, 0.2),
            _ => (0.28 + glitch * 0.42, 0.22 + glitch * 0.3),
        }
    }

    pub fn update(&mut self, now_ms: u64) {
        if self.auth_state == AuthState::Booting {
            if now_ms.saturating_sub(self.state_started_at) >= self.config.boot_ms as u64 {
                self.auth_state = AuthState::Idle;
                self.state_started_at = now_ms;
                self.animation_state = AnimationState::None;
            }
            return;
        }

        if self.auth_state == AuthState::Verifying {
            if now_ms.saturating_sub(self.state_started_at) >= self.config.verify_ms as u64 {
                if self.is_code_correct() {
                    self.auth_state = AuthState::Success;
                } else {
                    self.auth_state = AuthState::Error;
                    self.clear_code();
                }
                self.state_started_at = now_ms;
            }
            return;
        }

        if self.auth_state == AuthState::Success {
            if now_ms.saturating_sub(self.state_started_at) >= self.config.success_flash_ms as u64 {
                self.auth_state = AuthState::Idle;
                self.state_started_at = now_ms;
                self.clear_code();
            }
            return;
        }

        if self.auth_state == AuthState::Error {
            if now_ms.saturating_sub(self.state_started_at) >= self.config.error_flash_ms as u64 {
                self.auth_state = AuthState::Idle;
                self.state_started_at = now_ms;
            }
        }

        if let AnimationState::KeypressFlash { .. } = self.animation_state {
            if now_ms.saturating_sub(self.animation_started_at)
                >= self.config.keypress_flash_ms as u64
            {
                self.animation_state = AnimationState::None;
            }
        }
    }

    fn submit(&mut self, now_ms: u64) {
        self.auth_state = if self.code_len == 0 {
            AuthState::Error
        } else {
            AuthState::Verifying
        };
        self.state_started_at = now_ms;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_timing_matches_spec() {
        let c = DeusExConfig::new();
        assert_eq!(c.boot_ms, BOOT_MS);
        assert_eq!(c.keypress_flash_ms, KEYPRESS_FLASH_MS);
        assert_eq!(c.verify_ms, VERIFY_MS);
        assert_eq!(c.success_flash_ms, SUCCESS_FLASH_MS);
        assert_eq!(c.error_flash_ms, ERROR_FLASH_MS);
        assert_eq!(c.max_code_length, 4);
    }

    #[test]
    fn test_boot_to_idle_transition() {
        let mut keypad = DeusExKeypad::new();
        keypad.start_boot(1000);
        keypad.update(1399);
        assert_eq!(keypad.auth_state(), AuthState::Booting);
        keypad.update(1400);
        assert_eq!(keypad.auth_state(), AuthState::Idle);
    }

    #[test]
    fn test_keypress_flash_intensity() {
        let mut keypad = DeusExKeypad::new();
        keypad.start_boot(0);
        keypad.update(400);
        assert_eq!(keypad.press('1', 500), PressResult::Accepted);
        let flash = keypad.keypress_flash_intensity(550);
        assert!(flash.is_some());
        let (_, intensity) = flash.unwrap_or(('\0', 0.0));
        assert!(intensity < 1.0 && intensity > 0.0);
        assert!(keypad.keypress_flash_intensity(601).is_none());
    }

    #[test]
    fn test_visual_profiles_exist() {
        let mut keypad = DeusExKeypad::new();
        keypad.start_boot(0);
        assert!(keypad.boot_glitch_intensity(100) > 0.0);
        keypad.update(400);
        let phase = keypad.scanline_phase(1400);
        assert!((0.0..=1.0).contains(&phase));
        keypad.press('1', 500);
        let pulse = keypad.key_visual_pulse(550);
        assert!(pulse.is_some());
        let panel = keypad.panel_glow_profile(550);
        assert!(panel.0 > 0.0);
        assert!(panel.1 > 0.0);
    }

    #[test]
    fn test_verify_flow() {
        let mut keypad = DeusExKeypad::new();
        keypad.start_boot(0);
        keypad.update(400);
        keypad.press('0', 500);
        keypad.press('4', 600);
        keypad.press('5', 700);
        keypad.press('1', 800);
        keypad.press('E', 900);
        assert_eq!(keypad.auth_state(), AuthState::Verifying);
        keypad.update(1700);
        assert_eq!(keypad.auth_state(), AuthState::Success);
    }
}
