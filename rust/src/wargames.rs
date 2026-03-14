use core::cmp;

pub const BUFFER_LINE_CAP: usize = 96;
pub const BUFFER_CAPACITY: usize = 80;
pub const INPUT_CAPACITY: usize = 64;

pub const MENU_LINES: [&str; 8] = [
    "GAMES LIST:",
    "",
    "1. GLOBAL THERMONUCLEAR WAR",
    "2. POKER",
    "3. CHESS",
    "4. FIGHTER COMBAT",
    "",
    "SELECT: _",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WargamesConfig {
    pub type_delay_ms: u32,
    pub type_variance_ms: u32,
    pub newline_pause_ms: u32,
    pub cursor_blink_min_ms: u32,
    pub cursor_blink_max_ms: u32,
}

impl WargamesConfig {
    #[inline]
    pub const fn new() -> Self {
        Self {
            type_delay_ms: 30,
            type_variance_ms: 15,
            newline_pause_ms: 170,
            cursor_blink_min_ms: 530,
            cursor_blink_max_ms: 1000,
        }
    }
}

impl Default for WargamesConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineStyle {
    Normal,
    Dim,
    Highlight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalLine {
    bytes: [u8; BUFFER_LINE_CAP],
    len: usize,
    style: LineStyle,
}

impl TerminalLine {
    #[inline]
    pub const fn new() -> Self {
        Self {
            bytes: [0; BUFFER_LINE_CAP],
            len: 0,
            style: LineStyle::Normal,
        }
    }

    #[inline]
    pub const fn style(&self) -> LineStyle {
        self.style
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    #[inline]
    fn reset_with(&mut self, text: &str, style: LineStyle) {
        self.len = 0;
        self.style = style;
        let bytes = text.as_bytes();
        let max = cmp::min(bytes.len(), BUFFER_LINE_CAP);
        self.bytes[..max].copy_from_slice(&bytes[..max]);
        self.len = max;
    }

    #[inline]
    fn append_byte(&mut self, byte: u8) {
        if self.len < BUFFER_LINE_CAP {
            self.bytes[self.len] = byte;
            self.len += 1;
        }
    }
}

impl Default for TerminalLine {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct TerminalBuffer {
    lines: [TerminalLine; BUFFER_CAPACITY],
    start: usize,
    len: usize,
}

impl TerminalBuffer {
    #[inline]
    pub const fn new() -> Self {
        Self {
            lines: [TerminalLine::new(); BUFFER_CAPACITY],
            start: 0,
            len: 0,
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.start = 0;
        self.len = 0;
    }

    #[inline]
    pub fn push(&mut self, text: &str, style: LineStyle) {
        let idx = if self.len < BUFFER_CAPACITY {
            let idx = (self.start + self.len) % BUFFER_CAPACITY;
            self.len += 1;
            idx
        } else {
            let idx = self.start;
            self.start = (self.start + 1) % BUFFER_CAPACITY;
            idx
        };

        self.lines[idx].reset_with(text, style);
    }

    #[inline]
    pub fn line(&self, visible_index: usize) -> Option<&TerminalLine> {
        if visible_index >= self.len {
            return None;
        }
        let idx = (self.start + visible_index) % BUFFER_CAPACITY;
        Some(&self.lines[idx])
    }

    #[inline]
    fn push_empty(&mut self, style: LineStyle) -> usize {
        let idx = if self.len < BUFFER_CAPACITY {
            let idx = (self.start + self.len) % BUFFER_CAPACITY;
            self.len += 1;
            idx
        } else {
            let idx = self.start;
            self.start = (self.start + 1) % BUFFER_CAPACITY;
            idx
        };
        self.lines[idx].reset_with("", style);
        idx
    }

    #[inline]
    fn line_mut(&mut self, absolute_index: usize) -> &mut TerminalLine {
        &mut self.lines[absolute_index]
    }
}

impl Default for TerminalBuffer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CursorState {
    pub visible: bool,
    pub next_toggle_at_ms: u64,
}

impl CursorState {
    #[inline]
    pub const fn new() -> Self {
        Self {
            visible: true,
            next_toggle_at_ms: 0,
        }
    }
}

impl Default for CursorState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
struct TypeQueueItem {
    bytes: [u8; BUFFER_LINE_CAP],
    len: usize,
    style: LineStyle,
}

impl TypeQueueItem {
    #[inline]
    const fn new() -> Self {
        Self {
            bytes: [0; BUFFER_LINE_CAP],
            len: 0,
            style: LineStyle::Normal,
        }
    }

    #[inline]
    fn set(&mut self, text: &str, style: LineStyle) {
        self.len = 0;
        self.style = style;
        let bytes = text.as_bytes();
        let max = cmp::min(bytes.len(), BUFFER_LINE_CAP);
        self.bytes[..max].copy_from_slice(&bytes[..max]);
        self.len = max;
    }
}

#[derive(Clone, Debug)]
pub struct WargamesTerminal {
    config: WargamesConfig,
    pub buffer: TerminalBuffer,
    pub cursor: CursorState,
    pub input: [u8; INPUT_CAPACITY],
    input_len: usize,
    queued: [TypeQueueItem; 16],
    queued_len: usize,
    active: Option<TypeQueueItem>,
    active_index: usize,
    active_line_idx: Option<usize>,
    next_type_at_ms: u64,
    seed: u64,
}

impl WargamesTerminal {
    #[inline]
    pub fn new(config: WargamesConfig) -> Self {
        Self {
            config,
            buffer: TerminalBuffer::new(),
            cursor: CursorState::new(),
            input: [0; INPUT_CAPACITY],
            input_len: 0,
            queued: [TypeQueueItem::new(); 16],
            queued_len: 0,
            active: None,
            active_index: 0,
            active_line_idx: None,
            next_type_at_ms: 0,
            seed: 0x9E37_79B9_7F4A_7C15,
        }
    }

    #[inline]
    pub fn config(&self) -> WargamesConfig {
        self.config
    }

    #[inline]
    pub const fn input_len(&self) -> usize {
        self.input_len
    }

    #[inline]
    pub fn input_bytes(&self) -> &[u8] {
        &self.input[..self.input_len]
    }

    #[inline]
    pub fn set_input(&mut self, text: &str) {
        let bytes = text.as_bytes();
        let max = cmp::min(bytes.len(), INPUT_CAPACITY);
        self.input[..max].copy_from_slice(&bytes[..max]);
        self.input_len = max;
    }

    #[inline]
    pub fn clear_input(&mut self) {
        self.input_len = 0;
    }

    #[inline]
    pub fn queue_line(&mut self, text: &str, style: LineStyle) -> bool {
        if self.queued_len >= self.queued.len() {
            return false;
        }
        self.queued[self.queued_len].set(text, style);
        self.queued_len += 1;
        true
    }

    #[inline]
    pub fn queue_menu(&mut self) {
        for line in MENU_LINES {
            let _ = self.queue_line(line, LineStyle::Normal);
        }
    }

    #[inline]
    pub fn boot_sequence(&mut self) {
        let _ = self.queue_line("", LineStyle::Dim);
        let _ = self.queue_line("NORAD REMOTE STRATEGIC TERMINAL", LineStyle::Highlight);
        let _ = self.queue_line("IMSAI 8080 CONNECTED", LineStyle::Dim);
        let _ = self.queue_line("SECURITY LEVEL: SIMULATION / TRAINING", LineStyle::Dim);
        let _ = self.queue_line("TYPE HELP FOR COMMAND LIST", LineStyle::Dim);
        let _ = self.queue_line("", LineStyle::Dim);
        self.queue_menu();
    }

    #[inline]
    pub fn is_typing(&self) -> bool {
        self.active.is_some() || self.queued_len > 0
    }

    #[inline]
    pub fn tick(&mut self, now_ms: u64) {
        if self.cursor.next_toggle_at_ms == 0 {
            self.cursor.next_toggle_at_ms = now_ms + self.next_cursor_delay_ms() as u64;
        }

        if now_ms >= self.cursor.next_toggle_at_ms {
            self.cursor.visible = !self.cursor.visible;
            self.cursor.next_toggle_at_ms = now_ms + self.next_cursor_delay_ms() as u64;
        }

        if self.active.is_none() && self.queued_len > 0 {
            let first = self.queued[0];
            for i in 1..self.queued_len {
                self.queued[i - 1] = self.queued[i];
            }
            self.queued_len -= 1;
            self.active = Some(first);
            self.active_index = 0;
            let idx = self.buffer.push_empty(first.style);
            self.active_line_idx = Some(idx);
            self.next_type_at_ms = now_ms + self.next_type_delay_ms() as u64;
        }

        if let Some(active) = self.active {
            if now_ms >= self.next_type_at_ms {
                if self.active_index < active.len {
                    if let Some(line_idx) = self.active_line_idx {
                        let byte = active.bytes[self.active_index];
                        self.buffer.line_mut(line_idx).append_byte(byte);
                    }
                    self.active_index += 1;
                    self.next_type_at_ms = now_ms + self.next_type_delay_ms() as u64;
                } else {
                    self.active = None;
                    self.active_line_idx = None;
                    self.next_type_at_ms = now_ms + self.config.newline_pause_ms as u64;
                }
            }
        }
    }

    #[inline]
    fn random_u32(&mut self) -> u32 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        (self.seed.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    #[inline]
    fn next_type_delay_ms(&mut self) -> u32 {
        let variance = self.config.type_variance_ms;
        if variance == 0 {
            return self.config.type_delay_ms;
        }
        let spread = variance.saturating_mul(2).saturating_add(1);
        let bucket = self.random_u32() % spread;
        let jitter = bucket as i32 - variance as i32;
        let base = self.config.type_delay_ms as i32;
        cmp::max(8, base + jitter) as u32
    }

    #[inline]
    fn next_cursor_delay_ms(&mut self) -> u32 {
        let min_ms = self.config.cursor_blink_min_ms;
        let max_ms = self.config.cursor_blink_max_ms;
        if max_ms <= min_ms {
            return min_ms;
        }
        let span = max_ms - min_ms + 1;
        min_ms + (self.random_u32() % span)
    }
}

pub mod colors {
    pub const PHOSPHOR_GREEN: (u8, u8, u8) = (0, 170, 0);
    pub const BRIGHT_GREEN: (u8, u8, u8) = (24, 230, 153);
    pub const DIM_GREEN: (u8, u8, u8) = (16, 84, 14);
    pub const BACKGROUND: (u8, u8, u8) = (5, 26, 5);
    pub const BLACK: (u8, u8, u8) = (0, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_match_spec() {
        let cfg = WargamesConfig::new();
        assert_eq!(cfg.type_delay_ms, 30);
        assert_eq!(cfg.type_variance_ms, 15);
        assert_eq!(cfg.cursor_blink_min_ms, 530);
        assert_eq!(cfg.cursor_blink_max_ms, 1000);
    }

    #[test]
    fn terminal_buffer_ring_behavior() {
        let mut buffer = TerminalBuffer::new();
        for i in 0..(BUFFER_CAPACITY + 4) {
            let text = if i % 2 == 0 { "A" } else { "B" };
            buffer.push(text, LineStyle::Normal);
        }

        assert_eq!(buffer.len(), BUFFER_CAPACITY);
        assert_eq!(buffer.line(0).unwrap().as_bytes(), b"A");
        assert_eq!(buffer.line(BUFFER_CAPACITY - 1).unwrap().as_bytes(), b"B");
    }

    #[test]
    fn typing_writes_into_buffer() {
        let mut terminal = WargamesTerminal::new(WargamesConfig::new());
        assert!(terminal.queue_line("HELLO", LineStyle::Highlight));
        for t in 0..3000 {
            terminal.tick(t);
        }

        assert!(!terminal.buffer.is_empty());
        let line = terminal.buffer.line(0).unwrap();
        assert_eq!(line.as_bytes(), b"HELLO");
        assert_eq!(line.style(), LineStyle::Highlight);
    }

    #[test]
    fn color_constants_match_spec() {
        assert_eq!(colors::PHOSPHOR_GREEN, (0, 170, 0));
        assert_eq!(colors::BRIGHT_GREEN, (24, 230, 153));
        assert_eq!(colors::DIM_GREEN, (16, 84, 14));
        assert_eq!(colors::BACKGROUND, (5, 26, 5));
        assert_eq!(colors::BLACK, (0, 0, 0));
    }
}
