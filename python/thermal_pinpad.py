"""
Thermal Pinpad - Splinter Cell Style

A Python implementation of the Splinter Cell thermal vision keypad effect.
Core classes are platform-agnostic and portable to LVGL/MicroPython.

Usage:
    # As a library
    from thermal_pinpad import ThermalKeypad, ThermalPalette, ThermalColorMapper
    
    keypad = ThermalKeypad()
    keypad.press_button('1')
    intensities = keypad.get_all_intensities()
    
    mapper = ThermalColorMapper()
    rgb = mapper.intensity_to_rgb(0.8, ThermalPalette.SPLINTER_CELL)
    
    # Run demo
    python thermal_pinpad.py
"""

from dataclasses import dataclass
from enum import Enum
from typing import Dict, Optional, Tuple, Union
import math
import random
import time


# =============================================================================
# CORE CLASSES (Platform-Agnostic)
# =============================================================================

@dataclass
class ThermalConfig:
    """Configuration for thermal effect parameters.
    
    Attributes:
        decay_time_ms: Time in milliseconds for full heat decay (default: 30000)
        min_visible_intensity: Minimum intensity to render (default: 0.02)
        num_rings: Number of concentric glow rings (default: 10)
    """
    decay_time_ms: int = 30000
    min_visible_intensity: float = 0.02
    num_rings: int = 10


class ThermalPalette(Enum):
    """Available thermal color palettes."""
    SPLINTER_CELL = "splinter"
    CLASSIC = "classic"
    IRONBOW = "ironbow"
    HOT_COLD = "hotcold"


class ThermalButton:
    """Represents a single button with thermal state.
    
    Tracks when the button was pressed and calculates current heat intensity
    based on exponential decay over time.
    """
    
    def __init__(self, label: str):
        """Initialize a thermal button.
        
        Args:
            label: The button's display label (e.g., '1', '*', '#')
        """
        self.label = label
        self._pressed_at: Optional[float] = None
    
    @property
    def intensity(self) -> float:
        """Current intensity (convenience property using current time)."""
        return self.get_intensity(time.time() * 1000)
    
    @property
    def pressed_at(self) -> Optional[float]:
        """Timestamp when button was pressed (milliseconds since epoch)."""
        return self._pressed_at
    
    def press(self) -> None:
        """Record a button press at the current time."""
        self._pressed_at = time.time() * 1000
    
    def press_at(self, timestamp_ms: float) -> None:
        """Record a button press at a specific timestamp.
        
        Args:
            timestamp_ms: Press time in milliseconds since epoch
        """
        self._pressed_at = timestamp_ms
    
    def get_intensity(self, now_ms: float, config: Optional[ThermalConfig] = None) -> float:
        """Calculate current heat intensity based on decay formula.
        
        Uses exponential decay: intensity = e^(-decay_progress * 3)
        
        Args:
            now_ms: Current time in milliseconds
            config: Thermal configuration (uses defaults if None)
            
        Returns:
            Intensity value between 0.0 and 1.0
        """
        if config is None:
            config = ThermalConfig()
        
        if self._pressed_at is None:
            return 0.0
        
        elapsed = now_ms - self._pressed_at
        decay_progress = elapsed / config.decay_time_ms
        
        # Exponential decay formula
        intensity = math.exp(-decay_progress * 3)
        
        return max(0.0, min(1.0, intensity))
    
    def reset(self) -> None:
        """Clear the heat signature (reset to unpressed state)."""
        self._pressed_at = None


class ThermalKeypad:
    """Manages all 12 buttons in a telephone keypad layout.
    
    Provides methods to press buttons, query intensities, and reset heat.
    """
    
    BUTTONS = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '*', '0', '#']
    
    def __init__(self, config: Optional[ThermalConfig] = None):
        """Initialize the keypad with all buttons.
        
        Args:
            config: Thermal configuration (uses defaults if None)
        """
        self.config = config or ThermalConfig()
        self._buttons: Dict[str, ThermalButton] = {
            label: ThermalButton(label) for label in self.BUTTONS
        }
    
    def press_button(self, label: str) -> bool:
        """Press a button by label.
        
        Args:
            label: The button label to press
            
        Returns:
            True if button exists and was pressed, False otherwise
        """
        if label in self._buttons:
            self._buttons[label].press()
            return True
        return False
    
    def press_button_at(self, label: str, timestamp_ms: float) -> bool:
        """Press a button at a specific timestamp.
        
        Args:
            label: The button label to press
            timestamp_ms: Press time in milliseconds
            
        Returns:
            True if button exists and was pressed, False otherwise
        """
        if label in self._buttons:
            self._buttons[label].press_at(timestamp_ms)
            return True
        return False
    
    def get_intensity(self, label: str, now_ms: Optional[float] = None) -> float:
        """Get current intensity for a specific button.
        
        Args:
            label: The button label
            now_ms: Current time (uses time.time() if None)
            
        Returns:
            Intensity value between 0.0 and 1.0, or 0.0 if button not found
        """
        if label not in self._buttons:
            return 0.0
        if now_ms is None:
            now_ms = time.time() * 1000
        return self._buttons[label].get_intensity(now_ms, self.config)
    
    def get_all_intensities(self, now_ms: Optional[float] = None) -> Dict[str, float]:
        """Get intensities for all buttons.
        
        Args:
            now_ms: Current time (uses time.time() if None)
            
        Returns:
            Dictionary mapping button labels to intensity values
        """
        if now_ms is None:
            now_ms = time.time() * 1000
        return {
            label: button.get_intensity(now_ms, self.config)
            for label, button in self._buttons.items()
        }
    
    def get_button(self, label: str) -> Optional[ThermalButton]:
        """Get a specific button object.
        
        Args:
            label: The button label
            
        Returns:
            ThermalButton instance or None if not found
        """
        return self._buttons.get(label)
    
    def reset(self) -> None:
        """Clear all heat signatures."""
        for button in self._buttons.values():
            button.reset()
    
    def enter_code(self, code: str, interval_ms: float = 200) -> None:
        """Simulate entering a code with delays between presses.
        
        Args:
            code: String of button labels to press
            interval_ms: Milliseconds between each press
        """
        now_ms = time.time() * 1000
        for i, char in enumerate(code):
            if char in self._buttons:
                self._buttons[char].press_at(now_ms + i * interval_ms)


class ThermalColorMapper:
    """Maps intensity values to RGB colors using various palettes.
    
    For Splinter Cell palette, uses HSV interpolation for perceptually smooth gradients.
    Other palettes use RGB interpolation.
    """
    
    # Gamma curve for better perceptual separation (γ=1.3)
    GAMMA = 1.3
    
    # HSV stops for Splinter Cell palette (hue: 0-360, sat: 0-100, val: 0-100)
    _SPLINTER_HSV_STOPS = [
        (0.0,  230, 85, 12),   # deep blue-black
        (0.2,  225, 90, 43),   # cobalt
        (0.4,  195, 88, 67),   # cyan
        (0.62, 150, 58, 75),   # green
        (0.8,  55,  60, 86),   # yellow
        (0.92, 48,  90, 94),   # warm yellow
        (1.0,  42,  88, 94),   # yellow-orange peak
    ]
    
    # Color palette definitions: list of (t, [r, g, b])
    _PALETTES = {
        ThermalPalette.SPLINTER_CELL: None,  # Built from HSV LUT at init
        ThermalPalette.CLASSIC: [
            (0.0,  [0, 0, 40]),
            (0.2,  [0, 80, 160]),
            (0.4,  [0, 200, 200]),
            (0.6,  [200, 200, 0]),
            (0.8,  [255, 128, 0]),
            (1.0,  [255, 50, 50]),
        ],
        ThermalPalette.IRONBOW: [
            (0.0,  [0, 0, 0]),
            (0.2,  [40, 0, 60]),
            (0.4,  [150, 0, 50]),
            (0.6,  [255, 80, 0]),
            (0.8,  [255, 220, 80]),
            (1.0,  [255, 255, 255]),
        ],
        ThermalPalette.HOT_COLD: [
            (0.0,  [0, 0, 100]),
            (0.3,  [50, 50, 200]),
            (0.5,  [255, 255, 255]),
            (0.7,  [255, 150, 50]),
            (1.0,  [255, 0, 0]),
        ],
    }
    
    def __init__(self):
        self._splinter_lut = self._build_hsv_lut()
        self._PALETTES[ThermalPalette.SPLINTER_CELL] = self._splinter_lut
    
    @staticmethod
    def _hsv_to_rgb(h: float, s: float, v: float) -> Tuple[int, int, int]:
        s /= 100
        v /= 100
        c = v * s
        x = c * (1 - abs(((h / 60) % 2) - 1))
        m = v - c
        
        if h < 60:
            r, g, b = c, x, 0
        elif h < 120:
            r, g, b = x, c, 0
        elif h < 180:
            r, g, b = 0, c, x
        elif h < 240:
            r, g, b = 0, x, c
        elif h < 300:
            r, g, b = x, 0, c
        else:
            r, g, b = c, 0, x
        
        return (
            round((r + m) * 255),
            round((g + m) * 255),
            round((b + m) * 255)
        )
    
    def _build_hsv_lut(self, size: int = 256) -> list:
        stops = self._SPLINTER_HSV_STOPS
        lut = []
        
        for i in range(size):
            t = i / (size - 1)
            gamma_t = t ** self.GAMMA
            
            lower = stops[0]
            upper = stops[-1]
            
            for j in range(len(stops) - 1):
                if stops[j][0] <= gamma_t <= stops[j + 1][0]:
                    lower = stops[j]
                    upper = stops[j + 1]
                    break
            
            range_t = upper[0] - lower[0]
            factor = 0.0 if range_t == 0 else (gamma_t - lower[0]) / range_t
            
            h = lower[1] + (upper[1] - lower[1]) * factor
            s = lower[2] + (upper[2] - lower[2]) * factor
            v = lower[3] + (upper[3] - lower[3]) * factor
            
            lut.append(self._hsv_to_rgb(h, s, v))
        
        return lut
    
    def intensity_to_rgb(
        self, 
        intensity: float, 
        palette: ThermalPalette = ThermalPalette.SPLINTER_CELL
    ) -> Tuple[int, int, int]:
        intensity = max(0.0, min(1.0, intensity))
        
        if palette == ThermalPalette.SPLINTER_CELL:
            idx = min(255, max(0, int(intensity * 255)))
            return self._splinter_lut[idx]
        
        stops = self._PALETTES[palette]
        
        lower = stops[0]
        upper = stops[-1]
        
        for i in range(len(stops) - 1):
            if stops[i][0] <= intensity <= stops[i + 1][0]:
                lower = stops[i]
                upper = stops[i + 1]
                break
        
        t_lower, color_lower = lower
        t_upper, color_upper = upper
        
        range_t = t_upper - t_lower
        factor = 0.0 if range_t == 0 else (intensity - t_lower) / range_t
        
        r = round(color_lower[0] + (color_upper[0] - color_lower[0]) * factor)
        g = round(color_lower[1] + (color_upper[1] - color_lower[1]) * factor)
        b = round(color_lower[2] + (color_upper[2] - color_lower[2]) * factor)
        
        return (r, g, b)
    
    def intensity_to_hex(
        self, 
        intensity: float, 
        palette: ThermalPalette = ThermalPalette.SPLINTER_CELL
    ) -> str:
        """Convert intensity to hex color string.
        
        Args:
            intensity: Intensity value between 0.0 and 1.0
            palette: The color palette to use
            
        Returns:
            Hex color string (e.g., '#ff8800')
        """
        r, g, b = self.intensity_to_rgb(intensity, palette)
        return f'#{r:02x}{g:02x}{b:02x}'


def calculate_ring_intensity(
    base_intensity: float, 
    ring_index: int, 
    total_rings: int
) -> float:
    """Calculate intensity for a specific ring with quadratic falloff.
    
    ring_intensity = base_intensity * (1 - ring_index/total_rings)^2
    
    Args:
        base_intensity: The button's base intensity
        ring_index: Ring number (0 = innermost)
        total_rings: Total number of rings
        
    Returns:
        Ring intensity with quadratic falloff applied
    """
    falloff = (1 - (ring_index / total_rings)) ** 2
    return base_intensity * falloff


# =============================================================================
# TKINTER DEMO
# =============================================================================

if __name__ == "__main__":
    import tkinter as tk
    from tkinter import ttk
    
    # Demo configuration
    CANVAS_WIDTH = 320
    CANVAS_HEIGHT = 400
    BUTTON_WIDTH = 80
    BUTTON_HEIGHT = 60
    BUTTON_GAP = 10
    MARGIN_X = 25
    MARGIN_Y = 30
    BUTTONS_PER_ROW = 3
    TARGET_FPS = 60
    NOISE_POINTS = 140
    SCANLINE_SPACING = 3
    
    class ThermalPinpadDemo:
        """Tkinter demo application for the thermal pinpad."""
        
        def __init__(self, root: tk.Tk):
            self.root = root
            self.root.title("Thermal Pinpad - Splinter Cell Style")
            self.root.configure(bg='#050508')
            self.root.resizable(False, False)
            
            # Initialize thermal components
            self.config = ThermalConfig()
            self.keypad = ThermalKeypad(self.config)
            self.color_mapper = ThermalColorMapper()
            self.current_palette = ThermalPalette.SPLINTER_CELL
            
            # Demo mode state
            self.demo_mode = False
            self.demo_job = None
            
            # Build UI
            self._build_ui()
            
            # Start animation loop
            self._animate()
        
        def _build_ui(self):
            """Build the user interface."""
            # Main container
            container = tk.Frame(self.root, bg='#0a0a14', padx=20, pady=20)
            container.pack(padx=10, pady=10)
            
            # Header
            header = tk.Frame(container, bg='#0a0a14')
            header.pack(pady=(0, 10))
            
            title = tk.Label(
                header, 
                text="THERMAL VISION",
                font=('Consolas', 14),
                fg='#3a4a5a',
                bg='#0a0a14'
            )
            title.pack()
            
            subtitle = tk.Label(
                header,
                text="Keypad Analysis System",
                font=('Consolas', 9),
                fg='#3a4a5a',
                bg='#0a0a14'
            )
            subtitle.pack()
            
            # Canvas for keypad
            self.canvas = tk.Canvas(
                container,
                width=CANVAS_WIDTH,
                height=CANVAS_HEIGHT,
                bg='#050508',
                highlightthickness=1,
                highlightbackground='#1a2a3a'
            )
            self.canvas.pack()
            self.canvas.bind('<Button-1>', self._on_canvas_click)
            
            # Status bar
            status_frame = tk.Frame(container, bg='#0a0a14')
            status_frame.pack(fill='x', pady=(15, 0))
            
            self.status_var = tk.StringVar(value="READY - Click buttons to simulate presses")
            status_label = tk.Label(
                status_frame,
                textvariable=self.status_var,
                font=('Consolas', 10),
                fg='#0af',
                bg='#0a0a14',
                padx=10,
                pady=5
            )
            status_label.pack(fill='x')
            
            # Controls
            controls = tk.Frame(container, bg='#0a0a14')
            controls.pack(fill='x', pady=(15, 0))
            
            # Style for buttons
            style = ttk.Style()
            style.configure('Thermal.TButton', padding=5)
            
            btn_frame = tk.Frame(controls, bg='#0a0a14')
            btn_frame.pack()
            
            # Demo button
            self.demo_btn = tk.Button(
                btn_frame,
                text="Demo Mode",
                font=('Consolas', 9),
                fg='#6a8a9a',
                bg='#0a1e32',
                activebackground='#143c50',
                activeforeground='#0af',
                relief='flat',
                padx=10,
                pady=5,
                command=self._toggle_demo_mode
            )
            self.demo_btn.pack(side='left', padx=5)
            
            # Reset button
            reset_btn = tk.Button(
                btn_frame,
                text="Reset",
                font=('Consolas', 9),
                fg='#6a8a9a',
                bg='#0a1e32',
                activebackground='#143c50',
                activeforeground='#0af',
                relief='flat',
                padx=10,
                pady=5,
                command=self._reset_heat
            )
            reset_btn.pack(side='left', padx=5)
            
            # Random code button
            random_btn = tk.Button(
                btn_frame,
                text="Random Code",
                font=('Consolas', 9),
                fg='#6a8a9a',
                bg='#0a1e32',
                activebackground='#143c50',
                activeforeground='#0af',
                relief='flat',
                padx=10,
                pady=5,
                command=self._enter_random_code
            )
            random_btn.pack(side='left', padx=5)
            
            # Palette dropdown
            palette_frame = tk.Frame(controls, bg='#0a0a14')
            palette_frame.pack(pady=(10, 0))
            
            tk.Label(
                palette_frame,
                text="Palette:",
                font=('Consolas', 9),
                fg='#6a8a9a',
                bg='#0a0a14'
            ).pack(side='left', padx=(0, 5))
            
            self.palette_var = tk.StringVar(value="Splinter Cell")
            palette_options = ["Splinter Cell", "Classic", "Ironbow", "Hot/Cold"]
            
            palette_menu = ttk.Combobox(
                palette_frame,
                textvariable=self.palette_var,
                values=palette_options,
                state='readonly',
                width=12,
                font=('Consolas', 9)
            )
            palette_menu.pack(side='left')
            palette_menu.bind('<<ComboboxSelected>>', self._on_palette_change)
            
            # Instructions
            instructions = tk.Label(
                container,
                text="Brightness indicates recency: brightest = most recent press",
                font=('Consolas', 8),
                fg='#3a4a5a',
                bg='#0a0a14'
            )
            instructions.pack(pady=(15, 0))
        
        def _get_button_position(self, index: int) -> Tuple[int, int]:
            """Get the top-left position of a button by index."""
            row = index // BUTTONS_PER_ROW
            col = index % BUTTONS_PER_ROW
            x = MARGIN_X + col * (BUTTON_WIDTH + BUTTON_GAP)
            y = MARGIN_Y + row * (BUTTON_HEIGHT + BUTTON_GAP)
            return (x, y)
        
        def _get_button_at_position(self, x: int, y: int) -> Optional[str]:
            """Find which button is at the given canvas coordinates."""
            for i, label in enumerate(ThermalKeypad.BUTTONS):
                bx, by = self._get_button_position(i)
                if bx <= x <= bx + BUTTON_WIDTH and by <= y <= by + BUTTON_HEIGHT:
                    return label
            return None
        
        def _on_canvas_click(self, event):
            """Handle canvas click to detect button press."""
            label = self._get_button_at_position(event.x, event.y)
            if label:
                self.keypad.press_button(label)
                self.status_var.set(f'Button "{label}" pressed')
        
        def _toggle_demo_mode(self):
            """Toggle demo mode on/off."""
            self.demo_mode = not self.demo_mode
            
            if self.demo_mode:
                self.demo_btn.configure(text="Stop Demo", bg='#0a4a5a')
                self.status_var.set("DEMO MODE ACTIVE - Auto-entering codes...")
                self._run_demo_sequence()
            else:
                self.demo_btn.configure(text="Demo Mode", bg='#0a1e32')
                self.status_var.set("Demo mode stopped")
                if self.demo_job:
                    self.root.after_cancel(self.demo_job)
                    self.demo_job = None
        
        def _run_demo_sequence(self):
            """Run the demo sequence - enter codes every 8 seconds."""
            if not self.demo_mode:
                return
            
            self._enter_random_code()
            self.demo_job = self.root.after(8000, self._run_demo_sequence)
        
        def _reset_heat(self):
            """Reset all heat signatures."""
            self.keypad.reset()
            self.status_var.set("All heat signatures cleared")
        
        def _enter_random_code(self):
            """Enter a random 4-digit code."""
            import random
            digits = list('0123456789')
            code = ''.join(random.choices(digits, k=4))
            
            # Enter with timing
            now_ms = time.time() * 1000
            for i, digit in enumerate(code):
                self.keypad.press_button_at(digit, now_ms + i * 200)
            
            self.status_var.set(f"Code entered: {' '.join(code)}")
        
        def _on_palette_change(self, event):
            """Handle palette selection change."""
            palette_name = self.palette_var.get()
            palette_map = {
                "Splinter Cell": ThermalPalette.SPLINTER_CELL,
                "Classic": ThermalPalette.CLASSIC,
                "Ironbow": ThermalPalette.IRONBOW,
                "Hot/Cold": ThermalPalette.HOT_COLD,
            }
            self.current_palette = palette_map.get(palette_name, ThermalPalette.SPLINTER_CELL)
            self.status_var.set(f"Palette changed to: {palette_name}")
        
        def _animate(self):
            """Animation loop at target FPS."""
            self._render()
            self.root.after(1000 // TARGET_FPS, self._animate)
        
        def _render(self):
            """Render the keypad to the canvas."""
            self.canvas.delete('all')
            
            # Draw background gradient effect
            self._draw_background()
            
            # Get current intensities
            now_ms = time.time() * 1000
            intensities = self.keypad.get_all_intensities(now_ms)
            
            # Draw all buttons
            for i, label in enumerate(ThermalKeypad.BUTTONS):
                self._draw_button(i, label, intensities[label])

            self._draw_noise()
            self._draw_scanlines()
        
        def _draw_background(self):
            """Draw subtle background gradient."""
            self.canvas.create_rectangle(
                0, 0, CANVAS_WIDTH, CANVAS_HEIGHT,
                fill='#050f1e', outline=''
            )

            cx, cy = CANVAS_WIDTH // 2, CANVAS_HEIGHT // 2
            for i in range(10, 0, -1):
                ratio = i / 10
                alpha = int(22 * ratio)
                color = f'#{alpha:02x}{int(alpha*2.0):02x}{int(alpha*4.0):02x}'
                size = int(max(CANVAS_WIDTH, CANVAS_HEIGHT) * ratio)
                self.canvas.create_oval(
                    cx - size, cy - size, cx + size, cy + size,
                    fill=color, outline=''
                )

            vignette = 60
            self.canvas.create_rectangle(0, 0, vignette, CANVAS_HEIGHT, fill='#000000', stipple='gray50', outline='')
            self.canvas.create_rectangle(CANVAS_WIDTH - vignette, 0, CANVAS_WIDTH, CANVAS_HEIGHT, fill='#000000', stipple='gray50', outline='')
            self.canvas.create_rectangle(0, 0, CANVAS_WIDTH, vignette, fill='#000000', stipple='gray50', outline='')
            self.canvas.create_rectangle(0, CANVAS_HEIGHT - vignette, CANVAS_WIDTH, CANVAS_HEIGHT, fill='#000000', stipple='gray50', outline='')

        def _draw_noise(self):
            import random
            for _ in range(NOISE_POINTS):
                x = random.randint(0, CANVAS_WIDTH - 1)
                y = random.randint(0, CANVAS_HEIGHT - 1)
                n = random.randint(10, 36)
                c = f'#{n:02x}{min(255, n + 20):02x}{min(255, n + 45):02x}'
                self.canvas.create_rectangle(x, y, x + 1, y + 1, fill=c, outline='')

        def _draw_scanlines(self):
            for y in range(0, CANVAS_HEIGHT, SCANLINE_SPACING):
                self.canvas.create_line(0, y, CANVAS_WIDTH, y, fill='#000000', stipple='gray50')
        
        def _draw_button(self, index: int, label: str, intensity: float):
            """Draw a single button with thermal effect."""
            x, y = self._get_button_position(index)
            cx = x + BUTTON_WIDTH // 2
            cy = y + BUTTON_HEIGHT // 2
            
            # Draw thermal glow if intensity is above minimum
            if intensity >= self.config.min_visible_intensity:
                self._draw_thermal_glow(cx, cy, intensity)
            
            # Draw button background
            self._draw_button_background(x, y, intensity)
            
            # Draw button border
            self._draw_button_border(x, y, intensity)
            
            # Draw button label
            self._draw_button_label(cx, cy, label, intensity)
        
        def _draw_thermal_glow(self, cx: int, cy: int, base_intensity: float):
            """Draw concentric circle thermal glow effect."""
            max_radius = max(BUTTON_WIDTH, BUTTON_HEIGHT) * 1.1
            layers = [
                (max_radius, base_intensity * 0.26),
                (max_radius * 0.7, base_intensity * 0.34),
                (max_radius * 0.42, base_intensity * 0.42),
            ]

            for radius, value in layers:
                r, g, b = self.color_mapper.intensity_to_rgb(value, self.current_palette)
                bg_r, bg_g, bg_b = 5, 15, 30
                alpha = min(0.6, value)
                final_r = int(r * alpha + bg_r * (1 - alpha))
                final_g = int(g * alpha + bg_g * (1 - alpha))
                final_b = int(b * alpha + bg_b * (1 - alpha))
                color = f'#{final_r:02x}{final_g:02x}{final_b:02x}'
                self.canvas.create_oval(cx - radius, cy - radius, cx + radius, cy + radius, fill=color, outline='')
        
        def _draw_button_background(self, x: int, y: int, intensity: float):
            """Draw button background."""
            if intensity < self.config.min_visible_intensity:
                # Cold button - dark background
                color = '#0f141e'
            else:
                # Warm button - tinted background
                r, g, b = self.color_mapper.intensity_to_rgb(
                    intensity * 0.3, self.current_palette
                )
                # Blend with dark base
                base_r, base_g, base_b = 15, 20, 30
                alpha = 0.4
                final_r = int(r * alpha + base_r * (1 - alpha))
                final_g = int(g * alpha + base_g * (1 - alpha))
                final_b = int(b * alpha + base_b * (1 - alpha))
                color = f'#{final_r:02x}{final_g:02x}{final_b:02x}'
            
            self._draw_rounded_rect(x, y, BUTTON_WIDTH, BUTTON_HEIGHT, 6, color)
        
        def _draw_button_border(self, x: int, y: int, intensity: float):
            """Draw button border."""
            if intensity > self.config.min_visible_intensity:
                alpha = 0.3 + intensity * 0.4
                r, g, b = int(60 * alpha), int(100 * alpha), int(120 * alpha)
                color = f'#{r:02x}{g:02x}{b:02x}'
            else:
                color = '#1e3246'
            
            self._draw_rounded_rect_outline(
                x, y, BUTTON_WIDTH, BUTTON_HEIGHT, 6, color
            )
        
        def _draw_button_label(self, cx: int, cy: int, label: str, intensity: float):
            """Draw button label."""
            if intensity > self.config.min_visible_intensity:
                r, g, b = self.color_mapper.intensity_to_rgb(
                    intensity, self.current_palette
                )
                color = f'#{r:02x}{g:02x}{b:02x}'
            else:
                color = '#506478'
            
            self.canvas.create_text(
                cx, cy,
                text=label,
                font=('Consolas', 18, 'bold'),
                fill=color
            )
        
        def _draw_rounded_rect(self, x: int, y: int, w: int, h: int, r: int, color: str):
            """Draw a filled rounded rectangle."""
            # Draw rectangle with rounded corners using polygon
            points = [
                x + r, y,
                x + w - r, y,
                x + w, y + r,
                x + w, y + h - r,
                x + w - r, y + h,
                x + r, y + h,
                x, y + h - r,
                x, y + r,
            ]
            self.canvas.create_polygon(points, fill=color, outline='', smooth=True)
        
        def _draw_rounded_rect_outline(self, x: int, y: int, w: int, h: int, r: int, color: str):
            """Draw a rounded rectangle outline."""
            # Draw corners
            self.canvas.create_arc(x, y, x + r * 2, y + r * 2, 
                                   start=90, extent=90, style='arc', outline=color)
            self.canvas.create_arc(x + w - r * 2, y, x + w, y + r * 2,
                                   start=0, extent=90, style='arc', outline=color)
            self.canvas.create_arc(x + w - r * 2, y + h - r * 2, x + w, y + h,
                                   start=270, extent=90, style='arc', outline=color)
            self.canvas.create_arc(x, y + h - r * 2, x + r * 2, y + h,
                                   start=180, extent=90, style='arc', outline=color)
            
            # Draw edges
            self.canvas.create_line(x + r, y, x + w - r, y, fill=color)
            self.canvas.create_line(x + w, y + r, x + w, y + h - r, fill=color)
            self.canvas.create_line(x + r, y + h, x + w - r, y + h, fill=color)
            self.canvas.create_line(x, y + r, x, y + h - r, fill=color)
    
    # Run the demo
    root = tk.Tk()
    app = ThermalPinpadDemo(root)
    root.mainloop()
