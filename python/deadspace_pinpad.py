from __future__ import annotations

from dataclasses import dataclass
import importlib
import random
from typing import Any

pygame: Any = importlib.import_module("pygame")


COLORS = {
    "primary_cyan": (0, 255, 255),
    "deep_blue": (0, 71, 171),
    "health_teal": (0, 255, 204),
    "stasis_blue": (51, 153, 255),
    "alert_red": (255, 51, 0),
    "text_blue": (160, 230, 255),
}


@dataclass
class DeadSpaceConfig:
    width: int = 900
    height: int = 560
    holo_opacity: int = 185
    scanline_alpha: int = 20
    jitter_px: float = 2.0
    chromatic_offset: int = 2
    grid_rows: int = 4
    grid_cols: int = 4
    fps: int = 60


class DeadSpaceRigDemo:
    def __init__(self, config: DeadSpaceConfig):
        self.config: DeadSpaceConfig = config
        self.screen: Any = pygame.display.set_mode((config.width, config.height))
        pygame.display.set_caption("Dead Space RIG Holographic Interface")
        self.clock: Any = pygame.time.Clock()

        self.title_font: Any = pygame.font.SysFont("consolas", 20, bold=True)
        self.ui_font: Any = pygame.font.SysFont("consolas", 16)
        self.small_font: Any = pygame.font.SysFont("consolas", 13)

        self.grid_origin: tuple[int, int] = (config.width - 330, 125)
        self.slot_size: int = 66
        self.slot_gap: int = 10
        self.selected: int = 0
        self.health: int = 82
        self.stasis: int = 61
        self.mode_text: str = "STABLE LINK"
        self.mode_color: tuple[int, int, int] = COLORS["stasis_blue"]
        self.glyphs: list[str] = [
            "--", "PL", "SM", "TK",
            "ST", "NO", "MD", "AT",
            "SE", "BA", "EN", "RG",
            "MP", "HL", "DM", "RX",
        ]

    def run(self) -> None:
        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.KEYDOWN:
                    running = self.handle_key(event.key)

            self.draw_frame()
            pygame.display.flip()
            self.clock.tick(self.config.fps)

    def handle_key(self, key: int) -> bool:
        rows = self.config.grid_rows
        cols = self.config.grid_cols
        row = self.selected // cols
        col = self.selected % cols

        if key in (pygame.K_ESCAPE,):
            return False
        if key in (pygame.K_UP, pygame.K_w):
            row = (row - 1) % rows
        elif key in (pygame.K_DOWN, pygame.K_s):
            row = (row + 1) % rows
        elif key in (pygame.K_LEFT, pygame.K_a):
            col = (col - 1) % cols
        elif key in (pygame.K_RIGHT, pygame.K_d):
            col = (col + 1) % cols
        elif key in (pygame.K_RETURN, pygame.K_KP_ENTER):
            self.interact()
        elif key == pygame.K_SPACE:
            self.glitch_ping()

        self.selected = row * cols + col
        return True

    def interact(self) -> None:
        self.mode_text = f"SYNC {self.glyphs[self.selected]}"
        self.mode_color = COLORS["primary_cyan"]
        self.health = max(16, min(100, self.health + random.randint(-3, 4)))
        self.stasis = max(8, min(100, self.stasis + random.randint(-4, 5)))

    def glitch_ping(self) -> None:
        self.mode_text = "HOLO DESYNC"
        self.mode_color = COLORS["alert_red"]

    def draw_frame(self) -> None:
        self.draw_background()

        jitter_x = random.uniform(-self.config.jitter_px, self.config.jitter_px)
        jitter_y = random.uniform(-self.config.jitter_px, self.config.jitter_px)

        holo = pygame.Surface((self.config.width - 120, self.config.height - 90), pygame.SRCALPHA)
        holo.fill((0, 71, 171, self.config.holo_opacity // 5))

        self.draw_holo_shell(holo)
        self.draw_status_panel(holo)
        self.draw_inventory_panel(holo)
        self.draw_scanlines(holo)
        self.blit_chromatic(holo, (60, 45), int(jitter_x), int(jitter_y))

    def draw_background(self) -> None:
        self.screen.fill((3, 7, 16))
        for i in range(25):
            ratio = i / 24
            r = int(2 + 8 * ratio)
            g = int(7 + 18 * ratio)
            b = int(16 + 30 * ratio)
            pygame.draw.rect(self.screen, (r, g, b), (0, i * 24, self.config.width, 24), 0)

    def draw_holo_shell(self, surf: Any) -> None:
        w, h = surf.get_size()
        border = pygame.Rect(0, 0, w, h)
        pygame.draw.rect(surf, (0, 255, 255, 80), border, 1, border_radius=15)

        title = self.title_font.render("USG ISHIMURA / RIG HOLO-UI", True, COLORS["text_blue"])
        surf.blit(title, (16, 12))
        mode = self.ui_font.render(self.mode_text, True, self.mode_color)
        surf.blit(mode, (w - 200, 14))
        pygame.draw.line(surf, (0, 255, 255, 70), (0, 44), (w, 44), 1)

    def draw_status_panel(self, surf: Any) -> None:
        panel = pygame.Rect(18, 72, 350, 360)
        pygame.draw.rect(surf, (0, 20, 36, 110), panel, 0, border_radius=12)
        pygame.draw.rect(surf, (0, 255, 255, 70), panel, 1, border_radius=12)

        title = self.ui_font.render("STATUS PROJECTION", True, COLORS["text_blue"])
        surf.blit(title, (panel.x + 14, panel.y + 14))
        self.draw_gauge(surf, panel.x + 14, panel.y + 58, 300, "HEALTH", self.health, COLORS["health_teal"])
        self.draw_gauge(surf, panel.x + 14, panel.y + 120, 300, "STASIS", self.stasis, COLORS["stasis_blue"])

        txt = self.small_font.render("OBJECTIVE: DECK B2 | SYSTEM READY", True, COLORS["text_blue"])
        surf.blit(txt, (panel.x + 14, panel.y + 190))

    def draw_gauge(
        self,
        surf: Any,
        x: int,
        y: int,
        width: int,
        label: str,
        value: int,
        glow_color: tuple[int, int, int],
    ) -> None:
        label_text = self.small_font.render(f"{label} {value}%", True, COLORS["text_blue"])
        surf.blit(label_text, (x, y - 20))
        bar_bg = pygame.Rect(x, y, width, 12)
        pygame.draw.rect(surf, (0, 30, 48, 150), bar_bg, 0, border_radius=7)
        pygame.draw.rect(surf, (0, 255, 255, 90), bar_bg, 1, border_radius=7)

        fill = pygame.Rect(x + 1, y + 1, int((width - 2) * (value / 100.0)), 10)
        pygame.draw.rect(surf, (*glow_color, 210), fill, 0, border_radius=6)

    def draw_inventory_panel(self, surf: Any) -> None:
        panel = pygame.Rect(390, 72, 370, 360)
        pygame.draw.rect(surf, (0, 20, 36, 110), panel, 0, border_radius=12)
        pygame.draw.rect(surf, (0, 255, 255, 70), panel, 1, border_radius=12)
        title = self.ui_font.render("INVENTORY GRID 4x4", True, COLORS["text_blue"])
        surf.blit(title, (panel.x + 14, panel.y + 14))

        for i in range(self.config.grid_rows * self.config.grid_cols):
            row = i // self.config.grid_cols
            col = i % self.config.grid_cols
            x = self.grid_origin[0] - 60 + col * (self.slot_size + self.slot_gap)
            y = self.grid_origin[1] - 28 + row * (self.slot_size + self.slot_gap)
            rect = pygame.Rect(x, y, self.slot_size, self.slot_size)

            if i == self.selected:
                pygame.draw.rect(surf, (0, 255, 255, 70), rect, 0, border_radius=10)
                pygame.draw.rect(surf, (0, 255, 255, 180), rect, 2, border_radius=10)
            else:
                pygame.draw.rect(surf, (0, 255, 255, 16), rect, 0, border_radius=10)
                pygame.draw.rect(surf, (0, 255, 255, 80), rect, 1, border_radius=10)

            glyph = self.ui_font.render(self.glyphs[i], True, COLORS["text_blue"])
            surf.blit(glyph, (rect.centerx - glyph.get_width() // 2, rect.centery - glyph.get_height() // 2))

    def draw_scanlines(self, surf: Any) -> None:
        w, h = surf.get_size()
        for y in range(0, h, 4):
            pygame.draw.line(surf, (0, 0, 0, self.config.scanline_alpha), (0, y), (w, y), 1)

    def blit_chromatic(self, surf: Any, pos: tuple[int, int], jitter_x: int, jitter_y: int) -> None:
        x, y = pos
        off = self.config.chromatic_offset

        red = surf.copy()
        red.fill((255, 40, 20, 0), special_flags=pygame.BLEND_RGB_MULT)
        cyan = surf.copy()
        cyan.fill((0, 255, 255, 0), special_flags=pygame.BLEND_RGB_MULT)
        blue = surf.copy()
        blue.fill((51, 153, 255, 0), special_flags=pygame.BLEND_RGB_MULT)

        self.screen.blit(red, (x - off + jitter_x, y + jitter_y), special_flags=pygame.BLEND_ADD)
        self.screen.blit(cyan, (x + jitter_x, y + jitter_y), special_flags=pygame.BLEND_ADD)
        self.screen.blit(blue, (x + off + jitter_x, y + jitter_y), special_flags=pygame.BLEND_ADD)

        footer = self.small_font.render(
            f"SELECTED SLOT {self.selected + 1} / {self.glyphs[self.selected]}   |   ARROWS/WASD MOVE   ENTER INTERACT   SPACE GLITCH",
            True,
            COLORS["text_blue"],
        )
        self.screen.blit(footer, (66, self.config.height - 30))


def main() -> None:
    pygame.init()
    try:
        demo = DeadSpaceRigDemo(DeadSpaceConfig())
        demo.run()
    finally:
        pygame.quit()


if __name__ == "__main__":
    main()
