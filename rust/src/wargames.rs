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
    pub selected_game: Option<u8>,
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
            selected_game: None,
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

    pub fn handle_command(&mut self) {
        let input_len = self.input_len;
        let input_str = core::str::from_utf8(&self.input[..input_len]).unwrap_or("");

        let mut upper_buf: [u8; INPUT_CAPACITY] = [0; INPUT_CAPACITY];
        let mut upper_len = 0;
        for b in input_str.trim().bytes() {
            if upper_len >= INPUT_CAPACITY {
                break;
            }
            upper_buf[upper_len] = if b >= b'a' && b <= b'z' { b - 32 } else { b };
            upper_len += 1;
        }
        self.clear_input();

        let upper = core::str::from_utf8(&upper_buf[..upper_len]).unwrap_or("");
        self.queue_command_echo(upper);

        if upper.is_empty() {
            let _ = self.queue_line("ENTER COMMAND. TRY HELP.", LineStyle::Dim);
            return;
        }

        if upper == "HELP" || upper == "?" || upper == "MAN" {
            self.queue_help();
            return;
        }

        if upper == "LIST" || upper == "GAMES" {
            self.queue_menu();
            return;
        }

        if upper.starts_with("SELECT ") {
            self.handle_select(&upper[7..]);
            return;
        }

        if upper.starts_with("PLAY") {
            let arg = if upper.len() > 4 {
                &upper[4..].trim()
            } else {
                ""
            };
            self.handle_play(arg);
            return;
        }

        if upper == "STATUS" {
            self.handle_status();
            return;
        }

        if upper == "CLEAR" || upper == "CLS" {
            self.buffer.clear();
            return;
        }

        self.queue_unknown_command(upper);
    }

    fn queue_command_echo(&mut self, command: &str) {
        let mut buf: [u8; BUFFER_LINE_CAP] = [0; BUFFER_LINE_CAP];
        let prefix = b"NORAD> ";
        let prefix_len = prefix.len();
        buf[..prefix_len].copy_from_slice(prefix);
        let cmd_bytes = command.as_bytes();
        let max_cmd = core::cmp::min(cmd_bytes.len(), BUFFER_LINE_CAP - prefix_len);
        buf[prefix_len..prefix_len + max_cmd].copy_from_slice(&cmd_bytes[..max_cmd]);
        if let Ok(s) = core::str::from_utf8(&buf[..prefix_len + max_cmd]) {
            let _ = self.queue_line(s, LineStyle::Normal);
        }
    }

    fn queue_unknown_command(&mut self, upper: &str) {
        let mut buf: [u8; BUFFER_LINE_CAP] = [0; BUFFER_LINE_CAP];
        let prefix = b"UNKNOWN COMMAND: ";
        let prefix_len = prefix.len();
        buf[..prefix_len].copy_from_slice(prefix);
        let cmd_bytes = upper.as_bytes();
        let max_cmd = core::cmp::min(cmd_bytes.len(), BUFFER_LINE_CAP - prefix_len);
        buf[prefix_len..prefix_len + max_cmd].copy_from_slice(&cmd_bytes[..max_cmd]);
        if let Ok(s) = core::str::from_utf8(&buf[..prefix_len + max_cmd]) {
            let _ = self.queue_line(s, LineStyle::Dim);
        }
        let _ = self.queue_line("TYPE HELP FOR COMMAND INDEX.", LineStyle::Dim);
    }

    fn queue_help(&mut self) {
        let _ = self.queue_line("AVAILABLE COMMANDS:", LineStyle::Dim);
        let _ = self.queue_line("  LIST              SHOW GAMES LIST", LineStyle::Dim);
        let _ = self.queue_line("  SELECT <1-4>      CHOOSE A GAME", LineStyle::Dim);
        let _ = self.queue_line("  PLAY <NAME|#>     START SELECTED GAME", LineStyle::Dim);
        let _ = self.queue_line("  STATUS            SHOW CURRENT TARGET", LineStyle::Dim);
        let _ = self.queue_line("  CLEAR             CLEAR TERMINAL", LineStyle::Dim);
        let _ = self.queue_line("  HELP              SHOW THIS MESSAGE", LineStyle::Dim);
    }

    fn handle_select(&mut self, token: &str) {
        let selection = self.parse_selection(token.trim());
        match selection {
            Some(n) => {
                self.selected_game = Some(n);
                let prefix = "SELECTION ACCEPTED: ";
                let mut buf: [u8; BUFFER_LINE_CAP] = [0; BUFFER_LINE_CAP];
                let prefix_bytes = prefix.as_bytes();
                let name_bytes = self.game_name(n).as_bytes();
                let total = core::cmp::min(prefix_bytes.len() + name_bytes.len(), BUFFER_LINE_CAP);
                buf[..prefix_bytes.len()].copy_from_slice(prefix_bytes);
                let name_len = total - prefix_bytes.len();
                buf[prefix_bytes.len()..total].copy_from_slice(&name_bytes[..name_len]);
                if let Ok(s) = core::str::from_utf8(&buf[..total]) {
                    let _ = self.queue_line(s, LineStyle::Highlight);
                }
                let _ = self.queue_line("TYPE PLAY TO EXECUTE.", LineStyle::Dim);
            }
            None => {
                let _ = self.queue_line("INVALID SELECTION. CHOOSE 1-4.", LineStyle::Dim);
            }
        }
    }

    fn handle_play(&mut self, arg: &str) {
        let selection = if arg.is_empty() {
            self.selected_game
        } else {
            self.parse_selection(arg)
        };

        match selection {
            Some(1) => {
                let _ = self.queue_line(
                    "SIMULATION BOOTSTRAP: GLOBAL THERMONUCLEAR WAR",
                    LineStyle::Highlight,
                );
                let _ = self.queue_line("CONNECTING TO WOPR...", LineStyle::Highlight);
                let _ = self.queue_line("GREETINGS PROFESSOR FALKEN.", LineStyle::Highlight);
                let _ = self.queue_line("SHALL WE PLAY A GAME?", LineStyle::Highlight);
            }
            Some(n) => {
                let name = self.game_name(n);
                let mut buf: [u8; BUFFER_LINE_CAP] = [0; BUFFER_LINE_CAP];
                let suffix = b" NOT INSTALLED ON THIS NODE.";
                let name_bytes = name.as_bytes();
                let total = core::cmp::min(name_bytes.len() + suffix.len(), BUFFER_LINE_CAP);
                buf[..name_bytes.len()].copy_from_slice(name_bytes);
                let suffix_len = total - name_bytes.len();
                buf[name_bytes.len()..total].copy_from_slice(&suffix[..suffix_len]);
                if let Ok(s) = core::str::from_utf8(&buf[..total]) {
                    let _ = self.queue_line(s, LineStyle::Dim);
                }
                let _ = self.queue_line(
                    "RECOMMENDED: GLOBAL THERMONUCLEAR WAR",
                    LineStyle::Highlight,
                );
            }
            None => {
                let _ = self.queue_line("NO GAME SELECTED. USE SELECT <1-4>.", LineStyle::Dim);
            }
        }
    }

    fn handle_status(&mut self) {
        match self.selected_game {
            Some(n) => {
                let prefix = "STATUS: READY / TARGET=";
                let mut buf: [u8; BUFFER_LINE_CAP] = [0; BUFFER_LINE_CAP];
                let prefix_bytes = prefix.as_bytes();
                let name_bytes = self.game_name(n).as_bytes();
                let total = core::cmp::min(prefix_bytes.len() + name_bytes.len(), BUFFER_LINE_CAP);
                buf[..prefix_bytes.len()].copy_from_slice(prefix_bytes);
                let name_len = total - prefix_bytes.len();
                buf[prefix_bytes.len()..total].copy_from_slice(&name_bytes[..name_len]);
                if let Ok(s) = core::str::from_utf8(&buf[..total]) {
                    let _ = self.queue_line(s, LineStyle::Highlight);
                }
            }
            None => {
                let _ = self.queue_line("STATUS: IDLE / NO ACTIVE GAME", LineStyle::Dim);
            }
        }
    }

    fn parse_selection(&self, token: &str) -> Option<u8> {
        if token == "1" || token.contains("GLOBAL") {
            return Some(1);
        }
        if token == "2" || token.contains("POKER") {
            return Some(2);
        }
        if token == "3" || token.contains("CHESS") {
            return Some(3);
        }
        if token == "4" || token.contains("FIGHTER") {
            return Some(4);
        }
        None
    }

    fn game_name(&self, n: u8) -> &'static str {
        match n {
            1 => "GLOBAL THERMONUCLEAR WAR",
            2 => "POKER",
            3 => "CHESS",
            4 => "FIGHTER COMBAT",
            _ => "UNKNOWN",
        }
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

    #[test]
    fn command_handling_works() {
        let mut terminal = WargamesTerminal::new(WargamesConfig::new());
        terminal.set_input("HELP");
        terminal.handle_command();
        for t in 0..5000 {
            terminal.tick(t);
        }
        assert!(!terminal.buffer.is_empty());
    }

    #[test]
    fn game_selection_persists() {
        let mut terminal = WargamesTerminal::new(WargamesConfig::new());
        terminal.set_input("SELECT 1");
        terminal.handle_command();
        assert_eq!(terminal.selected_game, Some(1));
    }
}
