from dataclasses import dataclass
from enum import Enum, auto
from typing import Final
import time


class DeusExColors:
    BACKGROUND: Final[tuple[int, int, int]] = (19, 18, 0)
    PRIMARY_GOLD: Final[tuple[int, int, int]] = (255, 234, 33)
    AMBER: Final[tuple[int, int, int]] = (229, 175, 46)
    DARK_GOLD: Final[tuple[int, int, int]] = (180, 145, 37)
    CYAN: Final[tuple[int, int, int]] = (0, 255, 255)
    RED: Final[tuple[int, int, int]] = (255, 0, 0)


@dataclass
class DeusExConfig:
    boot_ms: int = 400
    keypress_flash_ms: int = 100
    verify_ms: int = 800
    success_flash_ms: int = 400
    error_flash_ms: int = 250
    max_digits: int = 4


class AuthState(Enum):
    BOOTING = auto()
    IDLE = auto()
    VERIFYING = auto()
    SUCCESS = auto()
    ERROR = auto()


class DeusExKeypad:
    BUTTONS: Final[list[str]] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "C", "0", "E"]

    def __init__(self, config: DeusExConfig | None = None, correct_code: str = "0451"):
        self.config: DeusExConfig = config or DeusExConfig()
        self.correct_code: str = correct_code
        self.code: str = ""
        self.state: AuthState = AuthState.BOOTING
        self._state_started_ms: float = time.time() * 1000
        self._flashing_button: str | None = None
        self._flash_started_ms: float = 0.0

    @property
    def masked_code(self) -> str:
        return "*" * len(self.code)

    def get_boot_progress(self, now_ms: float | None = None) -> float:
        if now_ms is None:
            now_ms = time.time() * 1000
        elapsed = now_ms - self._state_started_ms
        return max(0.0, min(1.0, elapsed / self.config.boot_ms))

    def _set_state(self, state: AuthState) -> None:
        self.state = state
        self._state_started_ms = time.time() * 1000

    def press(self, label: str) -> bool:
        if self.state in (AuthState.BOOTING, AuthState.VERIFYING):
            return False

        if label not in self.BUTTONS:
            return False

        self._flashing_button = label
        self._flash_started_ms = time.time() * 1000

        if label.isdigit():
            if len(self.code) < self.config.max_digits:
                self.code += label
            return True

        if label == "C":
            self.code = ""
            return True

        if label == "E":
            self._submit()
            return True

        return False

    def _submit(self) -> None:
        if not self.code:
            self._set_state(AuthState.ERROR)
            return
        self._set_state(AuthState.VERIFYING)

    def update(self, now_ms: float | None = None) -> None:
        if now_ms is None:
            now_ms = time.time() * 1000

        if self.state == AuthState.BOOTING and self.get_boot_progress(now_ms) >= 1.0:
            self._set_state(AuthState.IDLE)

        if self.state == AuthState.VERIFYING and now_ms - self._state_started_ms >= self.config.verify_ms:
            if self.code == self.correct_code:
                self._set_state(AuthState.SUCCESS)
            else:
                self.code = ""
                self._set_state(AuthState.ERROR)

        if self.state == AuthState.SUCCESS and now_ms - self._state_started_ms >= self.config.success_flash_ms:
            self._set_state(AuthState.IDLE)
            self.code = ""

        if self.state == AuthState.ERROR and now_ms - self._state_started_ms >= self.config.error_flash_ms:
            self._set_state(AuthState.IDLE)

    def flashing_button_intensity(self, now_ms: float | None = None) -> tuple[str | None, float]:
        if now_ms is None:
            now_ms = time.time() * 1000
        if self._flashing_button is None:
            return (None, 0.0)

        elapsed = now_ms - self._flash_started_ms
        if elapsed >= self.config.keypress_flash_ms:
            self._flashing_button = None
            return (None, 0.0)

        return (self._flashing_button, 1.0 - (elapsed / self.config.keypress_flash_ms))


if __name__ == "__main__":
    demo_namespace: dict[str, object] = {
        "DeusExKeypad": DeusExKeypad,
        "AuthState": AuthState,
        "DeusExColors": DeusExColors,
        "time": time,
    }
    exec(
        """
import pygame

pygame.init()
pygame.display.set_caption("Deus Ex: Human Revolution Keypad")

WIDTH, HEIGHT = 420, 560
GRID_TOP = 170
BTN_W, BTN_H, GAP = 110, 76, 12
FPS = 60

screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()

font_title = pygame.font.SysFont("consolas", 18, bold=True)
font_small = pygame.font.SysFont("consolas", 14)
font_code = pygame.font.SysFont("consolas", 40, bold=True)
font_key = pygame.font.SysFont("consolas", 34, bold=True)

keypad = DeusExKeypad()
button_rects = {}
for idx, label in enumerate(DeusExKeypad.BUTTONS):
    row = idx // 3
    col = idx % 3
    x = 34 + col * (BTN_W + GAP)
    y = GRID_TOP + row * (BTN_H + GAP)
    button_rects[label] = pygame.Rect(x, y, BTN_W, BTN_H)

running = True
while running:
    now_ms = time.time() * 1000
    keypad.update(now_ms)
    flash_label, flash_intensity = keypad.flashing_button_intensity(now_ms)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        elif event.type == pygame.MOUSEBUTTONDOWN and event.button == 1:
            for label, rect in button_rects.items():
                if rect.collidepoint(event.pos):
                    keypad.press(label)
                    break
        elif event.type == pygame.KEYDOWN:
            if pygame.K_0 <= event.key <= pygame.K_9:
                keypad.press(chr(event.key))
            elif event.key in (pygame.K_BACKSPACE, pygame.K_c):
                keypad.press("C")
            elif event.key in (pygame.K_RETURN, pygame.K_KP_ENTER, pygame.K_e):
                keypad.press("E")

    screen.fill(DeusExColors.BACKGROUND)
    header = font_title.render("SARIF SECURITY PANEL", True, DeusExColors.AMBER)
    io = font_small.render("I/O PORT", True, DeusExColors.CYAN)
    screen.blit(header, (30, 24))
    screen.blit(io, (WIDTH - io.get_width() - 30, 28))

    pygame.draw.rect(screen, (10, 10, 0), pygame.Rect(26, 50, WIDTH - 52, 120), border_radius=10)
    pygame.draw.rect(screen, DeusExColors.AMBER, pygame.Rect(26, 50, WIDTH - 52, 120), 2, border_radius=10)

    state_text = "AWAITING PASSCODE"
    state_color = DeusExColors.DARK_GOLD
    if keypad.state == AuthState.BOOTING:
        state_text = "BOOT SEQUENCE..."
    elif keypad.state == AuthState.VERIFYING:
        state_text = "VERIFYING..."
        state_color = DeusExColors.AMBER
    elif keypad.state == AuthState.SUCCESS:
        state_text = "ACCESS GRANTED"
        state_color = (156, 255, 156)
    elif keypad.state == AuthState.ERROR:
        state_text = "ACCESS DENIED"
        state_color = DeusExColors.RED

    status_surface = font_small.render(state_text, True, state_color)
    code_surface = font_code.render(keypad.masked_code, True, (255, 248, 210))
    screen.blit(status_surface, (40, 68))
    screen.blit(code_surface, (40, 92))

    for label, rect in button_rects.items():
        border = DeusExColors.AMBER
        glow = 0.25
        if flash_label == label:
            border = (255, 255, 255)
            glow = 0.4 + 0.6 * flash_intensity
        pygame.draw.rect(screen, (16, 15, 0), rect, border_radius=10)
        pygame.draw.rect(screen, border, rect, 2, border_radius=10)
        center = rect.center
        radius = int(max(rect.width, rect.height) * 0.9)
        glow_surf = pygame.Surface((radius * 2, radius * 2), pygame.SRCALPHA)
        pygame.draw.circle(glow_surf, (*DeusExColors.AMBER, int(100 * glow)), (radius, radius), radius)
        pygame.draw.circle(glow_surf, (255, 255, 255, int(120 * glow)), (radius, radius), int(radius * 0.45))
        screen.blit(glow_surf, (center[0] - radius, center[1] - radius), special_flags=pygame.BLEND_PREMULTIPLIED)
        text = font_key.render(label, True, (255, 246, 191))
        screen.blit(text, text.get_rect(center=rect.center))

    for y in range(0, HEIGHT, 4):
        pygame.draw.line(screen, (0, 0, 0), (0, y), (WIDTH, y), 1)

    footer = font_small.render("Keys: 0-9, C/Backspace clear, E/Enter submit", True, DeusExColors.DARK_GOLD)
    screen.blit(footer, (24, HEIGHT - 28))
    pygame.display.flip()
    clock.tick(FPS)

pygame.quit()
""",
        demo_namespace,
        demo_namespace,
    )
