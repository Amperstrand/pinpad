"""
Sevastolink Terminal Pinpad - Alien: Isolation (2014) Style

A Python implementation of the Sevastolink terminal interface from Alien: Isolation.
Features CRT effects, scan lines, and the iconic green-on-black color scheme.

Usage:
    # As a library
    from sevastolink_pinpad import SevastolinkKeypad, SevastolinkColors
    
    keypad = SevastolinkKeypad()
    keypad.press_button('1')
    keypad.enter_code()  # Submit
    
    # Run demo
    python sevastolink_pinpad.py
"""

from dataclasses import dataclass
from enum import Enum, auto
from typing import Literal, Optional, Tuple
import random
import time


# =============================================================================
# COLOR DEFINITIONS (Cross-Platform Consistent)
# =============================================================================

class SevastolinkColors:
    """Exact color values matching the JavaScript implementation."""
    
    # Primary colors (RGB tuples)
    XENOMORPH_SKIN = (12, 41, 12)       # #0c290c - Background dark green
    TERMINAL_GREEN = (19, 66, 19)       # #134213 - Secondary background
    SEEGSON_GREEN = (5, 182, 105)       # #05b669 - Primary text/highlights
    ACID_BLOOD = (240, 120, 38)         # #f07826 - Warnings/errors
    HYPERSLEEP_WHITE = (204, 213, 212)  # #ccd5d4 - Bright text
    SYNTHETIC_SKIN = (122, 128, 127)    # #7a807f - Muted text
    PURE_BLACK = (0, 0, 0)              # #000000 - CRT background
    DARK_BG = (10, 10, 10)              # #0a0a0a - Screen border

    @classmethod
    def to_hex(cls, color: Tuple[int, int, int]) -> str:
        """Convert RGB tuple to hex string."""
        return f'#{color[0]:02x}{color[1]:02x}{color[2]:02x}'


# =============================================================================
# CORE CLASSES (Platform-Agnostic)
# =============================================================================

@dataclass
class SevastolinkConfig:
    """Configuration for Sevastolink terminal parameters.
    
    Attributes:
        max_code_length: Maximum digits in access code (default: 8)
        cursor_blink_ms: Cursor blink interval (default: 530)
        keypress_flash_ms: Button flash duration (default: 150)
        error_flash_ms: Error display duration (default: 250)
        success_flash_ms: Success display duration (default: 400)
        verify_delay_ms: Authentication delay (default: 800)
    """
    max_code_length: int = 8
    cursor_blink_ms: int = 530
    keypress_flash_ms: int = 150
    error_flash_ms: int = 250
    success_flash_ms: int = 400
    verify_delay_ms: int = 800
    scanline_spacing_px: int = 2
    flicker_min_ms: int = 2000
    flicker_max_ms: int = 4000
    flicker_duration_ms: int = 100
    auth_code: str = '1234'


class AuthState(Enum):
    """Authentication state machine states."""
    IDLE = auto()
    VERIFYING = auto()
    SUCCESS = auto()
    DENIED = auto()


class SevastolinkKeypad:
    """Manages the Sevastolink terminal keypad state.
    
    Tracks entered code, authentication state, and button press timing.
    """
    
    BUTTONS = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'C', '0', 'E']
    
    def __init__(self, config: Optional[SevastolinkConfig] = None):
        """Initialize the keypad.
        
        Args:
            config: Terminal configuration (uses defaults if None)
        """
        self.config = config or SevastolinkConfig()
        self._code: str = ''
        self._auth_state = AuthState.IDLE
        self._flashing_button: Optional[str] = None
        self._flash_start_time: float = 0
        self._state_change_time: float = 0
    
    @property
    def code(self) -> str:
        """Current entered code."""
        return self._code
    
    @property
    def code_masked(self) -> str:
        """Code masked with block characters."""
        return '\u2588' * len(self._code)
    
    @property
    def auth_state(self) -> AuthState:
        """Current authentication state."""
        return self._auth_state
    
    @property
    def flashing_button(self) -> Optional[str]:
        """Currently flashing button (if any)."""
        return self._flashing_button
    
    def press_button(self, label: str) -> bool:
        """Process a button press.
        
        Args:
            label: The button label pressed
            
        Returns:
            True if press was processed, False if ignored
        """
        if self._auth_state == AuthState.VERIFYING:
            return False
        
        # Record flash
        self._flashing_button = label
        self._flash_start_time = time.time() * 1000
        
        if label == 'C':
            # Clear last digit
            self._code = self._code[:-1]
            return True
        elif label == 'E':
            # Submit code
            self._submit_code()
            return True
        elif label in '0123456789':
            if len(self._code) < self.config.max_code_length:
                self._code += label
                return True
        
        return False
    
    def _submit_code(self) -> None:
        """Submit the current code for verification."""
        if len(self._code) == 0:
            self._auth_state = AuthState.DENIED
            self._state_change_time = time.time() * 1000
            return
        
        self._auth_state = AuthState.VERIFYING
        self._state_change_time = time.time() * 1000
    
    def verify_complete(self, success: bool) -> None:
        """Complete verification with result.
        
        Args:
            success: Whether authentication succeeded
        """
        if success:
            self._auth_state = AuthState.SUCCESS
        else:
            self._auth_state = AuthState.DENIED
        self._state_change_time = time.time() * 1000
    
    def reset_auth_state(self) -> None:
        """Reset authentication state to idle."""
        self._auth_state = AuthState.IDLE
        self._code = ''
    
    def clear_code(self) -> None:
        """Clear the entered code."""
        self._code = ''
    
    def get_flash_intensity(self, now_ms: Optional[float] = None) -> Tuple[Optional[str], float]:
        """Get current flash state and intensity.
        
        Args:
            now_ms: Current time (uses time.time() if None)
            
        Returns:
            Tuple of (button_label, intensity 0.0-1.0)
        """
        if now_ms is None:
            now_ms = time.time() * 1000
        
        if self._flashing_button is None:
            return (None, 0.0)
        
        elapsed = now_ms - self._flash_start_time
        if elapsed >= self.config.keypress_flash_ms:
            self._flashing_button = None
            return (None, 0.0)
        
        intensity = 1.0 - (elapsed / self.config.keypress_flash_ms)
        return (self._flashing_button, max(0.0, intensity))
    
    def get_state_duration(self, now_ms: Optional[float] = None) -> float:
        """Get how long we've been in current auth state.
        
        Args:
            now_ms: Current time (uses time.time() if None)
            
        Returns:
            Duration in milliseconds
        """
        if now_ms is None:
            now_ms = time.time() * 1000
        return now_ms - self._state_change_time


def lerp_color(
    color1: Tuple[int, int, int], 
    color2: Tuple[int, int, int], 
    t: float
) -> Tuple[int, int, int]:
    """Linearly interpolate between two colors.
    
    Args:
        color1: Starting color (RGB)
        color2: Ending color (RGB)
        t: Interpolation factor (0.0-1.0)
        
    Returns:
        Interpolated color (RGB)
    """
    t = max(0.0, min(1.0, t))
    return (
        round(color1[0] + (color2[0] - color1[0]) * t),
        round(color1[1] + (color2[1] - color1[1]) * t),
        round(color1[2] + (color2[2] - color1[2]) * t)
    )


# =============================================================================
# TKINTER DEMO
# =============================================================================

if __name__ == "__main__":
    import tkinter as tk
    from tkinter import ttk
    
    # Demo configuration
    CANVAS_WIDTH = 320
    CANVAS_HEIGHT = 420
    BUTTON_WIDTH = 80
    BUTTON_HEIGHT = 50
    BUTTON_GAP = 12
    MARGIN_X = 25
    MARGIN_Y = 140
    BUTTONS_PER_ROW = 3
    TARGET_FPS = 60
    
    # CRT effect parameters
    NOISE_INTENSITY = 0.08
    GRAIN_UPDATE_INTERVAL_MS = 100  # ~10fps for film grain

    class SevastolinkDemo:
        """Tkinter demo application for the Sevastolink terminal."""
        
        def __init__(self, root: tk.Tk):
            self.root = root
            self.root.title("Sevastolink Terminal - Alien: Isolation")
            self.root.configure(bg='#0a0a0a')
            self.root.resizable(False, False)
            
            # Initialize components
            self.config = SevastolinkConfig()
            self.keypad = SevastolinkKeypad(self.config)
            
            # Animation state
            self.cursor_visible = True
            self.last_cursor_toggle = time.time() * 1000
            self.screen_flicker = False
            self.flicker_end_time = 0
            self.next_flicker = time.time() * 1000 + random.randint(
                self.config.flicker_min_ms,
                self.config.flicker_max_ms
            )
            
            # Demo mode
            self.demo_mode = False
            self.demo_job = None
            self.verify_job = None

            # Cached grain for performance (regenerated at ~10fps)
            self.cached_grain_points = []
            self.last_grain_update = 0

            # Build UI
            self._build_ui()
            
            # Start animation
            self._animate()
        
        def _build_ui(self):
            """Build the user interface."""
            # Main container
            container = tk.Frame(self.root, bg='#0a0a0a', padx=10, pady=10)
            container.pack(padx=10, pady=10)
            
            # Header
            header = tk.Frame(container, bg='#0a0a0a')
            header.pack(pady=(0, 5))
            
            title = tk.Label(
                header,
                text="SEEGSON SEVASTOLINK",
                font=('Courier New', 11),
                fg='#05b669',
                bg='#0a0a0a'
            )
            title.pack()
            
            subtitle = tk.Label(
                header,
                text="Sevastopol Station - Terminal Access",
                font=('Courier New', 9),
                fg='#134213',
                bg='#0a0a0a'
            )
            subtitle.pack()
            
            # CRT container frame
            crt_frame = tk.Frame(
                container, 
                bg='#1a1a1a', 
                padx=8, 
                pady=8,
                highlightbackground='#1a1a1a',
                highlightthickness=8
            )
            crt_frame.pack()
            
            # Canvas for terminal
            self.canvas = tk.Canvas(
                crt_frame,
                width=CANVAS_WIDTH,
                height=CANVAS_HEIGHT,
                bg='#000000',
                highlightthickness=0
            )
            self.canvas.pack()
            self.canvas.bind('<Button-1>', self._on_canvas_click)
            
            # Keyboard bindings
            self.root.bind('<Key>', self._on_key_press)
            
            # Status bar
            status_frame = tk.Frame(container, bg='#0a0a0a')
            status_frame.pack(fill='x', pady=(15, 0))
            
            self.status_var = tk.StringVar(value="STATUS: AWAITING INPUT")
            status_label = tk.Label(
                status_frame,
                textvariable=self.status_var,
                font=('Courier New', 9),
                fg='#05b669',
                bg='#0c290c',
                padx=10,
                pady=5
            )
            status_label.pack(fill='x')
            
            # Controls
            controls = tk.Frame(container, bg='#0a0a0a')
            controls.pack(fill='x', pady=(15, 0))
            
            btn_frame = tk.Frame(controls, bg='#0a0a0a')
            btn_frame.pack()
            
            # Demo button
            self.demo_btn = tk.Button(
                btn_frame,
                text="Demo",
                font=('Courier New', 9),
                fg='#05b669',
                bg='#134213',
                activebackground='#05b669',
                activeforeground='#0c290c',
                relief='flat',
                padx=12,
                pady=5,
                command=self._toggle_demo_mode
            )
            self.demo_btn.pack(side='left', padx=5)
            
            # Reset button
            reset_btn = tk.Button(
                btn_frame,
                text="Reset",
                font=('Courier New', 9),
                fg='#05b669',
                bg='#134213',
                activebackground='#05b669',
                activeforeground='#0c290c',
                relief='flat',
                padx=12,
                pady=5,
                command=self._reset_terminal
            )
            reset_btn.pack(side='left', padx=5)
            
            # Clear button
            clear_btn = tk.Button(
                btn_frame,
                text="Clear",
                font=('Courier New', 9),
                fg='#05b669',
                bg='#134213',
                activebackground='#05b669',
                activeforeground='#0c290c',
                relief='flat',
                padx=12,
                pady=5,
                command=self._clear_code
            )
            clear_btn.pack(side='left', padx=5)
            
            # Instructions
            instructions = tk.Label(
                container,
                text="Enter access code. Press ENTER to submit.",
                font=('Courier New', 8),
                fg='#7a807f',
                bg='#0a0a0a'
            )
            instructions.pack(pady=(10, 0))
        
        def _get_button_position(self, index: int) -> Tuple[int, int]:
            """Get the top-left position of a button by index."""
            row = index // BUTTONS_PER_ROW
            col = index % BUTTONS_PER_ROW
            x = MARGIN_X + col * (BUTTON_WIDTH + BUTTON_GAP)
            y = MARGIN_Y + row * (BUTTON_HEIGHT + BUTTON_GAP)
            return (x, y)
        
        def _get_button_at_position(self, x: int, y: int) -> Optional[str]:
            """Find which button is at the given canvas coordinates."""
            for i, label in enumerate(SevastolinkKeypad.BUTTONS):
                bx, by = self._get_button_position(i)
                if bx <= x <= bx + BUTTON_WIDTH and by <= y <= by + BUTTON_HEIGHT:
                    return label
            return None
        
        def _on_canvas_click(self, event):
            """Handle canvas click."""
            label = self._get_button_at_position(event.x, event.y)
            if label:
                self._press_button(label)
        
        def _on_key_press(self, event):
            """Handle keyboard input."""
            key = event.keysym.upper()
            
            if key.isdigit():
                self._press_button(key)
            elif key == 'C' or key == 'BACKSPACE':
                self._press_button('C')
            elif key == 'RETURN':
                self._press_button('E')
        
        def _press_button(self, label: str):
            """Process a button press."""
            if self.keypad.press_button(label):
                if label == 'E':
                    self._update_status('VERIFYING...')
                    # Schedule verification
                    if self.verify_job:
                        self.root.after_cancel(self.verify_job)
                    self.verify_job = self.root.after(
                        self.config.verify_delay_ms,
                        self._complete_verification
                    )
                elif label == 'C':
                    self._update_status('INPUT CLEARED')
                else:
                    self._update_status('INPUT RECEIVED')
        
        def _complete_verification(self):
            """Complete the verification process."""
            if len(self.keypad.code) == 0:
                self.keypad.verify_complete(False)
                self._update_status('ERROR: NO CODE ENTERED')
                self._schedule_reset(self.config.error_flash_ms)
                return
            
            success = self.keypad.code == self.config.auth_code
            self.keypad.verify_complete(success)
            
            if success:
                self._update_status('ACCESS GRANTED')
                self._schedule_reset(self.config.success_flash_ms)
            else:
                self._update_status('ACCESS DENIED')
                self._schedule_reset(self.config.error_flash_ms)
        
        def _schedule_reset(self, delay_ms: int):
            """Schedule terminal reset after delay."""
            def reset():
                self.keypad.reset_auth_state()
                self._update_status('AWAITING INPUT')
            
            self.root.after(delay_ms, reset)
        
        def _toggle_demo_mode(self):
            """Toggle demo mode."""
            self.demo_mode = not self.demo_mode
            
            if self.demo_mode:
                self.demo_btn.configure(text="Stop", bg='#05b669', fg='#0c290c')
                self._run_demo_sequence()
            else:
                self.demo_btn.configure(text="Demo", bg='#134213', fg='#05b669')
                if self.demo_job:
                    self.root.after_cancel(self.demo_job)
                    self.demo_job = None
        
        def _run_demo_sequence(self):
            """Run the demo sequence."""
            if not self.demo_mode:
                return
            
            self._enter_random_code()
            self.demo_job = self.root.after(5000, self._run_demo_sequence)
        
        def _enter_random_code(self):
            """Enter a random code."""
            if self.keypad.auth_state != AuthState.IDLE:
                return
            
            length = random.randint(4, 7)
            code = ''.join(random.choices('0123456789', k=length))
            
            for i, digit in enumerate(code):
                self.root.after(i * 200, lambda d=digit: self._press_button(d))
            
            # Submit after entering
            self.root.after(length * 200 + 300, lambda: self._press_button('E'))
        
        def _reset_terminal(self):
            """Reset the terminal."""
            self.keypad.reset_auth_state()
            self._update_status('TERMINAL RESET')
        
        def _clear_code(self):
            """Clear the entered code."""
            self.keypad.clear_code()
            self._update_status('CODE CLEARED')
        
        def _update_status(self, text: str):
            """Update status bar."""
            self.status_var.set(f'STATUS: {text}')
        
        def _animate(self):
            """Animation loop."""
            self._update_state()
            self._render()
            self.root.after(1000 // TARGET_FPS, self._animate)
        
        def _update_state(self):
            """Update animation state."""
            now_ms = time.time() * 1000
            
            # Update cursor blink
            if now_ms - self.last_cursor_toggle >= self.config.cursor_blink_ms:
                self.cursor_visible = not self.cursor_visible
                self.last_cursor_toggle = now_ms
            
            # Update screen flicker
            if now_ms >= self.next_flicker:
                self.screen_flicker = True
                self.flicker_end_time = now_ms + self.config.flicker_duration_ms
                self.next_flicker = now_ms + random.randint(
                    self.config.flicker_min_ms,
                    self.config.flicker_max_ms
                )
            
            if self.screen_flicker and now_ms >= self.flicker_end_time:
                self.screen_flicker = False
        
        def _render(self):
            """Render the terminal."""
            self.canvas.delete('all')
            
            # Draw background with noise
            self._draw_background()
            
            # Draw header
            self._draw_header()
            
            # Draw display area
            self._draw_display()
            
            # Draw keypad
            self._draw_keypad()
            
            # Draw footer
            self._draw_footer()
            
            # Draw CRT effects
            self._draw_crt_effects()
        
        def _draw_background(self):
            """Draw background with noise."""
            self.canvas.create_rectangle(
                0, 0, CANVAS_WIDTH, CANVAS_HEIGHT,
                fill='#000000', outline=''
            )

            # Regenerate grain at ~10fps
            now_ms = time.time() * 1000
            if not self.cached_grain_points or now_ms - self.last_grain_update >= GRAIN_UPDATE_INTERVAL_MS:
                self.cached_grain_points = []
                for _ in range(int(CANVAS_WIDTH * CANVAS_HEIGHT * NOISE_INTENSITY * 0.01)):
                    x = random.randint(0, CANVAS_WIDTH)
                    y = random.randint(0, CANVAS_HEIGHT)
                    noise = random.randint(5, 15)
                    color = f'#{noise:02x}{noise*2:02x}{noise:02x}'
                    self.cached_grain_points.append((x, y, color))
                self.last_grain_update = now_ms

            # Draw cached grain
            for x, y, color in self.cached_grain_points:
                self.canvas.create_rectangle(x, y, x+1, y+1, fill=color, outline='')

            # Screen flicker overlay
            if self.screen_flicker:
                self.canvas.create_rectangle(
                    0, 0, CANVAS_WIDTH, CANVAS_HEIGHT,
                    fill='#05b669', stipple='gray12'
                )
        
        def _draw_header(self):
            """Draw terminal header."""
            # Title
            self._draw_glow_text(
                CANVAS_WIDTH // 2, 25,
                text="SEEGSON SEVASTOLINK",
                font=('Courier New', 12),
                fill='#05b669',
                glow='#05b669',
                glow_offset=1
            )
            
            # Separator
            self.canvas.create_line(
                30, 40, CANVAS_WIDTH - 30, 40,
                fill='#134213', width=1
            )
            
            # Station info
            self.canvas.create_text(
                CANVAS_WIDTH // 2, 60,
                text="SEVASTOPOL STATION - 2137",
                font=('Courier New', 10),
                fill='#7a807f'
            )
        
        def _draw_display(self):
            """Draw code display area."""
            display_y = 80
            display_height = 45
            
            # Background
            self.canvas.create_rectangle(
                25, display_y,
                CANVAS_WIDTH - 25, display_y + display_height,
                fill='#0c290c', outline='#134213', width=1
            )
            
            # Label
            self.canvas.create_text(
                35, display_y + 15,
                text="ACCESS CODE:",
                font=('Courier New', 10),
                fill='#05b669',
                anchor='w'
            )
            
            # Code display
            code_text = self.keypad.code_masked
            if self.cursor_visible and self.keypad.auth_state == AuthState.IDLE:
                code_text += '\u2588'
            
            self._draw_glow_text(
                35, display_y + 35,
                text=code_text,
                font=('Courier New', 18),
                fill='#ccd5d4',
                anchor='w',
                glow='#05b669',
                glow_offset=1
            )
            
            # Auth state overlay
            if self.keypad.auth_state == AuthState.SUCCESS:
                self.canvas.create_rectangle(
                    25, display_y,
                    CANVAS_WIDTH - 25, display_y + display_height,
                    fill='#05b669', stipple='gray50'
                )
            elif self.keypad.auth_state == AuthState.DENIED:
                self.canvas.create_rectangle(
                    25, display_y,
                    CANVAS_WIDTH - 25, display_y + display_height,
                    fill='#f07826', stipple='gray50'
                )
        
        def _draw_keypad(self):
            """Draw keypad buttons."""
            flashing_btn, flash_intensity = self.keypad.get_flash_intensity()
            
            for i, label in enumerate(SevastolinkKeypad.BUTTONS):
                self._draw_button(i, label, flashing_btn, flash_intensity)
        
        def _draw_button(self, index: int, label: str, flashing_btn: Optional[str], flash_intensity: float):
            """Draw a single button."""
            x, y = self._get_button_position(index)
            is_flashing = flashing_btn == label

            # Chromatic aberration on hot edges (when flashing intensely)
            apply_chromatic = is_flashing and flash_intensity > 0.4
            chroma_offset = 2

            if apply_chromatic:
                # Red channel offset (left)
                self._draw_rounded_rect(
                    x - chroma_offset, y, BUTTON_WIDTH, BUTTON_HEIGHT, 3,
                    fill='', outline='#ff0000'
                )
                # Blue channel offset (right)
                self._draw_rounded_rect(
                    x + chroma_offset, y, BUTTON_WIDTH, BUTTON_HEIGHT, 3,
                    fill='', outline='#0000ff'
                )

            # Determine colors
            if is_flashing and flash_intensity > 0:
                bg_color = lerp_color(
                    SevastolinkColors.XENOMORPH_SKIN,
                    SevastolinkColors.SEEGSON_GREEN,
                    flash_intensity * 0.5
                )
                border_color = SevastolinkColors.HYPERSLEEP_WHITE
                text_color = SevastolinkColors.HYPERSLEEP_WHITE
            else:
                bg_color = SevastolinkColors.XENOMORPH_SKIN
                border_color = SevastolinkColors.TERMINAL_GREEN
                text_color = SevastolinkColors.SEEGSON_GREEN

            # Draw button background
            self._draw_rounded_rect(
                x, y, BUTTON_WIDTH, BUTTON_HEIGHT, 3,
                fill=SevastolinkColors.to_hex(bg_color),
                outline=SevastolinkColors.to_hex(border_color)
            )

            # Button label
            display_label = label
            if label == 'C':
                display_label = 'CLR'
            elif label == 'E':
                display_label = 'ENT'

            cx = x + BUTTON_WIDTH // 2
            cy = y + BUTTON_HEIGHT // 2

            # Chromatic aberration on label when flashing hard
            if apply_chromatic:
                self.canvas.create_text(
                    cx - chroma_offset, cy,
                    text=display_label,
                    font=('Courier New', 16, 'bold'),
                    fill='#ff0044'
                )
                self.canvas.create_text(
                    cx + chroma_offset, cy,
                    text=display_label,
                    font=('Courier New', 16, 'bold'),
                    fill='#0044ff'
                )

            self.canvas.create_text(
                cx, cy,
                text=display_label,
                font=('Courier New', 16, 'bold'),
                fill=SevastolinkColors.to_hex(text_color)
            )
        
        def _draw_rounded_rect(self, x: int, y: int, w: int, h: int, r: int, fill: str, outline: str):
            """Draw a rounded rectangle."""
            # Simple rectangle with small corners
            self.canvas.create_rectangle(
                x + r, y, x + w - r, y + h,
                fill=fill, outline=''
            )
            self.canvas.create_rectangle(
                x, y + r, x + w, y + h - r,
                fill=fill, outline=''
            )
            
            # Outline
            self.canvas.create_rectangle(
                x, y, x + w, y + h,
                fill='', outline=outline, width=1
            )
        
        def _draw_footer(self):
            """Draw footer."""
            footer_y = CANVAS_HEIGHT - 25
            
            # Separator
            self.canvas.create_line(
                30, footer_y - 10,
                CANVAS_WIDTH - 30, footer_y - 10,
                fill='#134213', width=1
            )
            
            # Status text
            if self.keypad.auth_state == AuthState.VERIFYING:
                text = 'VERIFYING...'
                color = '#05b669'
            elif self.keypad.auth_state == AuthState.SUCCESS:
                text = 'ACCESS GRANTED'
                color = '#05b669'
            elif self.keypad.auth_state == AuthState.DENIED:
                text = 'ACCESS DENIED'
                color = '#f07826'
            else:
                text = 'READY'
                color = '#7a807f'
            
            self.canvas.create_text(
                CANVAS_WIDTH // 2, footer_y,
                text=text,
                font=('Courier New', 9),
                fill=color
            )

        def _draw_glow_text(
            self,
            x: int,
            y: int,
            text: str,
            font: Tuple[str, int],
            fill: str,
            anchor: Literal['nw', 'n', 'ne', 'w', 'center', 'e', 'sw', 's', 'se'] = 'center',
            glow: str = '#05b669',
            glow_offset: int = 1,
        ):
            for dx, dy in [(-glow_offset, 0), (glow_offset, 0), (0, -glow_offset), (0, glow_offset)]:
                self.canvas.create_text(
                    x + dx,
                    y + dy,
                    text=text,
                    font=font,
                    fill=glow,
                    anchor=anchor
                )
            self.canvas.create_text(x, y, text=text, font=font, fill=fill, anchor=anchor)
        
        def _draw_crt_effects(self):
            """Draw CRT scan lines and vignette."""
            # Scan lines
            for y in range(0, CANVAS_HEIGHT, self.config.scanline_spacing_px):
                self.canvas.create_line(
                    0, y, CANVAS_WIDTH, y,
                    fill='#000000', width=1, stipple='gray50'
                )

            for inset, stipple in [(0, 'gray50'), (8, 'gray25'), (16, 'gray12')]:
                self.canvas.create_rectangle(
                    inset,
                    inset,
                    CANVAS_WIDTH - inset,
                    CANVAS_HEIGHT - inset,
                    outline='#000000',
                    width=3,
                    stipple=stipple
                )
    
    # Run the demo
    root = tk.Tk()
    app = SevastolinkDemo(root)
    root.mainloop()
