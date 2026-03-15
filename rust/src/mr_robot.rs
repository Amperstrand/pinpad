//! Mr. Robot FBI Terminal Pinpad state management.
//!
//! Provides state and logic for the hacking terminal pinpad.

use core::ops::{Index, IndexMut};

pub const MR_ROBOT_BUTTONS: [char; 12] =
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E'];

/// Configuration for the Mr. Robot terminal effect.
#[derive(Clone, Copy, Debug)]
pub struct TerminalConfig {
    pub cursor_blink_ms: u32,
    pub typing_delay_ms: u32,
    pub typing_variance_ms: u32,
    pub button_active_ms: u32,
    pub grain_update_interval_ms: u32,
    pub chromatic_offset_px: u8,
}

impl Default for TerminalConfig {
    #[inline]
    fn default() -> Self {
        Self {
            cursor_blink_ms: 530,
            typing_delay_ms: 30,
            typing_variance_ms: 15,
            button_active_ms: 150,
            grain_update_interval_ms: 100,
            chromatic_offset_px: 2,
        }
    }
}

impl TerminalConfig {
    #[inline]
    pub const fn new() -> Self {
        Self {
            cursor_blink_ms: 530,
            typing_delay_ms: 30,
            typing_variance_ms: 15,
            button_active_ms: 150,
            grain_update_interval_ms: 100,
            chromatic_offset_px: 2,
        }
    }

    #[inline]
    pub const fn typing_delay_bounds(&self) -> (u32, u32) {
        (
            self.typing_delay_ms.saturating_sub(self.typing_variance_ms),
            self.typing_delay_ms.saturating_add(self.typing_variance_ms),
        )
    }

    #[inline]
    pub const fn button_active_ms(mut self, button_active_ms: u32) -> Self {
        self.button_active_ms = button_active_ms;
        self
    }

    #[inline]
    pub fn typing_delay_for(&self, entropy: u64) -> u32 {
        let (min, max) = self.typing_delay_bounds();
        let span = max.saturating_sub(min).saturating_add(1);
        if span <= 1 {
            return min;
        }

        let mixed = entropy
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        min.saturating_add((mixed as u32) % span)
    }

    #[inline]
    pub const fn grain_update_interval_ms(mut self, interval_ms: u32) -> Self {
        self.grain_update_interval_ms = interval_ms;
        self
    }

    #[inline]
    pub const fn chromatic_offset_px(mut self, offset_px: u8) -> Self {
        self.chromatic_offset_px = offset_px;
        self
    }
}

/// Minimal frame cache helper for effects like film grain.
#[derive(Clone, Copy, Debug)]
pub struct GrainCacheState {
    last_update_ms: Option<u64>,
}

impl GrainCacheState {
    #[inline]
    pub const fn new() -> Self {
        Self {
            last_update_ms: None,
        }
    }

    #[inline]
    pub const fn last_update_ms(&self) -> Option<u64> {
        self.last_update_ms
    }

    #[inline]
    pub fn should_refresh(&self, now_ms: u64, interval_ms: u32) -> bool {
        match self.last_update_ms {
            Some(last) => now_ms.saturating_sub(last) >= interval_ms as u64,
            None => true,
        }
    }

    #[inline]
    pub fn mark_updated(&mut self, now_ms: u64) {
        self.last_update_ms = Some(now_ms);
    }
}

impl Default for GrainCacheState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a terminal button.
#[derive(Clone, Copy, Debug)]
pub struct TerminalButton {
    pub label: char,
    pressed_at: Option<u64>,
}

impl TerminalButton {
    #[inline]
    pub const fn new(label: char) -> Self {
        Self {
            label,
            pressed_at: None,
        }
    }

    #[inline]
    pub fn press(&mut self, timestamp_ms: u64) {
        self.pressed_at = Some(timestamp_ms);
    }

    #[inline]
    pub const fn pressed_at(&self) -> Option<u64> {
        self.pressed_at
    }

    #[inline]
    pub fn is_pressed(&self, now_ms: u64) -> bool {
        self.is_pressed_for(now_ms, 150)
    }

    #[inline]
    pub fn is_pressed_for(&self, now_ms: u64, active_window_ms: u32) -> bool {
        match self.pressed_at {
            Some(t) => now_ms.saturating_sub(t) < active_window_ms as u64,
            None => false,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.pressed_at = None;
    }
}

/// Manages the Mr. Robot keypad state.
#[derive(Clone, Debug)]
pub struct MrRobotKeypad {
    buttons: [TerminalButton; 12],
    config: TerminalConfig,
    entered_pin: [char; 6],
    pin_len: usize,
}

impl Default for MrRobotKeypad {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MrRobotKeypad {
    #[inline]
    pub fn new() -> Self {
        Self::with_config(TerminalConfig::new())
    }

    #[inline]
    pub fn with_config(config: TerminalConfig) -> Self {
        let buttons = [
            TerminalButton::new('1'),
            TerminalButton::new('2'),
            TerminalButton::new('3'),
            TerminalButton::new('4'),
            TerminalButton::new('5'),
            TerminalButton::new('6'),
            TerminalButton::new('7'),
            TerminalButton::new('8'),
            TerminalButton::new('9'),
            TerminalButton::new('C'),
            TerminalButton::new('0'),
            TerminalButton::new('E'),
        ];

        Self {
            buttons,
            config,
            entered_pin: ['\0'; 6],
            pin_len: 0,
        }
    }

    #[inline]
    pub const fn config(&self) -> &TerminalConfig {
        &self.config
    }

    #[inline]
    pub fn press(&mut self, label: char, timestamp_ms: u64) -> bool {
        if let Some(button) = self.button_mut(label) {
            button.press(timestamp_ms);

            match label {
                'C' => {
                    self.clear_pin();
                }
                'E' => {
                    // Handled externally (verification)
                }
                '0'..='9' => {
                    if self.pin_len < 6 {
                        self.entered_pin[self.pin_len] = label;
                        self.pin_len += 1;
                    }
                }
                _ => {}
            }
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn button(&self, label: char) -> Option<&TerminalButton> {
        self.buttons.iter().find(|b| b.label == label)
    }

    #[inline]
    pub fn button_mut(&mut self, label: char) -> Option<&mut TerminalButton> {
        self.buttons.iter_mut().find(|b| b.label == label)
    }

    #[inline]
    pub const fn buttons(&self) -> &[TerminalButton; 12] {
        &self.buttons
    }

    #[inline]
    pub fn reset(&mut self) {
        self.clear_pin();
        for button in &mut self.buttons {
            button.reset();
        }
    }

    #[inline]
    pub fn clear_pin(&mut self) {
        self.pin_len = 0;
    }

    #[inline]
    pub fn entered_pin(&self) -> &[char] {
        &self.entered_pin[..self.pin_len]
    }

    #[inline]
    pub const fn demo_pin_for_cycle(cycle: usize) -> &'static [char; 4] {
        const PINS: [[char; 4]; 4] = [
            ['1', '2', '3', '4'],
            ['0', '0', '0', '0'],
            ['9', '9', '9', '9'],
            ['1', '3', '3', '7'],
        ];
        &PINS[cycle % PINS.len()]
    }

    #[inline]
    pub fn demo_event_delay_ms(&self, event_index: usize, seed: u64) -> u32 {
        self.config
            .typing_delay_for(seed.wrapping_add(event_index as u64))
    }
}

impl Index<usize> for MrRobotKeypad {
    type Output = TerminalButton;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.buttons[index]
    }
}

impl IndexMut<usize> for MrRobotKeypad {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buttons[index]
    }
}

/// Mr. Robot terminal color constants.
///
/// These match the JavaScript and Python implementations exactly.
pub mod colors {
    /// Deep black background: #0a0a0a / (10, 10, 10)
    pub const DEEP_BLACK: (u8, u8, u8) = (10, 10, 10);
    /// Primary phosphor green: #00FF41 / (0, 255, 65)
    pub const PHOSPHOR_GREEN: (u8, u8, u8) = (0, 255, 65);
    /// Dim green for secondary text: #008020 / (0, 128, 32)
    pub const DIM_GREEN: (u8, u8, u8) = (0, 128, 32);
    /// Terminal background: #001400 / (0, 20, 0)
    pub const TERMINAL_BG: (u8, u8, u8) = (0, 20, 0);
    /// Cyan accent: #7aecff / (122, 236, 255)
    pub const CYAN: (u8, u8, u8) = (122, 236, 255);
    /// Teal border: #1e505f / (30, 80, 95)
    pub const TEAL: (u8, u8, u8) = (30, 80, 95);
    /// Error red: #ff3333 / (255, 51, 51)
    pub const ERROR_RED: (u8, u8, u8) = (255, 51, 51);
    /// Success green: #00ff00 / (0, 255, 0)
    pub const SUCCESS_GREEN: (u8, u8, u8) = (0, 255, 0);
    /// Warning amber: #ffaa00 / (255, 170, 0)
    pub const WARNING_AMBER: (u8, u8, u8) = (255, 170, 0);
    /// Chromatic red edge channel used for active/hovered buttons.
    pub const CHROMATIC_RED: (u8, u8, u8) = (255, 60, 70);
    /// Chromatic blue edge channel used for active/hovered buttons.
    pub const CHROMATIC_BLUE: (u8, u8, u8) = (120, 210, 255);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_initial_state() {
        let button = TerminalButton::new('5');
        assert_eq!(button.label, '5');
        assert!(button.pressed_at().is_none());
        assert!(!button.is_pressed(0));
    }

    #[test]
    fn test_button_press() {
        let mut button = TerminalButton::new('5');
        button.press(1000);
        assert_eq!(button.pressed_at(), Some(1000));
        assert!(button.is_pressed(1000));
        assert!(button.is_pressed(1100)); // Within 150ms window
        assert!(!button.is_pressed(1200)); // Beyond 150ms window
    }

    #[test]
    fn test_button_reset() {
        let mut button = TerminalButton::new('5');
        button.press(1000);
        assert!(button.pressed_at().is_some());
        button.reset();
        assert!(button.pressed_at().is_none());
    }

    #[test]
    fn test_keypad_creation() {
        let keypad = MrRobotKeypad::new();
        assert_eq!(keypad[0].label, '1');
        assert_eq!(keypad[9].label, 'C');
        assert_eq!(keypad[10].label, '0');
        assert_eq!(keypad[11].label, 'E');
    }

    #[test]
    fn test_keypad_digit_entry() {
        let mut keypad = MrRobotKeypad::new();
        assert!(keypad.press('1', 100));
        assert!(keypad.press('2', 200));
        assert!(keypad.press('3', 300));
        assert!(keypad.press('4', 400));

        let pin = keypad.entered_pin();
        assert_eq!(pin, &['1', '2', '3', '4']);
    }

    #[test]
    fn test_keypad_max_pin_length() {
        let mut keypad = MrRobotKeypad::new();
        for i in 0..8 {
            keypad.press(char::from(b'0' + (i % 10)), i as u64 * 100);
        }
        // Should be capped at 6 digits
        assert_eq!(keypad.entered_pin().len(), 6);
    }

    #[test]
    fn test_keypad_clear() {
        let mut keypad = MrRobotKeypad::new();
        keypad.press('1', 100);
        keypad.press('2', 200);
        assert_eq!(keypad.entered_pin().len(), 2);

        keypad.press('C', 300);
        assert_eq!(keypad.entered_pin().len(), 0);
    }

    #[test]
    fn test_keypad_invalid_button() {
        let mut keypad = MrRobotKeypad::new();
        assert!(!keypad.press('X', 100));
    }

    #[test]
    fn test_config_defaults() {
        let config = TerminalConfig::new();
        assert_eq!(config.cursor_blink_ms, 530);
        assert_eq!(config.typing_delay_ms, 30);
        assert_eq!(config.typing_variance_ms, 15);
        assert_eq!(config.button_active_ms, 150);
        assert_eq!(config.grain_update_interval_ms, 100);
        assert_eq!(config.chromatic_offset_px, 2);
    }

    #[test]
    fn test_config_builders_for_visual_effects() {
        let config = TerminalConfig::new()
            .button_active_ms(180)
            .grain_update_interval_ms(120)
            .chromatic_offset_px(3);

        assert_eq!(config.button_active_ms, 180);
        assert_eq!(config.grain_update_interval_ms, 120);
        assert_eq!(config.chromatic_offset_px, 3);
    }

    #[test]
    fn test_typing_delay_stays_within_bounds() {
        let config = TerminalConfig::new();
        let (min, max) = config.typing_delay_bounds();

        for i in 0..128 {
            let delay = config.typing_delay_for(i);
            assert!(delay >= min);
            assert!(delay <= max);
        }
    }

    #[test]
    fn test_button_custom_active_window() {
        let mut button = TerminalButton::new('7');
        button.press(1_000);

        assert!(button.is_pressed_for(1_160, 200));
        assert!(!button.is_pressed_for(1_250, 200));
    }

    #[test]
    fn test_demo_pin_rotation() {
        assert_eq!(MrRobotKeypad::demo_pin_for_cycle(0), &['1', '2', '3', '4']);
        assert_eq!(MrRobotKeypad::demo_pin_for_cycle(3), &['1', '3', '3', '7']);
        assert_eq!(MrRobotKeypad::demo_pin_for_cycle(4), &['1', '2', '3', '4']);
    }

    #[test]
    fn test_demo_event_delay_uses_typing_timing() {
        let keypad = MrRobotKeypad::new();
        let (min, max) = keypad.config().typing_delay_bounds();
        let delay = keypad.demo_event_delay_ms(2, 42);

        assert!(delay >= min);
        assert!(delay <= max);
    }

    #[test]
    fn test_grain_cache_refresh_window() {
        let mut cache = GrainCacheState::new();
        assert!(cache.should_refresh(1000, 100));

        cache.mark_updated(1000);
        assert!(!cache.should_refresh(1050, 100));
        assert!(cache.should_refresh(1100, 100));
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(colors::DEEP_BLACK, (10, 10, 10));
        assert_eq!(colors::PHOSPHOR_GREEN, (0, 255, 65));
        assert_eq!(colors::DIM_GREEN, (0, 128, 32));
        assert_eq!(colors::TERMINAL_BG, (0, 20, 0));
    }
}
