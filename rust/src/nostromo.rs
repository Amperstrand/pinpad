use core::ops::{Index, IndexMut};
use embedded_graphics::pixelcolor::Rgb888;

pub const PANEL_BACKGROUND: Rgb888 = Rgb888::new(232, 230, 225);
pub const BUTTON_BLUE: Rgb888 = Rgb888::new(0, 90, 156);
pub const BUTTON_RED: Rgb888 = Rgb888::new(176, 0, 0);
pub const INDICATOR_AMBER: Rgb888 = Rgb888::new(255, 176, 0);
pub const INDICATOR_GREEN: Rgb888 = Rgb888::new(0, 255, 65);
pub const TEXT_COLOR: Rgb888 = Rgb888::new(26, 26, 26);
pub const WEAR_COLOR: Rgb888 = Rgb888::new(58, 53, 48);
pub const NOSTROMO_DEMO_DIGITS: usize = 4;
const DEMO_EVENT_CAPACITY: usize = NOSTROMO_DEMO_DIGITS + 1;

pub const NOSTROMO_BUTTON_LABELS: [char; 12] =
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E'];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NostromoStatus {
    Ready,
    Entering,
    Busy,
    Granted,
    Cleared,
}

#[derive(Clone, Copy, Debug)]
pub struct NostromoConfig {
    pub submit_delay_ms: u32,
    pub granted_hold_ms: u32,
    pub button_press_ms: u32,
    pub demo_interval_ms: u32,
    pub demo_digit_spacing_ms: u32,
    pub max_code_len: u8,
}

impl Default for NostromoConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl NostromoConfig {
    pub const fn new() -> Self {
        Self {
            submit_delay_ms: 700,
            granted_hold_ms: 900,
            button_press_ms: 120,
            demo_interval_ms: 2200,
            demo_digit_spacing_ms: 110,
            max_code_len: 6,
        }
    }

    pub const fn submit_delay_ms(mut self, ms: u32) -> Self {
        self.submit_delay_ms = ms;
        self
    }

    pub const fn button_press_ms(mut self, ms: u32) -> Self {
        self.button_press_ms = ms;
        self
    }

    pub const fn granted_hold_ms(mut self, ms: u32) -> Self {
        self.granted_hold_ms = ms;
        self
    }

    pub const fn demo_interval_ms(mut self, ms: u32) -> Self {
        self.demo_interval_ms = ms;
        self
    }

    pub const fn demo_digit_spacing_ms(mut self, ms: u32) -> Self {
        self.demo_digit_spacing_ms = ms;
        self
    }

    pub const fn max_code_len(mut self, len: u8) -> Self {
        self.max_code_len = len;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NostromoButton {
    pub label: char,
    pub color: Rgb888,
    pressed_at: Option<u64>,
}

impl NostromoButton {
    pub const fn new(label: char, color: Rgb888) -> Self {
        Self {
            label,
            color,
            pressed_at: None,
        }
    }

    pub fn press(&mut self, now_ms: u64) {
        self.pressed_at = Some(now_ms);
    }

    pub const fn pressed_at(&self) -> Option<u64> {
        self.pressed_at
    }

    pub fn is_pressed(&self, now_ms: u64, hold_ms: u32) -> bool {
        match self.pressed_at {
            Some(pressed) if now_ms >= pressed => now_ms - pressed < hold_ms as u64,
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.pressed_at = None;
    }
}

#[derive(Clone, Copy, Debug)]
struct DemoEvent {
    at_ms: u64,
    label: char,
}

impl DemoEvent {
    const EMPTY: Self = Self {
        at_ms: 0,
        label: '\0',
    };

    const fn new(at_ms: u64, label: char) -> Self {
        Self { at_ms, label }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NostromoKeypad {
    buttons: [NostromoButton; 12],
    code: [char; 6],
    code_len: u8,
    last_submitted: [char; 6],
    last_submitted_len: u8,
    status: NostromoStatus,
    busy_until_ms: u64,
    granted_until_ms: u64,
    demo_enabled: bool,
    next_demo_ms: u64,
    demo_state: u32,
    pending_demo: [DemoEvent; DEMO_EVENT_CAPACITY],
    pending_demo_len: u8,
    config: NostromoConfig,
}

impl Default for NostromoKeypad {
    fn default() -> Self {
        Self::new()
    }
}

impl NostromoKeypad {
    pub fn new() -> Self {
        Self::with_config(NostromoConfig::new())
    }

    pub fn with_config(config: NostromoConfig) -> Self {
        Self {
            buttons: [
                NostromoButton::new('1', BUTTON_BLUE),
                NostromoButton::new('2', BUTTON_BLUE),
                NostromoButton::new('3', BUTTON_BLUE),
                NostromoButton::new('4', BUTTON_BLUE),
                NostromoButton::new('5', BUTTON_BLUE),
                NostromoButton::new('6', BUTTON_BLUE),
                NostromoButton::new('7', BUTTON_BLUE),
                NostromoButton::new('8', BUTTON_BLUE),
                NostromoButton::new('9', BUTTON_BLUE),
                NostromoButton::new('C', BUTTON_RED),
                NostromoButton::new('0', BUTTON_BLUE),
                NostromoButton::new('E', BUTTON_BLUE),
            ],
            code: ['\0'; 6],
            code_len: 0,
            last_submitted: ['\0'; 6],
            last_submitted_len: 0,
            status: NostromoStatus::Ready,
            busy_until_ms: 0,
            granted_until_ms: 0,
            demo_enabled: false,
            next_demo_ms: 0,
            demo_state: 0x1979_A11E,
            pending_demo: [DemoEvent::EMPTY; DEMO_EVENT_CAPACITY],
            pending_demo_len: 0,
            config,
        }
    }

    pub const fn config(&self) -> &NostromoConfig {
        &self.config
    }

    pub const fn status(&self) -> NostromoStatus {
        self.status
    }

    pub fn is_busy(&self, now_ms: u64) -> bool {
        now_ms < self.busy_until_ms
    }

    pub fn indicators(&self, now_ms: u64) -> (bool, bool) {
        let amber = self.is_busy(now_ms);
        (amber, !amber)
    }

    pub fn indicator_glow(&self, now_ms: u64) -> (u8, u8) {
        let (amber, green) = self.indicators(now_ms);
        let amber_glow = if amber { 255 } else { 56 };
        let green_glow = if green { 255 } else { 56 };
        (amber_glow, green_glow)
    }

    pub fn button_depth_px(&self, label: char, now_ms: u64) -> Option<u8> {
        self.button(label).map(|button| {
            if button.is_pressed(now_ms, self.config.button_press_ms) {
                4
            } else {
                0
            }
        })
    }

    pub fn set_demo_mode(&mut self, enabled: bool, now_ms: u64) {
        self.demo_enabled = enabled;
        self.next_demo_ms = now_ms;
        self.clear_demo_events();
    }

    pub const fn demo_mode(&self) -> bool {
        self.demo_enabled
    }

    pub fn keyboard_to_label(key: char) -> Option<char> {
        match key {
            '0'..='9' => Some(key),
            'c' | 'C' => Some('C'),
            'e' | 'E' | '\n' | '\r' => Some('E'),
            '\u{8}' | '\u{7f}' | '\u{1b}' => Some('C'),
            _ => None,
        }
    }

    pub fn update(&mut self, now_ms: u64) {
        if self.demo_enabled && now_ms >= self.next_demo_ms && !self.is_busy(now_ms) {
            self.clear_code();
            self.queue_demo_sequence(now_ms);
            self.next_demo_ms = now_ms + self.config.demo_interval_ms as u64;
            self.status = NostromoStatus::Entering;
        }

        self.drain_demo_events(now_ms);

        if !self.is_busy(now_ms) && self.status == NostromoStatus::Busy {
            self.status = NostromoStatus::Granted;
            self.granted_until_ms = now_ms + self.config.granted_hold_ms as u64;
            return;
        }

        if self.status == NostromoStatus::Granted && now_ms >= self.granted_until_ms {
            self.status = NostromoStatus::Ready;
        }
    }

    pub fn press(&mut self, label: char, now_ms: u64) -> bool {
        if self.is_busy(now_ms) {
            return false;
        }

        if let Some(button) = self.button_mut(label) {
            button.press(now_ms);
        } else {
            return false;
        }

        match label {
            'C' => {
                self.clear_code();
                self.status = NostromoStatus::Cleared;
                self.granted_until_ms = 0;
            }
            'E' => self.submit(now_ms),
            '0'..='9' => {
                if (self.code_len as usize) < self.max_code_len() {
                    self.code[self.code_len as usize] = label;
                    self.code_len += 1;
                    self.status = NostromoStatus::Entering;
                }
            }
            _ => return false,
        }

        true
    }

    pub fn clear_code(&mut self) {
        self.code = ['\0'; 6];
        self.code_len = 0;
    }

    pub fn submit(&mut self, now_ms: u64) {
        if self.code_len == 0 {
            return;
        }
        self.last_submitted = ['\0'; 6];
        for i in 0..self.code_len as usize {
            self.last_submitted[i] = self.code[i];
        }
        self.last_submitted_len = self.code_len;
        self.clear_code();
        self.status = NostromoStatus::Busy;
        self.busy_until_ms = now_ms + self.config.submit_delay_ms as u64;
        self.granted_until_ms = 0;
    }

    pub const fn code_len(&self) -> u8 {
        self.code_len
    }

    pub const fn code_chars(&self) -> &[char; 6] {
        &self.code
    }

    pub const fn last_submitted_len(&self) -> u8 {
        self.last_submitted_len
    }

    pub const fn last_submitted_chars(&self) -> &[char; 6] {
        &self.last_submitted
    }

    pub const fn buttons(&self) -> &[NostromoButton; 12] {
        &self.buttons
    }

    pub fn button(&self, label: char) -> Option<&NostromoButton> {
        self.buttons.iter().find(|b| b.label == label)
    }

    pub fn button_mut(&mut self, label: char) -> Option<&mut NostromoButton> {
        self.buttons.iter_mut().find(|b| b.label == label)
    }

    pub fn reset(&mut self) {
        for button in &mut self.buttons {
            button.reset();
        }
        self.clear_code();
        self.clear_demo_events();
        self.last_submitted = ['\0'; 6];
        self.last_submitted_len = 0;
        self.status = NostromoStatus::Ready;
        self.busy_until_ms = 0;
        self.granted_until_ms = 0;
        self.demo_enabled = false;
        self.next_demo_ms = 0;
        self.demo_state = 0x1979_A11E;
    }

    fn max_code_len(&self) -> usize {
        (self.config.max_code_len as usize).min(6)
    }

    fn next_demo_digit(&mut self) -> char {
        self.demo_state = self
            .demo_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let digit = ((self.demo_state >> 24) % 10) as u8;
        char::from(b'0' + digit)
    }

    fn queue_demo_sequence(&mut self, now_ms: u64) {
        for idx in 0..NOSTROMO_DEMO_DIGITS {
            let when = now_ms + (idx as u64 * self.config.demo_digit_spacing_ms as u64);
            let digit = self.next_demo_digit();
            self.enqueue_demo_event(when, digit);
        }

        let submit_at =
            now_ms + (NOSTROMO_DEMO_DIGITS as u64 * self.config.demo_digit_spacing_ms as u64) + 40;
        self.enqueue_demo_event(submit_at, 'E');
    }

    fn enqueue_demo_event(&mut self, at_ms: u64, label: char) {
        let len = self.pending_demo_len as usize;
        if len < self.pending_demo.len() {
            self.pending_demo[len] = DemoEvent::new(at_ms, label);
            self.pending_demo_len += 1;
        }
    }

    fn pop_ready_demo_event(&mut self, now_ms: u64) -> Option<char> {
        let len = self.pending_demo_len as usize;
        if len == 0 {
            return None;
        }

        let mut idx: Option<usize> = None;
        for i in 0..len {
            if self.pending_demo[i].at_ms > now_ms {
                continue;
            }
            match idx {
                Some(current) if self.pending_demo[i].at_ms >= self.pending_demo[current].at_ms => {
                }
                _ => idx = Some(i),
            }
        }

        let remove_idx = idx?;
        let label = self.pending_demo[remove_idx].label;
        let last = len - 1;
        self.pending_demo[remove_idx] = self.pending_demo[last];
        self.pending_demo[last] = DemoEvent::EMPTY;
        self.pending_demo_len -= 1;
        Some(label)
    }

    fn drain_demo_events(&mut self, now_ms: u64) {
        while let Some(label) = self.pop_ready_demo_event(now_ms) {
            let _ = self.press(label, now_ms);
        }
    }

    fn clear_demo_events(&mut self) {
        for event in &mut self.pending_demo {
            *event = DemoEvent::EMPTY;
        }
        self.pending_demo_len = 0;
    }
}

impl Index<usize> for NostromoKeypad {
    type Output = NostromoButton;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buttons[index]
    }
}

impl IndexMut<usize> for NostromoKeypad {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buttons[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_match_spec() {
        assert_eq!(PANEL_BACKGROUND, Rgb888::new(232, 230, 225));
        assert_eq!(BUTTON_BLUE, Rgb888::new(0, 90, 156));
        assert_eq!(BUTTON_RED, Rgb888::new(176, 0, 0));
        assert_eq!(INDICATOR_AMBER, Rgb888::new(255, 176, 0));
        assert_eq!(INDICATOR_GREEN, Rgb888::new(0, 255, 65));
        assert_eq!(TEXT_COLOR, Rgb888::new(26, 26, 26));
        assert_eq!(WEAR_COLOR, Rgb888::new(58, 53, 48));
    }

    #[test]
    fn keypad_has_12_buttons() {
        let keypad = NostromoKeypad::new();
        assert_eq!(keypad.buttons().len(), 12);
        assert_eq!(keypad[9].label, 'C');
        assert_eq!(keypad[11].label, 'E');
    }

    #[test]
    fn keypad_input_flow() {
        let mut keypad = NostromoKeypad::new();
        assert!(keypad.press('1', 100));
        assert!(keypad.press('2', 120));
        assert_eq!(keypad.code_len(), 2);

        assert!(keypad.press('E', 140));
        assert_eq!(keypad.status(), NostromoStatus::Busy);
        assert_eq!(keypad.code_len(), 0);
        assert_eq!(keypad.last_submitted_len(), 2);

        keypad.update(1000);
        assert_eq!(keypad.status(), NostromoStatus::Granted);

        keypad.update(2200);
        assert_eq!(keypad.status(), NostromoStatus::Ready);
    }

    #[test]
    fn keyboard_mapping_matches_controls() {
        assert_eq!(NostromoKeypad::keyboard_to_label('7'), Some('7'));
        assert_eq!(NostromoKeypad::keyboard_to_label('c'), Some('C'));
        assert_eq!(NostromoKeypad::keyboard_to_label('\n'), Some('E'));
        assert_eq!(NostromoKeypad::keyboard_to_label('\u{1b}'), Some('C'));
        assert_eq!(NostromoKeypad::keyboard_to_label('x'), None);
    }

    #[test]
    fn demo_mode_generates_submission() {
        let mut keypad = NostromoKeypad::new();
        keypad.set_demo_mode(true, 0);

        keypad.update(0);
        keypad.update(600);
        assert_eq!(keypad.status(), NostromoStatus::Busy);

        keypad.update(1400);
        assert_eq!(keypad.status(), NostromoStatus::Granted);
        assert_eq!(keypad.last_submitted_len(), NOSTROMO_DEMO_DIGITS as u8);
    }
}
