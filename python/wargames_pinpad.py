from __future__ import annotations

from dataclasses import dataclass
import random
from collections import deque
from typing import TypedDict


COLORS = {
    "phosphor_green": (0, 170, 0),
    "bright_green": (24, 230, 153),
    "dim_green": (16, 84, 14),
    "background": (5, 26, 5),
    "black": (0, 0, 0),
}


@dataclass
class WargamesConfig:
    type_delay_ms: int = 30
    type_variance_ms: int = 15
    newline_pause_ms: int = 170
    cursor_blink_min_ms: int = 530
    cursor_blink_max_ms: int = 1000
    max_lines: int = 300


class TypeItem(TypedDict):
    text: str
    style: str
    index: int
    line_ref: int | None


class WargamesTerminal:
    MENU_LINES = (
        "GAMES LIST:",
        "",
        "1. GLOBAL THERMONUCLEAR WAR",
        "2. POKER",
        "3. CHESS",
        "4. FIGHTER COMBAT",
        "",
        "SELECT: _",
    )

    def __init__(self, config: WargamesConfig | None = None):
        self.config: WargamesConfig = config or WargamesConfig()
        self.lines: list[tuple[str, str]] = []
        self.current_input: str = ""
        self.selected_game: int | None = None
        self.is_typing: bool = True
        self._cursor_visible: bool = True
        self._next_cursor_toggle_ms: int = 0
        self._typing_queue: deque[TypeItem] = deque()
        self._active_type_item: TypeItem | None = None
        self._last_type_ms: int = 0
        self._next_type_delay_ms: int = self._rand_type_delay()

    def _rand_type_delay(self) -> int:
        delta = random.randint(-self.config.type_variance_ms, self.config.type_variance_ms)
        return max(8, self.config.type_delay_ms + delta)

    def _rand_blink_delay(self) -> int:
        return random.randint(self.config.cursor_blink_min_ms, self.config.cursor_blink_max_ms)

    def start(self, now_ms: int) -> None:
        self._next_cursor_toggle_ms = now_ms + self._rand_blink_delay()
        self.queue_block(
            (
                ("", "dim"),
                ("NORAD REMOTE STRATEGIC TERMINAL", "highlight"),
                ("IMSAI 8080 CONNECTED", "dim"),
                ("SECURITY LEVEL: SIMULATION / TRAINING", "dim"),
                ("TYPE HELP FOR COMMAND LIST", "dim"),
                ("", "dim"),
            )
        )
        self.queue_menu()

    def queue_line(self, text: str, style: str = "normal") -> None:
        self._typing_queue.append({"text": text, "style": style, "index": 0, "line_ref": None})

    def queue_block(self, block: tuple[tuple[str, str], ...]) -> None:
        for text, style in block:
            self.queue_line(text, style)

    def queue_menu(self) -> None:
        for line in self.MENU_LINES:
            self.queue_line(line, "normal")

    def update(self, now_ms: int) -> None:
        if now_ms >= self._next_cursor_toggle_ms:
            self._cursor_visible = not self._cursor_visible
            self._next_cursor_toggle_ms = now_ms + self._rand_blink_delay()

        if self._active_type_item is None and self._typing_queue:
            self._active_type_item = self._typing_queue.popleft()
            style = str(self._active_type_item["style"])
            self.lines.append(("", style))
            self._active_type_item["line_ref"] = len(self.lines) - 1
            self._next_type_delay_ms = self._rand_type_delay()
            self._last_type_ms = now_ms

        if self._active_type_item is not None and now_ms - self._last_type_ms >= self._next_type_delay_ms:
            item = self._active_type_item
            text = item["text"]
            index = item["index"]
            line_ref = item["line_ref"]

            if line_ref is None:
                self._active_type_item = None
                self._next_type_delay_ms = self.config.newline_pause_ms
                self._last_type_ms = now_ms
                return

            if index < len(text):
                current_text, style = self.lines[line_ref]
                self.lines[line_ref] = (current_text + text[index], style)
                item["index"] = index + 1
                self._next_type_delay_ms = self._rand_type_delay()
            else:
                self._active_type_item = None
                self._next_type_delay_ms = self.config.newline_pause_ms

            self._last_type_ms = now_ms

        self.is_typing = self._active_type_item is not None or bool(self._typing_queue)

        if len(self.lines) > self.config.max_lines:
            self.lines = self.lines[-self.config.max_lines :]

    def handle_enter(self) -> None:
        command = self.current_input.strip()
        self.queue_line(f"NORAD> {command}")
        self.current_input = ""

        if not command:
            self.queue_line("ENTER COMMAND. TRY HELP.", "dim")
            return

        upper = command.upper()

        if upper in ("HELP", "?", "MAN"):
            self.queue_block(
                (
                    ("AVAILABLE COMMANDS:", "dim"),
                    ("  LIST              SHOW GAMES LIST", "dim"),
                    ("  SELECT <1-4>      CHOOSE A GAME", "dim"),
                    ("  PLAY <NAME|#>     START SELECTED GAME", "dim"),
                    ("  STATUS            SHOW CURRENT TARGET", "dim"),
                    ("  CLEAR             CLEAR TERMINAL", "dim"),
                    ("  HELP              SHOW THIS MESSAGE", "dim"),
                )
            )
            return

        if upper in ("LIST", "GAMES"):
            self.queue_menu()
            return

        if upper.startswith("SELECT "):
            token = upper.replace("SELECT", "", 1).strip()
            selection = self._parse_selection(token)
            if selection is None:
                self.queue_line("INVALID SELECTION. CHOOSE 1-4.", "dim")
                return
            self.selected_game = selection
            self.queue_line(f"SELECTION ACCEPTED: {self._game_name(selection)}", "highlight")
            self.queue_line("TYPE PLAY TO EXECUTE.", "dim")
            return

        if upper.startswith("PLAY"):
            selection = self.selected_game
            if upper != "PLAY":
                token = upper.replace("PLAY", "", 1).strip()
                selection = self._parse_selection(token)

            if selection is None:
                self.queue_line("NO GAME SELECTED. USE SELECT <1-4>.", "dim")
                return

            if selection == 1:
                self.queue_block(
                    (
                        ("SIMULATION BOOTSTRAP: GLOBAL THERMONUCLEAR WAR", "highlight"),
                        ("CONNECTING TO WOPR...", "highlight"),
                        ("GREETINGS PROFESSOR FALKEN.", "highlight"),
                        ("SHALL WE PLAY A GAME?", "highlight"),
                    )
                )
                return

            self.queue_line(f"{self._game_name(selection)} NOT INSTALLED ON THIS NODE.", "dim")
            self.queue_line("RECOMMENDED: GLOBAL THERMONUCLEAR WAR", "highlight")
            return

        if upper == "STATUS":
            if self.selected_game is None:
                self.queue_line("STATUS: IDLE / NO ACTIVE GAME", "dim")
            else:
                self.queue_line(f"STATUS: READY / TARGET={self._game_name(self.selected_game)}", "highlight")
            return

        if upper in ("CLEAR", "CLS"):
            self.lines.clear()
            return

        self.queue_line(f"UNKNOWN COMMAND: {upper}", "dim")
        self.queue_line("TYPE HELP FOR COMMAND INDEX.", "dim")

    def _parse_selection(self, token: str) -> int | None:
        if token == "1" or "GLOBAL" in token:
            return 1
        if token == "2" or "POKER" in token:
            return 2
        if token == "3" or "CHESS" in token:
            return 3
        if token == "4" or "FIGHTER" in token:
            return 4
        return None

    def _game_name(self, selection: int) -> str:
        return {
            1: "GLOBAL THERMONUCLEAR WAR",
            2: "POKER",
            3: "CHESS",
            4: "FIGHTER COMBAT",
        }[selection]

    @property
    def cursor_visible(self) -> bool:
        return self._cursor_visible


if __name__ == "__main__":
    import importlib
    import sys

    pygame = importlib.import_module("pygame")

    pygame.init()
    width, height = 980, 650
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption("WarGames (1983) NORAD Terminal")

    font_size = 24
    small_size = 17
    try:
        font = pygame.font.SysFont("couriernew", font_size)
        header_font = pygame.font.SysFont("couriernew", small_size, bold=True)
    except Exception:
        font = pygame.font.Font(None, font_size)
        header_font = pygame.font.Font(None, small_size)

    terminal = WargamesTerminal()
    clock = pygame.time.Clock()
    terminal.start(pygame.time.get_ticks())

    grain_surface = pygame.Surface((width, height), pygame.SRCALPHA)
    last_grain_update = 0
    grain_interval_ms = 80
    scanline_surface = pygame.Surface((width, height), pygame.SRCALPHA)

    for y in range(0, height, 4):
        pygame.draw.line(scanline_surface, (0, 0, 0, 72), (0, y), (width, y), 2)

    curvature_edge_darkness = pygame.Surface((width, height), pygame.SRCALPHA)
    for y in range(height):
        for x in range(0, width, 8):
            dx = (x - width / 2) / (width / 2)
            dy = (y - height / 2) / (height / 2)
            dist = (dx * dx + dy * dy) ** 0.5
            alpha = int(min(180, dist * dist * 80))
            pygame.draw.line(curvature_edge_darkness, (0, 0, 0, alpha), (x, y), (x + 8, y))

    phosphor_glow_overlay = pygame.Surface((width, height), pygame.SRCALPHA)
    phosphor_glow_overlay.fill((0, 85, 0, 12))

    screen_flicker_timer_ms = 0
    screen_flicker_active = False
    screen_flicker_interval_ms = 8500

    def render_text_with_bloom(font, text, main_color, glow_base_color):
        surf = font.render(text, True, main_color)
        inner_glow = font.render(text, True, glow_base_color)
        outer_glow = font.render(text, True, (0, 100, 0))
        return surf, inner_glow, outer_glow

    def refresh_grain() -> None:
        grain_surface.fill((0, 0, 0, 0))
        for _ in range(1500):
            x = random.randint(0, width - 1)
            y = random.randint(0, height - 1)
            intensity = random.randint(16, 58)
            grain_surface.set_at((x, y), (0, min(255, intensity + 36), intensity // 3, 22))

    running = True
    while running:
        now_ms = pygame.time.get_ticks()

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN and not terminal.is_typing:
                if event.key == pygame.K_RETURN:
                    terminal.handle_enter()
                elif event.key == pygame.K_BACKSPACE:
                    terminal.current_input = terminal.current_input[:-1]
                elif event.key == pygame.K_ESCAPE:
                    terminal.current_input = ""
                elif event.unicode and 32 <= ord(event.unicode) <= 126:
                    terminal.current_input += event.unicode.upper()

        terminal.update(now_ms)

        if now_ms - last_grain_update >= grain_interval_ms:
            refresh_grain()
            last_grain_update = now_ms

        screen_flicker_timer_ms += 16
        if screen_flicker_timer_ms >= screen_flicker_interval_ms:
            if screen_flicker_timer_ms - screen_flicker_interval_ms < 150:
                screen_flicker_active = True
            else:
                screen_flicker_active = False
                screen_flicker_timer_ms = 0

        screen.fill(COLORS["black"])
        frame_rect = pygame.Rect(16, 16, width - 32, height - 32)
        screen.fill(COLORS["background"], frame_rect)
        pygame.draw.rect(screen, COLORS["dim_green"], frame_rect, 1, border_radius=12)

        screen.blit(phosphor_glow_overlay, (0, 0))

        center = (width // 2, height // 2)
        vignette = pygame.Surface((width, height), pygame.SRCALPHA)
        for radius in range(min(width, height) // 2, 0, -12):
            alpha = min(170, max(0, int((radius / (min(width, height) // 2)) * 4)))
            pygame.draw.circle(vignette, (0, 0, 0, alpha), center, radius, 1)
        screen.blit(vignette, (0, 0))

        header = "NORAD STRATEGIC RESPONSE SYSTEM // IMSAI 8080 REMOTE NODE"
        header_main, header_inner, header_outer = render_text_with_bloom(
            header_font, header, COLORS["bright_green"], COLORS["phosphor_green"]
        )
        screen.blit(header_outer, (30, 31))
        screen.blit(header_inner, (30, 30))
        screen.blit(header_main, (30, 29))
        pygame.draw.line(screen, COLORS["dim_green"], (28, 52), (width - 28, 52), 1)

        output_top = 66
        line_h = font_size + 5
        max_visible = (height - 140) // line_h
        visible_lines = terminal.lines[-max_visible:]

        y = output_top
        for text, style in visible_lines:
            if style == "highlight":
                color = COLORS["bright_green"]
            elif style == "dim":
                color = COLORS["dim_green"]
            else:
                color = COLORS["phosphor_green"]

            main_surf, inner_glow, outer_glow = render_text_with_bloom(
                font, text, color, COLORS["phosphor_green"]
            )
            screen.blit(outer_glow, (31, y + 2))
            screen.blit(inner_glow, (31, y + 1))
            screen.blit(main_surf, (30, y))
            y += line_h

        prompt_label = "NORAD> "
        input_text = terminal.current_input
        prompt_line_y = height - 58
        prompt_main, prompt_inner, prompt_outer = render_text_with_bloom(
            font, prompt_label + input_text, COLORS["bright_green"], COLORS["phosphor_green"]
        )
        screen.blit(prompt_outer, (31, prompt_line_y + 2))
        screen.blit(prompt_inner, (31, prompt_line_y + 1))
        screen.blit(prompt_main, (30, prompt_line_y))

        if terminal.cursor_visible:
            cursor_x = 30 + font.size(prompt_label + input_text)[0] + 2
            for i in range(3):
                alpha = 180 - i * 50
                expand = i * 2
                glow_rect = pygame.Rect(
                    cursor_x - expand, prompt_line_y + 2 - expand,
                    12 + expand * 2, font_size - 2 + expand * 2
                )
                glow_surf = pygame.Surface((glow_rect.width, glow_rect.height), pygame.SRCALPHA)
                pygame.draw.rect(glow_surf, (*COLORS["phosphor_green"], alpha), glow_surf.get_rect())
                screen.blit(glow_surf, glow_rect.topleft)
            pygame.draw.rect(screen, COLORS["phosphor_green"], (cursor_x, prompt_line_y + 2, 12, font_size - 2))

        screen.blit(curvature_edge_darkness, (0, 0))
        screen.blit(grain_surface, (0, 0))
        screen.blit(scanline_surface, (0, 0))

        if screen_flicker_active:
            flicker_overlay = pygame.Surface((width, height), pygame.SRCALPHA)
            flicker_overlay.fill((0, 60, 0, 25))
            screen.blit(flicker_overlay, (0, 0))

        pygame.display.flip()
        clock.tick(60)

    pygame.quit()
    sys.exit(0)
