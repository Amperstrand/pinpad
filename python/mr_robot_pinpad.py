# pyright: reportMissingImports=false
"""
Mr. Robot FBI Keypad - Hacker Terminal Style

A Python implementation of the Mr. Robot terminal pinpad effect using pygame.
Core classes are platform-agnostic.
"""

from dataclasses import dataclass
from typing import Any, Dict, Optional
import time
import random

# =============================================================================
# CONSTANTS & COLORS
# =============================================================================
DEEP_BLACK = (10, 10, 10)          # #0a0a0a - background
PHOSPHOR_GREEN = (0, 255, 65)      # #00FF41 - primary text
DIM_GREEN = (0, 128, 32)           # #008020 - dim text/secondary
TERMINAL_BG = (0, 20, 0)           # #001400 - terminal background
GLOW_GREEN = (0, 255, 65, 128)     # rgba(0,255,65,0.5) - glow effect
CYAN = (122, 236, 255)             # #7aecff
TEAL = (30, 80, 95)                # #1e505f
ERROR_RED = (255, 51, 51)          # #ff3333
SUCCESS_GREEN = (0, 255, 0)        # #00ff00
SCANLINE_COLOR = (0, 0, 0, 38)     # rgba(0,0,0,0.15)


# =============================================================================
# CORE CLASSES (Platform-Agnostic)
# =============================================================================

@dataclass
class TerminalConfig:
    cursor_blink_ms: int = 530
    typing_delay_ms: int = 30
    typing_variance_ms: int = 15
    grain_update_interval_ms: int = 100
    chromatic_offset_px: int = 2
    button_active_ms: int = 150

class TerminalButton:
    def __init__(self, label: str):
        self.label = label
        self._pressed_at: Optional[float] = None
    
    @property
    def pressed_at(self) -> Optional[float]:
        return self._pressed_at
    
    def press(self, timestamp_ms: Optional[float] = None) -> None:
        """Record a button press. Uses pygame ticks when running as demo."""
        self._pressed_at = timestamp_ms if timestamp_ms is not None else (time.time() * 1000)
        
    def reset(self) -> None:
        self._pressed_at = None

class MrRobotKeypad:
    BUTTONS = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E']
    MAX_PIN_LENGTH = 6
    CORRECT_PIN = '1234'
    
    def __init__(self, config: Optional[TerminalConfig] = None):
        self.config = config or TerminalConfig()
        self._buttons: Dict[str, TerminalButton] = {
            label: TerminalButton(label) for label in self.BUTTONS
        }
        self.entered_pin = ''
        
    def press_button(self, label: str) -> bool:
        if label not in self._buttons:
            return False
            
        self._buttons[label].press()
        
        if label == 'C':
            self.clear_pin()
        elif label == 'E':
            pass # handled externally
        elif label.isdigit():
            if len(self.entered_pin) < self.MAX_PIN_LENGTH:
                self.entered_pin += label
        return True

    def clear_pin(self) -> None:
        self.entered_pin = ''
        
    def reset(self) -> None:
        self.clear_pin()
        for btn in self._buttons.values():
            btn.reset()
            
    def get_button(self, label: str) -> Optional[TerminalButton]:
        return self._buttons.get(label)

# =============================================================================
# PYGAME DEMO
# =============================================================================

if __name__ == "__main__":
    import sys

    try:
        pygame = __import__("pygame")
    except ModuleNotFoundError:
        raise SystemExit("pygame is required to run mr_robot_pinpad.py")
    
    pygame.init()
    
    # Setup window
    WIDTH, HEIGHT = 600, 600
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Mr. Robot FBI Terminal Pinpad")
    
    # Load fonts
    try:
        font_main = pygame.font.SysFont("monaco", 18)
        font_large = pygame.font.SysFont("monaco", 24, bold=True)
    except:
        font_main = pygame.font.Font(None, 18)
        font_large = pygame.font.Font(None, 24)
    
    keypad = MrRobotKeypad()
    config = keypad.config
    clock = pygame.time.Clock()
    
    # Terminal output simulation
    terminal_lines: list[tuple[str, tuple[int, int, int]]] = [
        ("[*] FBI Femtocell Interception System v2.5", DIM_GREEN),
        ("[*] Target: E Corp HQ, Floor 23", DIM_GREEN),
        ("[+] System ready. Enter PIN to authenticate.", SUCCESS_GREEN)
    ]
    TERMINAL_MAX_LINES = 12
    TERMINAL_PROMPT = "fsociety@fbi-target:~$"
    DEMO_CYCLE_MS = 5000
    DEMO_INITIAL_DELAY_MS = 600
    DEMO_SUBMIT_DELAY_MS = 240
    DEMO_PINS = ["1234", "0000", "9999", "1337"]
    
    # Layout params
    MARGIN_X = 150
    MARGIN_Y = 250
    BTN_W = 70
    BTN_H = 55
    GAP = 12
    NOISE_POINTS = 700
    demo_mode = True
    next_demo_at = pygame.time.get_ticks() + DEMO_INITIAL_DELAY_MS
    demo_schedule: list[tuple[int, str]] = []
    demo_pin_waiting: Optional[str] = None
    typing_line: Optional[dict[str, Any]] = None

    def typing_delay_ms() -> int:
        variance = random.randint(-config.typing_variance_ms, config.typing_variance_ms)
        return max(1, config.typing_delay_ms + variance)

    def append_terminal_line(text: str, color: tuple[int, int, int]) -> None:
        terminal_lines.append((text, color))
        if len(terminal_lines) > TERMINAL_MAX_LINES:
            del terminal_lines[0]

    def queue_typed_command(command: str, color: tuple[int, int, int] = PHOSPHOR_GREEN) -> None:
        global typing_line
        full = f"{TERMINAL_PROMPT} {command}"
        typing_line = {
            "full": full,
            "visible": "",
            "next_tick": pygame.time.get_ticks() + typing_delay_ms(),
            "color": color,
        }

    def update_typed_command(now_ms: int) -> None:
        global typing_line, demo_pin_waiting, demo_schedule
        if typing_line is None:
            return

        next_tick = int(typing_line["next_tick"])
        if now_ms < next_tick:
            return

        full = str(typing_line["full"])
        visible = str(typing_line["visible"])
        if len(visible) < len(full):
            visible += full[len(visible)]
            typing_line["visible"] = visible
            typing_line["next_tick"] = now_ms + typing_delay_ms()

        if len(visible) == len(full):
            color = typing_line["color"]
            if isinstance(color, tuple):
                append_terminal_line(visible, color)
            typing_line = None

            if demo_pin_waiting is not None:
                elapsed = 0
                for digit in demo_pin_waiting:
                    elapsed += typing_delay_ms()
                    demo_schedule.append((now_ms + elapsed, digit))
                demo_schedule.append((now_ms + elapsed + DEMO_SUBMIT_DELAY_MS, 'E'))
                demo_pin_waiting = None

    def handle_button_press(label: str, now_ms: int, from_demo: bool = False) -> None:
        if not keypad.press_button(label):
            return

        button = keypad.get_button(label)
        if button is not None:
            button.press(now_ms)

        if label == 'E':
            masked = '*' * len(keypad.entered_pin)
            if masked:
                append_terminal_line(f"[*] Verifying PIN: {masked}", DIM_GREEN)
            if keypad.entered_pin == keypad.CORRECT_PIN:
                append_terminal_line("[+] ACCESS GRANTED", SUCCESS_GREEN)
                append_terminal_line("[+] Femtocell interception active", SUCCESS_GREEN)
            else:
                append_terminal_line("[-] ACCESS DENIED", ERROR_RED)
            keypad.clear_pin()
        elif label == 'C':
            append_terminal_line("[*] PIN cleared", DIM_GREEN)
        elif from_demo and label.isdigit():
            append_terminal_line(f"[*] Demo keytap: {label}", DIM_GREEN)
    
    def get_btn_rect(index: int):
        row = index // 3
        col = index % 3
        x = MARGIN_X + col * (BTN_W + GAP)
        y = MARGIN_Y + row * (BTN_H + GAP)
        return pygame.Rect(x, y, BTN_W, BTN_H)
    
    def draw_terminal():
        y = 30
        for text, color in terminal_lines[-10:]:
            surf = font_main.render(text, True, color)
            screen.blit(surf, (20, y))
            y += 20

        if typing_line is not None:
            active_text = str(typing_line["visible"])
            color = typing_line["color"]
            text_color = color if isinstance(color, tuple) else PHOSPHOR_GREEN
            surf = font_main.render(active_text, True, text_color)
            screen.blit(surf, (20, y))
            y += 20
            
        # Draw prompt and PIN asterisks
        prompt = f"{TERMINAL_PROMPT} auth --pin "
        pin_disp = "*" * len(keypad.entered_pin)
        full_text = prompt + pin_disp

        # Phosphor glow pass
        glow = font_main.render(full_text, True, (0, 80, 30))
        screen.blit(glow, (20, y))
        surf = font_main.render(full_text, True, CYAN)
        screen.blit(surf, (20, y))
        
        # Cursor
        if (pygame.time.get_ticks() // config.cursor_blink_ms) % 2 == 0:
            cx = 20 + font_main.size(full_text)[0] + 2
            pygame.draw.rect(screen, PHOSPHOR_GREEN, (cx, y + 2, 8, 14))

    def draw_scanlines():
        scanline_surf = pygame.Surface((WIDTH, HEIGHT), pygame.SRCALPHA)
        for y in range(0, HEIGHT, 4):
            pygame.draw.line(scanline_surf, SCANLINE_COLOR, (0, y), (WIDTH, y), 2)
        screen.blit(scanline_surf, (0, 0))

    def build_grain_surface():
        grain = pygame.Surface((WIDTH, HEIGHT), pygame.SRCALPHA)
        for _ in range(NOISE_POINTS):
            x = random.randint(0, WIDTH - 1)
            y = random.randint(0, HEIGHT - 1)
            g = random.randint(28, 58)
            color = (0, min(255, g + 45), g // 3, 18)
            grain.set_at((x, y), color)
        return grain

    def draw_button_border(rect, is_pressed: bool, is_hovered: bool):
        if not (is_pressed or is_hovered):
            pygame.draw.rect(screen, DIM_GREEN, rect, 1, border_radius=3)
            return

        alpha = 90 if is_pressed else 64
        overlay = pygame.Surface((rect.width + config.chromatic_offset_px * 2, rect.height), pygame.SRCALPHA)

        left_rect = pygame.Rect(0, 0, rect.width, rect.height)
        center_rect = pygame.Rect(config.chromatic_offset_px, 0, rect.width, rect.height)
        right_rect = pygame.Rect(config.chromatic_offset_px * 2, 0, rect.width, rect.height)

        pygame.draw.rect(overlay, (255, 60, 70, int(alpha * 0.45)), left_rect, 1, border_radius=3)
        pygame.draw.rect(overlay, (0, 255, 65, alpha), center_rect, 1, border_radius=3)
        pygame.draw.rect(overlay, (120, 210, 255, int(alpha * 0.5)), right_rect, 1, border_radius=3)

        screen.blit(overlay, (rect.x - config.chromatic_offset_px, rect.y))

    cached_grain = build_grain_surface()
    last_grain_update = pygame.time.get_ticks()

    running = True
    while running:
        current_time = pygame.time.get_ticks()
        hover_pos = pygame.mouse.get_pos()
        hovered_label = None
        
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
                
            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:
                    pos = event.pos
                    for i, label in enumerate(keypad.BUTTONS):
                        if get_btn_rect(i).collidepoint(pos):
                            handle_button_press(label, current_time)
                                
            elif event.type == pygame.KEYDOWN:
                char = event.unicode.upper()
                if char in keypad.BUTTONS and char not in ['C', 'E']:
                    handle_button_press(char, current_time)
                elif event.key == pygame.K_RETURN:
                    handle_button_press('E', current_time)
                elif event.key == pygame.K_ESCAPE or event.key == pygame.K_BACKSPACE:
                    handle_button_press('C', current_time)
                elif event.key == pygame.K_F1:
                    demo_mode = not demo_mode
                    state = "enabled" if demo_mode else "disabled"
                    append_terminal_line(f"[*] Demo mode {state}", DIM_GREEN)
                    if demo_mode:
                        next_demo_at = current_time + DEMO_INITIAL_DELAY_MS
                        demo_schedule.clear()
                        demo_pin_waiting = None

        update_typed_command(current_time)

        if demo_mode and typing_line is None and not demo_schedule and current_time >= next_demo_at:
            pin = random.choice(DEMO_PINS)
            demo_pin_waiting = pin
            queue_typed_command(f"pinpad-auth --target ecorp --pin {pin}")
            next_demo_at = current_time + DEMO_CYCLE_MS

        if demo_schedule:
            due = [item for item in demo_schedule if item[0] <= current_time]
            demo_schedule = [item for item in demo_schedule if item[0] > current_time]
            for _, label in due:
                handle_button_press(label, current_time, from_demo=True)
                     
        screen.fill(DEEP_BLACK)
        
        # Draw background elements
        pygame.draw.rect(screen, TERMINAL_BG, (10, 10, WIDTH-20, 200), border_radius=4)
        pygame.draw.rect(screen, TEAL, (10, 10, WIDTH-20, 200), 1, border_radius=4)
        
        draw_terminal()
        
        # Draw Keypad
        for i, label in enumerate(keypad.BUTTONS):
            rect = get_btn_rect(i)
            btn = keypad.get_button(label)
            if rect.collidepoint(hover_pos):
                hovered_label = label
            
            is_pressed = btn is not None and btn.pressed_at is not None and (current_time - btn.pressed_at < config.button_active_ms)
            is_hovered = hovered_label == label
            
            # Draw phosphor glow for pressed buttons
            if is_pressed:
                glow_surf = pygame.Surface((rect.width + 60, rect.height + 60), pygame.SRCALPHA)
                pygame.draw.ellipse(glow_surf, (0, 255, 65, 40), glow_surf.get_rect())
                screen.blit(glow_surf, (rect.x - 30, rect.y - 30))
            
            if is_pressed:
                bg_color = (0, 40, 20)
            elif is_hovered:
                bg_color = (0, 32, 16)
            else:
                bg_color = (0, 20, 0)
            
            pygame.draw.rect(screen, bg_color, rect, border_radius=3)
            draw_button_border(rect, is_pressed, is_hovered)
            
            disp_label = 'CLR' if label == 'C' else 'ENT' if label == 'E' else label
            text_color = PHOSPHOR_GREEN
            
            text_surf = font_large.render(disp_label, True, text_color)
            text_rect = text_surf.get_rect(center=rect.center)
            screen.blit(text_surf, text_rect)

        if current_time - last_grain_update >= config.grain_update_interval_ms:
            cached_grain = build_grain_surface()
            last_grain_update = current_time

        screen.blit(cached_grain, (0, 0))
            
        draw_scanlines()
        
        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
    sys.exit()
