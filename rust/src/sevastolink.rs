//! Sevastolink Terminal Pinpad - Alien: Isolation (2014)
//!
//! This module provides the core Sevastolink terminal logic, including
//! code entry, authentication state management, and CRT effect parameters.
//! Designed to match the iconic green-on-black aesthetic from the game.

// =============================================================================
// COLOR DEFINITIONS (Cross-Platform Consistent)
// =============================================================================

/// Sevastolink terminal color palette.
///
/// These RGB values match the JavaScript and Python implementations exactly.
pub mod colors {
    /// Background dark green / shadows - #0c290c
    pub const XENOMORPH_SKIN: (u8, u8, u8) = (12, 41, 12);
    /// Secondary background, dim text - #134213
    pub const TERMINAL_GREEN: (u8, u8, u8) = (19, 66, 19);
    /// Primary text, highlights, active elements - #05b669
    pub const SEEGSON_GREEN: (u8, u8, u8) = (5, 182, 105);
    /// Warnings, errors, accents - #f07826
    pub const ACID_BLOOD: (u8, u8, u8) = (240, 120, 38);
    /// Bright text, selection highlight - #ccd5d4
    pub const HYPERSLEEP_WHITE: (u8, u8, u8) = (204, 213, 212);
    /// Muted text, disabled elements - #7a807f
    pub const SYNTHETIC_SKIN: (u8, u8, u8) = (122, 128, 127);
    /// CRT screen background - #000000
    pub const PURE_BLACK: (u8, u8, u8) = (0, 0, 0);
}

// =============================================================================
// CONFIGURATION
// =============================================================================

/// Configuration for Sevastolink terminal parameters.
#[derive(Clone, Copy, Debug)]
pub struct SevastolinkConfig {
    /// Maximum digits in access code.
    /// Default: 8
    pub max_code_length: u8,

    /// Cursor blink interval in milliseconds.
    /// Default: 530
    pub cursor_blink_ms: u16,

    /// Button flash duration in milliseconds.
    /// Default: 150
    pub keypress_flash_ms: u16,

    /// Error display duration in milliseconds.
    /// Default: 250
    pub error_flash_ms: u16,

    /// Success display duration in milliseconds.
    /// Default: 400
    pub success_flash_ms: u16,

    /// Authentication verification delay in milliseconds.
    /// Default: 800
    pub verify_delay_ms: u16,

    /// CRT scan line intensity (0.0-1.0).
    /// Default: 0.25
    pub scanline_intensity: f32,

    /// Noise intensity (0.0-1.0).
    /// Default: 0.08
    pub noise_intensity: f32,

    /// Film grain update interval in milliseconds (~10fps).
    /// Default: 100
    pub grain_update_ms: u16,

    /// Chromatic aberration offset in pixels.
    /// Default: 2
    pub chroma_offset: u8,

    /// Chromatic aberration threshold (flash intensity > this triggers effect).
    /// Default: 0.4
    pub chroma_threshold: f32,
}

impl Default for SevastolinkConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SevastolinkConfig {
    /// Create a new configuration with default values.
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_code_length: 8,
            cursor_blink_ms: 530,
            keypress_flash_ms: 150,
            error_flash_ms: 250,
            success_flash_ms: 400,
            verify_delay_ms: 800,
            scanline_intensity: 0.25,
            noise_intensity: 0.08,
            grain_update_ms: 100,
            chroma_offset: 2,
            chroma_threshold: 0.4,
        }
    }

    /// Set the maximum code length.
    #[inline]
    pub const fn max_code_length(mut self, len: u8) -> Self {
        self.max_code_length = len;
        self
    }

    /// Set the cursor blink interval.
    #[inline]
    pub const fn cursor_blink_ms(mut self, ms: u16) -> Self {
        self.cursor_blink_ms = ms;
        self
    }

    /// Set the keypress flash duration.
    #[inline]
    pub const fn keypress_flash_ms(mut self, ms: u16) -> Self {
        self.keypress_flash_ms = ms;
        self
    }
}

// =============================================================================
// AUTHENTICATION STATE
// =============================================================================

/// Authentication state machine states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthState {
    /// Waiting for input
    Idle,
    /// Verifying entered code
    Verifying,
    /// Access granted
    Success,
    /// Access denied
    Denied,
}

impl Default for AuthState {
    #[inline]
    fn default() -> Self {
        Self::Idle
    }
}

// =============================================================================
// SEVASTOLINK KEYPAD
// =============================================================================

/// Standard keypad button labels in order.
pub const BUTTON_LABELS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E'];

/// Maximum code length (compile-time constant for array sizing)
const MAX_CODE_LEN: usize = 8;

/// Manages the Sevastolink terminal keypad state.
///
/// Tracks entered code, authentication state, and button press timing.
#[derive(Clone, Debug)]
pub struct SevastolinkKeypad {
    /// Entered code as a fixed-size array of chars
    code: [char; MAX_CODE_LEN],
    /// Current code length
    code_len: u8,
    /// Current authentication state
    auth_state: AuthState,
    /// Currently flashing button (if any)
    flashing_button: Option<char>,
    /// Timestamp when flash started (ms)
    flash_start_time: u64,
    /// Timestamp when auth state changed (ms)
    state_change_time: u64,
    /// Configuration
    config: SevastolinkConfig,
}

impl Default for SevastolinkKeypad {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SevastolinkKeypad {
    /// Create a new keypad with default configuration.
    #[inline]
    pub fn new() -> Self {
        Self::with_config(SevastolinkConfig::new())
    }

    /// Create a new keypad with custom configuration.
    #[inline]
    pub fn with_config(config: SevastolinkConfig) -> Self {
        Self {
            code: ['\0'; MAX_CODE_LEN],
            code_len: 0,
            auth_state: AuthState::Idle,
            flashing_button: None,
            flash_start_time: 0,
            state_change_time: 0,
            config,
        }
    }

    /// Get the configuration.
    #[inline]
    pub const fn config(&self) -> &SevastolinkConfig {
        &self.config
    }

    /// Get the current code as a slice of chars.
    #[inline]
    pub fn code_chars(&self) -> &[char] {
        &self.code[..self.code_len as usize]
    }

    /// Get the code length.
    #[inline]
    pub const fn code_len(&self) -> u8 {
        self.code_len
    }

    /// Get the current authentication state.
    #[inline]
    pub const fn auth_state(&self) -> AuthState {
        self.auth_state
    }

    /// Check if a button is currently valid to press.
    #[inline]
    pub fn can_press(&self) -> bool {
        self.auth_state != AuthState::Verifying
    }

    /// Process a button press.
    ///
    /// # Arguments
    /// * `label` - The button label pressed
    /// * `timestamp_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// `PressResult` indicating what action was taken
    #[inline]
    pub fn press(&mut self, label: char, timestamp_ms: u64) -> PressResult {
        if self.auth_state == AuthState::Verifying {
            return PressResult::Ignored;
        }

        // Record flash
        self.flashing_button = Some(label);
        self.flash_start_time = timestamp_ms;

        match label {
            'C' => {
                // Clear last digit
                if self.code_len > 0 {
                    self.code_len -= 1;
                    self.code[self.code_len as usize] = '\0';
                }
                PressResult::Cleared
            }
            'E' => {
                // Submit code
                self.submit_code(timestamp_ms);
                PressResult::Submitted
            }
            '0'..='9' => {
                if self.code_len < self.config.max_code_length {
                    self.code[self.code_len as usize] = label;
                    self.code_len += 1;
                    PressResult::Added
                } else {
                    PressResult::Ignored
                }
            }
            _ => PressResult::Ignored,
        }
    }

    /// Submit the current code for verification.
    fn submit_code(&mut self, timestamp_ms: u64) {
        if self.code_len == 0 {
            self.auth_state = AuthState::Denied;
            self.state_change_time = timestamp_ms;
            return;
        }

        self.auth_state = AuthState::Verifying;
        self.state_change_time = timestamp_ms;
    }

    /// Complete verification with result.
    ///
    /// # Arguments
    /// * `success` - Whether authentication succeeded
    /// * `timestamp_ms` - Current time in milliseconds
    #[inline]
    pub fn verify_complete(&mut self, success: bool, timestamp_ms: u64) {
        self.auth_state = if success {
            AuthState::Success
        } else {
            AuthState::Denied
        };
        self.state_change_time = timestamp_ms;
    }

    /// Reset authentication state to idle.
    #[inline]
    pub fn reset_auth_state(&mut self) {
        self.auth_state = AuthState::Idle;
        self.code = ['\0'; MAX_CODE_LEN];
        self.code_len = 0;
    }

    /// Clear the entered code.
    #[inline]
    pub fn clear_code(&mut self) {
        self.code = ['\0'; MAX_CODE_LEN];
        self.code_len = 0;
    }

    /// Get current flash state and intensity.
    ///
    /// # Arguments
    /// * `now_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// Tuple of (button_label, intensity 0.0-1.0)
    #[inline]
    pub fn flash_intensity(&mut self, now_ms: u64) -> (Option<char>, f32) {
        if self.flashing_button.is_none() {
            return (None, 0.0);
        }

        if now_ms < self.flash_start_time {
            return (None, 0.0);
        }

        let elapsed = now_ms - self.flash_start_time;
        let flash_ms = self.config.keypress_flash_ms as u64;

        if elapsed >= flash_ms {
            self.flashing_button = None;
            return (None, 0.0);
        }

        let intensity = 1.0 - (elapsed as f32 / flash_ms as f32);
        (self.flashing_button, intensity.max(0.0))
    }

    /// Get how long we've been in current auth state.
    ///
    /// # Arguments
    /// * `now_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// Duration in milliseconds
    #[inline]
    pub fn state_duration(&self, now_ms: u64) -> u64 {
        if now_ms > self.state_change_time {
            now_ms - self.state_change_time
        } else {
            0
        }
    }

    /// Check if auth state should auto-reset.
    ///
    /// # Arguments
    /// * `now_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// `true` if the state should be reset
    #[inline]
    pub fn should_reset(&self, now_ms: u64) -> bool {
        let duration = self.state_duration(now_ms);
        match self.auth_state {
            AuthState::Success => duration >= self.config.success_flash_ms as u64,
            AuthState::Denied => duration >= self.config.error_flash_ms as u64,
            _ => false,
        }
    }
}

/// Result of a button press.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressResult {
    /// Digit was added to code
    Added,
    /// Code was cleared
    Cleared,
    /// Code was submitted for verification
    Submitted,
    /// Press was ignored (invalid state or button)
    Ignored,
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Linearly interpolate between two colors.
///
/// # Arguments
/// * `color1` - Starting color (RGB)
/// * `color2` - Ending color (RGB)
/// * `t` - Interpolation factor (0.0-1.0)
///
/// # Returns
/// Interpolated color (RGB)
#[inline]
pub fn lerp_color(color1: (u8, u8, u8), color2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);

    let r = (color1.0 as f32 + (color2.0 as f32 - color1.0 as f32) * t) as u8;
    let g = (color1.1 as f32 + (color2.1 as f32 - color1.1 as f32) * t) as u8;
    let b = (color1.2 as f32 + (color2.2 as f32 - color1.2 as f32) * t) as u8;

    (r, g, b)
}

/// Calculate cursor blink state.
///
/// # Arguments
/// * `now_ms` - Current time in milliseconds
/// * `last_toggle_ms` - Last toggle timestamp
/// * `blink_interval_ms` - Blink interval in milliseconds
///
/// # Returns
/// Tuple of (is_visible, new_last_toggle)
#[inline]
pub fn cursor_blink(now_ms: u64, last_toggle_ms: u64, blink_interval_ms: u16) -> (bool, u64) {
    let elapsed = now_ms.saturating_sub(last_toggle_ms);
    if elapsed >= blink_interval_ms as u64 {
        let toggles = elapsed / blink_interval_ms as u64;
        let visible = toggles % 2 == 1;
        let new_toggle = last_toggle_ms + toggles * blink_interval_ms as u64;
        (visible, new_toggle)
    } else {
        // No toggle yet, return current state (assume started visible)
        (false, last_toggle_ms)
    }
}

/// Check if chromatic aberration should be applied based on flash intensity.
///
/// # Arguments
/// * `flash_intensity` - Current flash intensity (0.0-1.0)
/// * `threshold` - Threshold for triggering chromatic aberration
///
/// # Returns
/// `true` if chromatic aberration should be applied
#[inline]
pub fn should_apply_chromatic(flash_intensity: f32, threshold: f32) -> bool {
    flash_intensity > threshold
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG: SevastolinkConfig = SevastolinkConfig::new();

    #[test]
    fn test_keypad_creation() {
        let keypad = SevastolinkKeypad::new();
        assert_eq!(keypad.code_len(), 0);
        assert_eq!(keypad.auth_state(), AuthState::Idle);
        assert!(keypad.can_press());
    }

    #[test]
    fn test_button_press() {
        let mut keypad = SevastolinkKeypad::new();

        // Press digit
        let result = keypad.press('5', 1000);
        assert_eq!(result, PressResult::Added);
        assert_eq!(keypad.code_len(), 1);
        assert_eq!(keypad.code_chars(), &['5']);

        // Press another digit
        let result = keypad.press('3', 1100);
        assert_eq!(result, PressResult::Added);
        assert_eq!(keypad.code_len(), 2);
        assert_eq!(keypad.code_chars(), &['5', '3']);
    }

    #[test]
    fn test_clear_button() {
        let mut keypad = SevastolinkKeypad::new();

        keypad.press('1', 1000);
        keypad.press('2', 1100);
        assert_eq!(keypad.code_len(), 2);

        let result = keypad.press('C', 1200);
        assert_eq!(result, PressResult::Cleared);
        assert_eq!(keypad.code_len(), 1);
        assert_eq!(keypad.code_chars(), &['1']);
    }

    #[test]
    fn test_submit_code() {
        let mut keypad = SevastolinkKeypad::new();

        keypad.press('1', 1000);
        keypad.press('2', 1100);

        let result = keypad.press('E', 1200);
        assert_eq!(result, PressResult::Submitted);
        assert_eq!(keypad.auth_state(), AuthState::Verifying);
        assert!(!keypad.can_press());
    }

    #[test]
    fn test_submit_empty_code() {
        let mut keypad = SevastolinkKeypad::new();

        let result = keypad.press('E', 1000);
        assert_eq!(result, PressResult::Submitted);
        assert_eq!(keypad.auth_state(), AuthState::Denied);
    }

    #[test]
    fn test_verify_complete() {
        let mut keypad = SevastolinkKeypad::new();

        keypad.press('1', 1000);
        keypad.press('E', 1100);
        assert_eq!(keypad.auth_state(), AuthState::Verifying);

        keypad.verify_complete(true, 2000);
        assert_eq!(keypad.auth_state(), AuthState::Success);

        keypad.reset_auth_state();
        assert_eq!(keypad.auth_state(), AuthState::Idle);
        assert_eq!(keypad.code_len(), 0);
    }

    #[test]
    fn test_max_code_length() {
        let mut keypad = SevastolinkKeypad::new();

        // Enter 8 digits (max)
        for i in 0..8 {
            let result = keypad.press(('0' as u8 + i) as char, 1000 + i as u64 * 100);
            assert_eq!(result, PressResult::Added);
        }

        // 9th digit should be ignored
        let result = keypad.press('9', 2000);
        assert_eq!(result, PressResult::Ignored);
    }

    #[test]
    fn test_flash_intensity() {
        let mut keypad = SevastolinkKeypad::new();

        keypad.press('5', 1000);

        // Right after press
        let (_, intensity) = keypad.flash_intensity(1000);
        assert!((intensity - 1.0).abs() < 0.01);

        // Halfway through flash
        let (_, intensity) = keypad.flash_intensity(1075);
        assert!((intensity - 0.5).abs() < 0.1);

        // After flash duration
        let (_, intensity) = keypad.flash_intensity(1200);
        assert_eq!(intensity, 0.0);
    }

    #[test]
    fn test_lerp_color() {
        let color1 = (0, 0, 0);
        let color2 = (100, 200, 50);

        let result = lerp_color(color1, color2, 0.0);
        assert_eq!(result, (0, 0, 0));

        let result = lerp_color(color1, color2, 1.0);
        assert_eq!(result, (100, 200, 50));

        let result = lerp_color(color1, color2, 0.5);
        assert_eq!(result, (50, 100, 25));
    }

    #[test]
    fn test_cursor_blink() {
        let (visible, _) = cursor_blink(1000, 0, 530);
        // 1000ms / 530ms = ~1.89 toggles, so visible should be true
        assert!(visible);

        let (visible, _) = cursor_blink(500, 0, 530);
        // 500ms < 530ms, no toggle yet
        assert!(!visible);
    }

    #[test]
    fn test_config_builder() {
        let config = SevastolinkConfig::new()
            .max_code_length(6)
            .cursor_blink_ms(400)
            .keypress_flash_ms(100);

        assert_eq!(config.max_code_length, 6);
        assert_eq!(config.cursor_blink_ms, 400);
        assert_eq!(config.keypress_flash_ms, 100);
    }

    #[test]
    fn test_colors() {
        // Verify color values match spec
        assert_eq!(colors::XENOMORPH_SKIN, (12, 41, 12));
        assert_eq!(colors::TERMINAL_GREEN, (19, 66, 19));
        assert_eq!(colors::SEEGSON_GREEN, (5, 182, 105));
        assert_eq!(colors::ACID_BLOOD, (240, 120, 38));
        assert_eq!(colors::HYPERSLEEP_WHITE, (204, 213, 212));
        assert_eq!(colors::SYNTHETIC_SKIN, (122, 128, 127));
        assert_eq!(colors::PURE_BLACK, (0, 0, 0));
    }
}
