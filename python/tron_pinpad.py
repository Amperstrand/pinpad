# pyright: reportMissingImports=false, reportUnknownMemberType=false, reportUnknownVariableType=false, reportUnknownParameterType=false, reportUnknownArgumentType=false, reportUnusedCallResult=false

from dataclasses import dataclass
from enum import Enum, auto
from typing import Final
import random
import time


class TronColors:
    NEON_BLUE: Final[tuple[int, int, int]] = (42, 210, 255)
    NEON_ORANGE: Final[tuple[int, int, int]] = (255, 157, 0)
    NEON_WHITE: Final[tuple[int, int, int]] = (224, 247, 255)
    DEEP_BLACK: Final[tuple[int, int, int]] = (3, 5, 4)
    GRID_CYAN: Final[tuple[int, int, int]] = (0, 140, 163)


@dataclass
class TronConfig:
    verify_ms: int = 360
    key_flash_ms: int = 110
    max_digits: int = 4
    flicker_min: float = 0.9
    flicker_max: float = 1.0


class AuthState(Enum):
    IDLE = auto()
    VERIFYING = auto()
    SUCCESS = auto()
    ERROR = auto()


class TronKeypad:
    BUTTONS: Final[list[str]] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "C", "0", "E"]

    def __init__(self, config: TronConfig | None = None, correct_code: str = "1982"):
        self.config: TronConfig = config or TronConfig()
        self.correct_code: str = correct_code
        self.code: str = ""
        self.state: AuthState = AuthState.IDLE
        self.state_since_ms: float = time.time() * 1000
        self.flash_label: str | None = None
        self.flash_since_ms: float = 0.0

    @property
    def masked_code(self) -> str:
        return "*" * len(self.code)

    def _set_state(self, state: AuthState) -> None:
        self.state = state
        self.state_since_ms = time.time() * 1000

    def press(self, label: str) -> bool:
        if self.state == AuthState.VERIFYING:
            return False
        if label not in self.BUTTONS:
            return False

        self.flash_label = label
        self.flash_since_ms = time.time() * 1000

        if label.isdigit():
            if len(self.code) < self.config.max_digits:
                self.code += label
            return True

        if label == "C":
            self.code = ""
            self._set_state(AuthState.IDLE)
            return True

        if label == "E":
            if self.code:
                self._set_state(AuthState.VERIFYING)
            else:
                self._set_state(AuthState.ERROR)
            return True

        return False

    def update(self, now_ms: float | None = None) -> None:
        if now_ms is None:
            now_ms = time.time() * 1000

        if self.state == AuthState.VERIFYING and now_ms - self.state_since_ms >= self.config.verify_ms:
            if self.code == self.correct_code:
                self._set_state(AuthState.SUCCESS)
            else:
                self.code = ""
                self._set_state(AuthState.ERROR)

        if self.state == AuthState.SUCCESS and now_ms - self.state_since_ms >= self.config.key_flash_ms * 2:
            self.code = ""
            self._set_state(AuthState.IDLE)

        if self.state == AuthState.ERROR and now_ms - self.state_since_ms >= self.config.key_flash_ms * 2:
            self._set_state(AuthState.IDLE)

    def flash_intensity(self, now_ms: float | None = None) -> tuple[str | None, float]:
        if now_ms is None:
            now_ms = time.time() * 1000
        if self.flash_label is None:
            return (None, 0.0)

        elapsed = now_ms - self.flash_since_ms
        if elapsed >= self.config.key_flash_ms:
            self.flash_label = None
            return (None, 0.0)

        return (self.flash_label, 1.0 - (elapsed / self.config.key_flash_ms))


if __name__ == "__main__":
    import pygame

    pygame.init()
    pygame.display.set_caption("Tron (1982) Control Panel")

    WIDTH, HEIGHT = 430, 620
    DISPLAY_RECT = pygame.Rect(24, 52, WIDTH - 48, 104)
    KEYS_TOP = 252
    KEY_W, KEY_H, GAP = 116, 74, 9

    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    clock = pygame.time.Clock()

    font_title = pygame.font.SysFont("couriernew", 16, bold=True)
    font_small = pygame.font.SysFont("couriernew", 13)
    font_code = pygame.font.SysFont("couriernew", 40, bold=True)
    font_key = pygame.font.SysFont("couriernew", 34, bold=True)

    keypad = TronKeypad()

    key_rects: dict[str, pygame.Rect] = {}
    for idx, label in enumerate(TronKeypad.BUTTONS):
        row = idx // 3
        col = idx % 3
        x = 24 + col * (KEY_W + GAP)
        y = KEYS_TOP + row * (KEY_H + GAP)
        key_rects[label] = pygame.Rect(x, y, KEY_W, KEY_H)

    def polygon_for_bevel(rect: pygame.Rect, inset: int = 10) -> list[tuple[int, int]]:
        return [
            (rect.left + inset, rect.top),
            (rect.right - inset, rect.top),
            (rect.right, rect.top + inset),
            (rect.right, rect.bottom - inset),
            (rect.right - inset, rect.bottom),
            (rect.left + inset, rect.bottom),
            (rect.left, rect.bottom - inset),
            (rect.left, rect.top + inset),
        ]

    def draw_bloom(center: tuple[int, int], color: tuple[int, int, int], alpha: int, radius: int) -> None:
        surf = pygame.Surface((radius * 2, radius * 2), pygame.SRCALPHA)
        pygame.draw.circle(surf, (*color, alpha), (radius, radius), radius)
        pygame.draw.circle(surf, (*TronColors.NEON_WHITE, int(alpha * 0.48)), (radius, radius), int(radius * 0.45))
        screen.blit(surf, (center[0] - radius, center[1] - radius), special_flags=pygame.BLEND_PREMULTIPLIED)

    running = True
    while running:
        now_ms = time.time() * 1000
        keypad.update(now_ms)
        flash_label, flash_intensity = keypad.flash_intensity(now_ms)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.MOUSEBUTTONDOWN and event.button == 1:
                for label, rect in key_rects.items():
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

        flicker = random.uniform(keypad.config.flicker_min, keypad.config.flicker_max)

        screen.fill(TronColors.DEEP_BLACK)

        header_left = font_title.render("ENCOM GRID I/O", True, TronColors.NEON_BLUE)
        header_right = font_title.render("MCP BUS 12", True, TronColors.GRID_CYAN)
        screen.blit(header_left, (24, 18))
        screen.blit(header_right, (WIDTH - header_right.get_width() - 24, 18))

        pygame.draw.rect(screen, (2, 9, 11), DISPLAY_RECT)
        pygame.draw.rect(screen, TronColors.NEON_BLUE, DISPLAY_RECT, 1)

        state_text = "AWAITING COMMAND"
        state_color = TronColors.GRID_CYAN
        if keypad.state == AuthState.VERIFYING:
            state_text = "ROUTING PACKET..."
            state_color = TronColors.NEON_BLUE
        elif keypad.state == AuthState.SUCCESS:
            state_text = "ACCESS GRANTED"
            state_color = TronColors.NEON_WHITE
        elif keypad.state == AuthState.ERROR:
            state_text = "ACCESS DENIED"
            state_color = TronColors.NEON_ORANGE

        status_surf = font_small.render(state_text, True, state_color)
        code_surf = font_code.render(keypad.masked_code, True, TronColors.NEON_WHITE)
        hint_surf = font_small.render("0-9, C/BKSP clear, E/ENTER submit", True, TronColors.NEON_BLUE)
        screen.blit(status_surf, (DISPLAY_RECT.left + 10, DISPLAY_RECT.top + 12))
        screen.blit(code_surf, (DISPLAY_RECT.left + 10, DISPLAY_RECT.top + 35))
        screen.blit(hint_surf, (DISPLAY_RECT.left + 10, DISPLAY_RECT.bottom + 10))

        trace_color = (*TronColors.NEON_BLUE, int(200 * flicker))
        trace = pygame.Surface((WIDTH, HEIGHT), pygame.SRCALPHA)
        pygame.draw.line(trace, trace_color, (30, 186), (140, 186), 2)
        pygame.draw.line(trace, trace_color, (140, 186), (168, 214), 2)
        pygame.draw.line(trace, trace_color, (168, 214), (168, 244), 2)
        pygame.draw.line(trace, trace_color, (168, 244), (386, 244), 2)
        pygame.draw.line(trace, trace_color, (278, 244), (304, 218), 2)
        pygame.draw.line(trace, trace_color, (304, 218), (304, 191), 2)
        for node in ((140, 186), (168, 244), (304, 191)):
            pygame.draw.rect(trace, (*TronColors.NEON_WHITE, int(210 * flicker)), pygame.Rect(node[0] - 3, node[1] - 3, 6, 6), 1)
        screen.blit(trace, (0, 0))

        for label, rect in key_rects.items():
            is_flash = flash_label == label
            bloom_alpha = int((130 + 90 * flash_intensity) * flicker) if is_flash else int(105 * flicker)
            border_color = TronColors.NEON_WHITE if is_flash else TronColors.NEON_BLUE

            draw_bloom(rect.center, TronColors.NEON_BLUE, bloom_alpha, int(max(rect.width, rect.height) * 0.78))

            points = polygon_for_bevel(rect)
            pygame.draw.polygon(screen, (2, 13, 16), points)
            pygame.draw.polygon(screen, border_color, points, 1)

            core_rect = rect.inflate(-44, -30)
            core_points = polygon_for_bevel(core_rect, inset=6)
            pygame.draw.polygon(screen, (*TronColors.NEON_WHITE, 120), core_points)

            label_surf = font_key.render(label, True, TronColors.NEON_WHITE)
            screen.blit(label_surf, label_surf.get_rect(center=rect.center))

        overlay = pygame.Surface((WIDTH, HEIGHT), pygame.SRCALPHA)
        overlay.fill((0, 0, 0, int((1.0 - flicker) * 60)))
        screen.blit(overlay, (0, 0))

        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
