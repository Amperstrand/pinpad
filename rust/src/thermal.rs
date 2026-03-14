//! Thermal button and keypad state management.
//!
//! This module provides the core thermal effect logic, including button state
//! tracking and intensity calculations based on exponential decay.

use core::ops::{Index, IndexMut};

/// Standard keypad button labels in order.
pub const BUTTON_LABELS: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '*', '0', '#'];

/// Configuration for thermal effect parameters.
#[derive(Clone, Copy, Debug)]
pub struct ThermalConfig {
    /// Time in milliseconds for full heat decay.
    ///
    /// After this duration, a pressed button's intensity will be essentially zero.
    /// Default: 30000 (30 seconds)
    pub decay_time_ms: u32,

    /// Minimum intensity threshold for rendering.
    ///
    /// Buttons with intensity below this value are considered "cold" and won't
    /// render the thermal glow effect.
    /// Default: 0.02
    pub min_visible_intensity: f32,

    /// Number of concentric glow rings.
    ///
    /// More rings create a smoother gradient effect but require more rendering.
    /// Default: 10
    pub num_rings: u8,
}

impl Default for ThermalConfig {
    #[inline]
    fn default() -> Self {
        Self {
            decay_time_ms: 30000,
            min_visible_intensity: 0.02,
            num_rings: 10,
        }
    }
}

impl ThermalConfig {
    /// Create a new configuration with default values.
    #[inline]
    pub const fn new() -> Self {
        Self {
            decay_time_ms: 30000,
            min_visible_intensity: 0.02,
            num_rings: 10,
        }
    }

    /// Set the decay time in milliseconds.
    #[inline]
    pub const fn decay_time_ms(mut self, ms: u32) -> Self {
        self.decay_time_ms = ms;
        self
    }

    /// Set the minimum visible intensity.
    #[inline]
    pub const fn min_visible_intensity(mut self, intensity: f32) -> Self {
        self.min_visible_intensity = intensity;
        self
    }

    /// Set the number of glow rings.
    #[inline]
    pub const fn num_rings(mut self, rings: u8) -> Self {
        self.num_rings = rings;
        self
    }
}

/// Represents a single button with thermal state.
///
/// Tracks when the button was pressed and calculates current heat intensity
/// based on exponential decay over time.
#[derive(Clone, Copy, Debug)]
pub struct ThermalButton {
    /// The button's display label.
    pub label: char,
    /// Timestamp when button was pressed (milliseconds), or None if never pressed.
    pressed_at: Option<u64>,
}

impl ThermalButton {
    /// Create a new button with the given label.
    #[inline]
    pub const fn new(label: char) -> Self {
        Self {
            label,
            pressed_at: None,
        }
    }

    /// Record a button press at the given timestamp.
    ///
    /// # Arguments
    /// * `timestamp_ms` - Current time in milliseconds
    #[inline]
    pub fn press(&mut self, timestamp_ms: u64) {
        self.pressed_at = Some(timestamp_ms);
    }

    /// Get the timestamp when this button was pressed.
    #[inline]
    pub const fn pressed_at(&self) -> Option<u64> {
        self.pressed_at
    }

    /// Calculate current heat intensity.
    ///
    /// Uses exponential decay formula: `intensity = e^(-decay_progress * 3)`
    ///
    /// # Arguments
    /// * `now_ms` - Current time in milliseconds
    /// * `config` - Thermal configuration
    ///
    /// # Returns
    /// Intensity value between 0.0 and 1.0
    #[inline]
    pub fn intensity(&self, now_ms: u64, config: &ThermalConfig) -> f32 {
        let pressed_at = match self.pressed_at {
            Some(t) => t,
            None => return 0.0,
        };

        if now_ms < pressed_at {
            return 0.0;
        }

        let elapsed = (now_ms - pressed_at) as f32;
        let decay_progress = elapsed / config.decay_time_ms as f32;

        // Exponential decay: intensity = e^(-decay_progress * 3)
        let intensity = exp_neg_x_times_3(decay_progress);

        intensity.clamp(0.0, 1.0)
    }

    /// Check if this button is currently "hot" (above minimum intensity).
    #[inline]
    pub fn is_hot(&self, now_ms: u64, config: &ThermalConfig) -> bool {
        self.intensity(now_ms, config) >= config.min_visible_intensity
    }

    /// Clear the heat signature (reset to unpressed state).
    #[inline]
    pub fn reset(&mut self) {
        self.pressed_at = None;
    }
}

/// Calculate e^(-x * 3) using Taylor series approximation.
///
/// This is a no_std compatible implementation that doesn't require
/// the full `libm` crate. For most thermal use cases, this precision
/// is sufficient.
#[inline]
fn exp_neg_x_times_3(x: f32) -> f32 {
    // e^(-3x) = 1 - 3x + (3x)^2/2 - (3x)^3/6 + (3x)^4/24 - ...
    // For x >= 1.0, the value is very close to 0
    if x >= 1.0 {
        let x3 = x * 3.0;
        // Use more terms for better accuracy
        let x3_2 = x3 * x3;
        let x3_3 = x3_2 * x3;
        let x3_4 = x3_3 * x3;
        let x3_5 = x3_4 * x3;

        1.0 - x3 + x3_2 * 0.5 - x3_3 * 0.16666667 + x3_4 * 0.041666667 - x3_5 * 0.0083333333
    } else {
        // For small x, use a simpler approximation
        let x3 = x * 3.0;
        (1.0 - x3 * 0.5).max(0.0)
    }
}

/// Manages all 12 buttons in a telephone keypad layout.
///
/// Provides methods to press buttons, query intensities, and reset heat.
#[derive(Clone, Copy, Debug)]
pub struct ThermalKeypad {
    buttons: [ThermalButton; 12],
    config: ThermalConfig,
}

impl Default for ThermalKeypad {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalKeypad {
    /// Create a new keypad with default configuration.
    #[inline]
    pub fn new() -> Self {
        Self::with_config(ThermalConfig::new())
    }

    /// Create a new keypad with custom configuration.
    #[inline]
    pub fn with_config(config: ThermalConfig) -> Self {
        let buttons = [
            ThermalButton::new('1'),
            ThermalButton::new('2'),
            ThermalButton::new('3'),
            ThermalButton::new('4'),
            ThermalButton::new('5'),
            ThermalButton::new('6'),
            ThermalButton::new('7'),
            ThermalButton::new('8'),
            ThermalButton::new('9'),
            ThermalButton::new('*'),
            ThermalButton::new('0'),
            ThermalButton::new('#'),
        ];

        Self { buttons, config }
    }

    /// Get the configuration.
    #[inline]
    pub const fn config(&self) -> &ThermalConfig {
        &self.config
    }

    /// Press a button by label.
    ///
    /// # Arguments
    /// * `label` - The button label to press
    /// * `timestamp_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// `true` if the button exists and was pressed, `false` otherwise
    #[inline]
    pub fn press(&mut self, label: char, timestamp_ms: u64) -> bool {
        if let Some(button) = self.button_mut(label) {
            button.press(timestamp_ms);
            true
        } else {
            false
        }
    }

    /// Get intensity for a specific button.
    ///
    /// # Arguments
    /// * `label` - The button label
    /// * `now_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// Intensity value between 0.0 and 1.0, or 0.0 if button not found
    #[inline]
    pub fn intensity(&self, label: char, now_ms: u64) -> f32 {
        self.button(label)
            .map(|b| b.intensity(now_ms, &self.config))
            .unwrap_or(0.0)
    }

    /// Get all button intensities.
    ///
    /// # Arguments
    /// * `now_ms` - Current time in milliseconds
    ///
    /// # Returns
    /// Array of (label, intensity) pairs for all 12 buttons
    #[inline]
    pub fn intensities(&self, now_ms: u64) -> [(char, f32); 12] {
        let mut result = [(' ', 0.0f32); 12];

        for (i, button) in self.buttons.iter().enumerate() {
            result[i] = (button.label, button.intensity(now_ms, &self.config));
        }

        result
    }

    /// Get a button by label.
    #[inline]
    pub fn button(&self, label: char) -> Option<&ThermalButton> {
        self.buttons.iter().find(|b| b.label == label)
    }

    /// Get a mutable reference to a button by label.
    #[inline]
    pub fn button_mut(&mut self, label: char) -> Option<&mut ThermalButton> {
        self.buttons.iter_mut().find(|b| b.label == label)
    }

    /// Get a button by index (0-11).
    #[inline]
    pub const fn button_at(&self, index: usize) -> Option<&ThermalButton> {
        if index < 12 {
            Some(&self.buttons[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to a button by index (0-11).
    #[inline]
    pub fn button_at_mut(&mut self, index: usize) -> Option<&mut ThermalButton> {
        if index < 12 {
            Some(&mut self.buttons[index])
        } else {
            None
        }
    }

    /// Get all buttons.
    #[inline]
    pub const fn buttons(&self) -> &[ThermalButton; 12] {
        &self.buttons
    }

    /// Clear all heat signatures.
    #[inline]
    pub fn reset(&mut self) {
        for button in &mut self.buttons {
            button.reset();
        }
    }

    /// Simulate entering a code with delays between presses.
    ///
    /// # Arguments
    /// * `code` - String slice of button labels to press
    /// * `start_ms` - Starting timestamp in milliseconds
    /// * `interval_ms` - Milliseconds between each press
    pub fn enter_code(&mut self, code: &str, start_ms: u64, interval_ms: u64) {
        for (i, ch) in code.chars().enumerate() {
            let timestamp = start_ms + (i as u64 * interval_ms);
            self.press(ch, timestamp);
        }
    }
}

impl Index<usize> for ThermalKeypad {
    type Output = ThermalButton;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.buttons[index]
    }
}

impl IndexMut<usize> for ThermalKeypad {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buttons[index]
    }
}

/// Calculate intensity for a specific ring with quadratic falloff.
///
/// `ring_intensity = base_intensity * (1 - ring_index/total_rings)^2`
///
/// # Arguments
/// * `base_intensity` - The button's base intensity
/// * `ring_index` - Ring number (0 = innermost)
/// * `total_rings` - Total number of rings
#[inline]
pub fn ring_intensity(base_intensity: f32, ring_index: u8, total_rings: u8) -> f32 {
    let falloff = 1.0 - (ring_index as f32 / total_rings as f32);
    base_intensity * falloff * falloff
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG: ThermalConfig = ThermalConfig::new();

    #[test]
    fn test_button_initial_state() {
        let button = ThermalButton::new('5');
        assert_eq!(button.label, '5');
        assert!(button.pressed_at().is_none());
        assert_eq!(button.intensity(0, &TEST_CONFIG), 0.0);
    }

    #[test]
    fn test_button_press() {
        let mut button = ThermalButton::new('5');
        button.press(1000);

        assert_eq!(button.pressed_at(), Some(1000));

        // Right after press, intensity should be ~1.0
        let intensity = button.intensity(1000, &TEST_CONFIG);
        assert!((intensity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_button_decay() {
        let mut button = ThermalButton::new('5');
        button.press(0);

        // At half decay time (15000ms), intensity should be significantly reduced
        let half_intensity = button.intensity(15000, &TEST_CONFIG);
        assert!(half_intensity < 0.5);
        assert!(half_intensity > 0.0);

        // At full decay time (30000ms), intensity should be very low
        let full_intensity = button.intensity(30000, &TEST_CONFIG);
        assert!(full_intensity < 0.1);
    }

    #[test]
    fn test_button_reset() {
        let mut button = ThermalButton::new('5');
        button.press(1000);
        assert!(button.pressed_at().is_some());

        button.reset();
        assert!(button.pressed_at().is_none());
        assert_eq!(button.intensity(1000, &TEST_CONFIG), 0.0);
    }

    #[test]
    fn test_keypad_creation() {
        let keypad = ThermalKeypad::new();

        // Check all buttons exist
        assert_eq!(keypad[0].label, '1');
        assert_eq!(keypad[11].label, '#');
    }

    #[test]
    fn test_keypad_press() {
        let mut keypad = ThermalKeypad::new();

        // Press valid button
        assert!(keypad.press('5', 1000));
        assert_eq!(keypad.intensity('5', 1000), 1.0);

        // Press invalid button
        assert!(!keypad.press('X', 1000));
    }

    #[test]
    fn test_keypad_intensities() {
        let mut keypad = ThermalKeypad::new();
        keypad.press('1', 1000);
        keypad.press('2', 2000);

        let intensities = keypad.intensities(2000);

        // Button 1 should have decayed slightly
        assert!(intensities[0].1 < 1.0);
        assert!(intensities[0].1 > 0.9);

        // Button 2 should be at full intensity
        assert!((intensities[1].1 - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ring_intensity() {
        // Innermost ring (index 0)
        let inner = ring_intensity(1.0, 0, 10);
        assert!((inner - 1.0).abs() < 0.01);

        // Middle ring (index 5)
        let middle = ring_intensity(1.0, 5, 10);
        let expected = 0.25; // (1 - 0.5)^2
        assert!((middle - expected).abs() < 0.01);

        // Outer ring (index 9)
        let outer = ring_intensity(1.0, 9, 10);
        let expected = 0.01; // (1 - 0.9)^2
        assert!((outer - expected).abs() < 0.01);
    }

    #[test]
    fn test_config_builder() {
        let config = ThermalConfig::new()
            .decay_time_ms(60000)
            .min_visible_intensity(0.05)
            .num_rings(15);

        assert_eq!(config.decay_time_ms, 60000);
        assert!((config.min_visible_intensity - 0.05).abs() < 0.001);
        assert_eq!(config.num_rings, 15);
    }

    #[test]
    fn test_enter_code() {
        let mut keypad = ThermalKeypad::new();
        keypad.enter_code("1234", 1000, 100);

        // Check that all digits were pressed with correct timing
        assert_eq!(keypad[0].pressed_at(), Some(1000));
        assert_eq!(keypad[1].pressed_at(), Some(1100));
        assert_eq!(keypad[2].pressed_at(), Some(1200));
        assert_eq!(keypad[3].pressed_at(), Some(1300));
    }
}
