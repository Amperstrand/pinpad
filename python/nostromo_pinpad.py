from __future__ import annotations

# pyright: reportMissingImports=false, reportUnknownMemberType=false, reportUnknownVariableType=false, reportUnknownArgumentType=false, reportUnknownParameterType=false, reportMissingParameterType=false, reportDeprecated=false

from dataclasses import dataclass
import random
import time


COLORS = {
    "panel_background": (232, 230, 225),
    "button_blue": (0, 90, 156),
    "button_red": (176, 0, 0),
    "indicator_amber": (255, 176, 0),
    "indicator_green": (0, 255, 65),
    "text": (26, 26, 26),
    "wear": (58, 53, 48),
}


@dataclass
class NostromoConfig:
    submit_delay_ms: int = 700
    button_press_ms: int = 120
    demo_interval_ms: int = 2200
    demo_digit_spacing_ms: int = 110
    max_code_len: int = 6


class NostromoButton:
    def __init__(self, label: str, color: tuple[int, int, int]):
        self.label: str = label
        self.color: tuple[int, int, int] = color
        self.last_pressed_ms: int | None = None

    def press(self, now_ms: int) -> None:
        self.last_pressed_ms = now_ms

    def is_pressed(self, now_ms: int, hold_ms: int) -> bool:
        if self.last_pressed_ms is None:
            return False
        return now_ms - self.last_pressed_ms < hold_ms


class NostromoKeypad:
    LAYOUT: list[str] = [
        "1", "2", "3",
        "4", "5", "6",
        "7", "8", "9",
        "C", "0", "E",
    ]

    def __init__(self, config: NostromoConfig | None = None):
        self.config: NostromoConfig = config or NostromoConfig()
        self.buttons: dict[str, NostromoButton] = {}
        for label in self.LAYOUT:
            color = COLORS["button_blue"]
            if label == "C":
                color = COLORS["button_red"]
            self.buttons[label] = NostromoButton(label, color)

        self.code: str = ""
        self.status_message: str = "Ready"
        self.busy_until_ms: int = 0
        self.demo_enabled: bool = False
        self.next_demo_ms: int = 0
        self.pending_events: list[tuple[int, str]] = []

    def is_busy(self, now_ms: int) -> bool:
        return now_ms < self.busy_until_ms

    def lamps(self, now_ms: int) -> tuple[bool, bool]:
        amber = self.is_busy(now_ms)
        green = not amber
        return amber, green

    def queue_demo_sequence(self, now_ms: int) -> None:
        digits = [str(random.randint(0, 9)) for _ in range(4)]
        for idx, digit in enumerate(digits):
            when = now_ms + idx * self.config.demo_digit_spacing_ms
            self.pending_events.append((when, digit))
        submit_at = now_ms + len(digits) * self.config.demo_digit_spacing_ms + 40
        self.pending_events.append((submit_at, "E"))

    def update(self, now_ms: int) -> None:
        if self.demo_enabled and now_ms >= self.next_demo_ms and not self.is_busy(now_ms):
            self.code = ""
            self.queue_demo_sequence(now_ms)
            self.next_demo_ms = now_ms + self.config.demo_interval_ms

        ready_events = [ev for ev in self.pending_events if ev[0] <= now_ms]
        self.pending_events = [ev for ev in self.pending_events if ev[0] > now_ms]
        for _, key in sorted(ready_events, key=lambda ev: ev[0]):
            self.handle_key(key, now_ms)

        if not self.is_busy(now_ms) and self.status_message.startswith("Open"):
            self.status_message = "Ready"

    def _submit(self, now_ms: int) -> None:
        if not self.code:
            return
        submitted = self.code
        self.code = ""
        self.busy_until_ms = now_ms + self.config.submit_delay_ms
        self.status_message = f"Open {submitted}"

    def handle_key(self, key: str, now_ms: int) -> None:
        if key not in self.buttons or self.is_busy(now_ms):
            return

        self.buttons[key].press(now_ms)
        if key == "C":
            self.code = ""
            self.status_message = "Code cleared"
            return
        if key == "E":
            self.status_message = "Cycle in progress"
            self._submit(now_ms)
            return

        if len(self.code) < self.config.max_code_len:
            self.code += key
            self.status_message = "Entering code"


def _clamp_color(c: tuple[int, int, int]) -> tuple[int, int, int]:
    return (max(0, min(255, c[0])), max(0, min(255, c[1])), max(0, min(255, c[2])))


def _mix_color(a: tuple[int, int, int], b: tuple[int, int, int], t: float) -> tuple[int, int, int]:
    return _clamp_color(
        (
            int(a[0] + (b[0] - a[0]) * t),
            int(a[1] + (b[1] - a[1]) * t),
            int(a[2] + (b[2] - a[2]) * t),
        )
    )


if __name__ == "__main__":
    try:
        import pygame
    except ImportError as exc:
        raise SystemExit("pygame is required for demo mode: pip install pygame") from exc

    pygame.init()
    pygame.display.set_caption("Nostromo Door Control Pinpad")

    WIDTH, HEIGHT = 420, 560
    PANEL_RECT = pygame.Rect(28, 28, WIDTH - 56, HEIGHT - 56)
    DISPLAY_RECT = pygame.Rect(PANEL_RECT.left + 14, PANEL_RECT.top + 46, PANEL_RECT.width - 28, 62)
    LAMP_AMBER = (PANEL_RECT.right - 58, PANEL_RECT.top + 22)
    LAMP_GREEN = (PANEL_RECT.right - 34, PANEL_RECT.top + 22)
    KEY_W, KEY_H, GAP = 90, 64, 10
    KEYS_ORIGIN = (PANEL_RECT.left + 20, PANEL_RECT.top + 126)

    FONT_TITLE = pygame.font.SysFont("arialnarrow", 18, bold=True)
    FONT_CODE = pygame.font.SysFont("arialnarrow", 30, bold=True)
    FONT_LABEL = pygame.font.SysFont("arialnarrow", 28, bold=True)
    FONT_STATUS = pygame.font.SysFont("arialnarrow", 16, bold=True)
    FONT_FOOT = pygame.font.SysFont("arialnarrow", 14, bold=True)

    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    clock = pygame.time.Clock()

    config = NostromoConfig()
    keypad = NostromoKeypad(config)

    key_rects: dict[str, pygame.Rect] = {}
    for i, label in enumerate(NostromoKeypad.LAYOUT):
        row, col = divmod(i, 3)
        x = KEYS_ORIGIN[0] + col * (KEY_W + GAP)
        y = KEYS_ORIGIN[1] + row * (KEY_H + GAP)
        key_rects[label] = pygame.Rect(x, y, KEY_W, KEY_H)

    def draw_wear(surface, rect) -> None:
        overlay = pygame.Surface((rect.width, rect.height), pygame.SRCALPHA)
        for _ in range(420):
            x = random.randint(0, rect.width - 1)
            y = random.randint(0, rect.height - 1)
            alpha = random.randint(6, 28)
            overlay.set_at((x, y), (*COLORS["wear"], alpha))
        for i in range(0, rect.height, 8):
            pygame.draw.line(overlay, (*COLORS["wear"], 16), (0, i), (rect.width, i), 1)
        surface.blit(overlay, rect.topleft)

    def draw_key(surface, label: str, rect, now_ms: int) -> None:
        button = keypad.buttons[label]
        pressed = button.is_pressed(now_ms, config.button_press_ms)
        top = rect.y + (4 if pressed else 0)
        face = pygame.Rect(rect.x, top, rect.width, rect.height)
        shadow_offset = 1 if pressed else 4

        shadow_color = _mix_color(COLORS["wear"], (0, 0, 0), 0.45)
        pygame.draw.rect(surface, shadow_color, (rect.x, top + rect.height, rect.width, shadow_offset), border_radius=7)

        grad_top = _mix_color((255, 255, 255), button.color, 0.42)
        grad_bottom = _mix_color(button.color, (0, 0, 0), 0.38)
        for i in range(face.height):
            t = i / max(1, face.height - 1)
            color = _mix_color(grad_top, grad_bottom, t)
            pygame.draw.line(surface, color, (face.x, face.y + i), (face.right - 1, face.y + i))

        pygame.draw.rect(surface, (26, 26, 26), face, 1, border_radius=7)
        pygame.draw.rect(surface, (*COLORS["wear"], 90), face, 1, border_radius=7)
        jewel = pygame.Rect(face.x + 8, face.y + 8, face.width - 16, face.height - 20)
        pygame.draw.ellipse(surface, (255, 255, 255, 70), jewel)

        text = FONT_LABEL.render(label, True, (247, 251, 255))
        text_rect = text.get_rect(center=face.center)
        surface.blit(text, text_rect)

    def draw_panel(now_ms: int) -> None:
        screen.fill(_mix_color(COLORS["wear"], COLORS["panel_background"], 0.35))

        pygame.draw.rect(screen, COLORS["panel_background"], PANEL_RECT, border_radius=18)
        pygame.draw.rect(screen, _mix_color(COLORS["wear"], COLORS["panel_background"], 0.5), PANEL_RECT, 2, border_radius=18)
        draw_wear(screen, PANEL_RECT)

        title = FONT_TITLE.render("DOOR CONTROL", True, COLORS["text"])
        screen.blit(title, (PANEL_RECT.left + 14, PANEL_RECT.top + 14))

        amber_on, green_on = keypad.lamps(now_ms)
        amber_color = COLORS["indicator_amber"] if amber_on else _mix_color(COLORS["indicator_amber"], COLORS["wear"], 0.65)
        green_color = COLORS["indicator_green"] if green_on else _mix_color(COLORS["indicator_green"], COLORS["wear"], 0.65)
        pygame.draw.circle(screen, amber_color, LAMP_AMBER, 8)
        pygame.draw.circle(screen, (26, 26, 26), LAMP_AMBER, 8, 1)
        pygame.draw.circle(screen, green_color, LAMP_GREEN, 8)
        pygame.draw.circle(screen, (26, 26, 26), LAMP_GREEN, 8, 1)

        pygame.draw.rect(screen, _mix_color(COLORS["panel_background"], COLORS["wear"], 0.2), DISPLAY_RECT, border_radius=10)
        pygame.draw.rect(screen, _mix_color(COLORS["wear"], COLORS["text"], 0.35), DISPLAY_RECT, 1, border_radius=10)

        code_text = FONT_CODE.render(keypad.code, True, COLORS["text"])
        screen.blit(code_text, (DISPLAY_RECT.left + 10, DISPLAY_RECT.top + 6))
        status_text = FONT_STATUS.render(keypad.status_message.upper(), True, _mix_color(COLORS["text"], COLORS["wear"], 0.2))
        screen.blit(status_text, (DISPLAY_RECT.left + 10, DISPLAY_RECT.bottom - 24))

        for label in NostromoKeypad.LAYOUT:
            draw_key(screen, label, key_rects[label], now_ms)

        foot = FONT_FOOT.render("KEYS: 0-9 C E | D: DEMO", True, _mix_color(COLORS["text"], COLORS["wear"], 0.15))
        screen.blit(foot, (PANEL_RECT.left + 18, PANEL_RECT.bottom - 26))

    def key_from_pygame(ev) -> str | None:
        if ev.type != pygame.KEYDOWN:
            return None
        if pygame.K_0 <= ev.key <= pygame.K_9:
            return chr(ev.key)
        if ev.key == pygame.K_c:
            return "C"
        if ev.key in (pygame.K_e, pygame.K_RETURN):
            return "E"
        if ev.key in (pygame.K_BACKSPACE, pygame.K_DELETE, pygame.K_ESCAPE):
            return "C"
        return None

    running = True
    while running:
        now = int(time.time() * 1000)
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
                continue
            if event.type == pygame.KEYDOWN and event.key == pygame.K_d:
                keypad.demo_enabled = not keypad.demo_enabled
                keypad.next_demo_ms = now
                keypad.status_message = "Demo mode" if keypad.demo_enabled else "Ready"
                continue
            if event.type == pygame.MOUSEBUTTONDOWN and event.button == 1:
                for label, rect in key_rects.items():
                    if rect.collidepoint(event.pos):
                        keypad.handle_key(label, now)
                        break
                continue
            mapped = key_from_pygame(event)
            if mapped is not None:
                keypad.handle_key(mapped, now)

        keypad.update(now)
        draw_panel(now)
        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
