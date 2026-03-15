from dataclasses import dataclass
from enum import Enum, auto
from typing import Final
import math
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
    scanline_cycle_ms: int = 2800
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


@dataclass(frozen=True)
class KeyVisualPulse:
    label: str
    outer_bloom: float
    inner_core: float


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
        progress = elapsed / self.config.boot_ms
        if progress < 0.0:
            return 0.0
        if progress > 1.0:
            return 1.0
        return progress

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

    def key_visual_pulse(self, now_ms: float | None = None) -> KeyVisualPulse | None:
        label, intensity = self.flashing_button_intensity(now_ms)
        if label is None:
            return None
        outer_bloom = 0.5 + (0.9 * intensity)
        inner_core = 0.6 + (0.4 * intensity)
        return KeyVisualPulse(label=label, outer_bloom=outer_bloom, inner_core=inner_core)

    def boot_glitch_intensity(self, now_ms: float | None = None) -> float:
        if self.state != AuthState.BOOTING:
            return 0.0
        progress = self.get_boot_progress(now_ms)
        value = math.pow(1.0 - progress, 0.65)
        if value < 0.0:
            return 0.0
        if value > 1.0:
            return 1.0
        return value

    def scanline_offset(self, now_ms: float | None = None) -> float:
        if now_ms is None:
            now_ms = time.time() * 1000
        cycle = max(1, self.config.scanline_cycle_ms)
        return ((now_ms % cycle) / cycle)

    def panel_glow_profile(self, now_ms: float | None = None) -> tuple[float, float]:
        glitch = self.boot_glitch_intensity(now_ms)
        if self.state == AuthState.SUCCESS:
            return (0.5, 0.45)
        if self.state == AuthState.ERROR:
            return (0.42, 0.2)
        return (0.28 + glitch * 0.42, 0.22 + glitch * 0.3)


if __name__ == "__main__":
    raise SystemExit("Use this module from a renderer/integration layer.")
