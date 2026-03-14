#!/usr/bin/env python3
"""
LVGL Demo for Thermal Pinpad - Embedded Linux (Raspberry Pi with touchscreen)

This demo uses LVGL (Light and Versatile Graphics Library) to render
the thermal pinpad effect on embedded Linux systems with touchscreens.

Target Hardware: Raspberry Pi with 7" touchscreen
Display: 800x480 (typical 7" touchscreen resolution)

For desktop development, install: pip install lvgl
For SDL2 driver support: pip install "lvgl[SDL]"
"""

import sys
import time
import math

# Import core thermal pinpad classes (platform-agnostic)
from thermal_pinpad import (
    ThermalKeypad,
    ThermalButton,
    ThermalConfig,
    ThermalPalette,
    ThermalColorMapper,
)

# LVGL import with graceful fallback
try:
    import lvgl as lv
except ImportError:
    print("ERROR: LVGL not installed. Run: pip install lvgl")
    print("For SDL2 driver: pip install \"lvgl[SDL]\"")
    sys.exit(1)

# =============================================================================
# Configuration
# =============================================================================

# Display settings for 7" touchscreen
DISPLAY_WIDTH = 800
DISPLAY_HEIGHT = 480

# Thermal colors (must match JavaScript/Rust exactly)
COLORS = {
    "background": 0x050F1E,       # Dark blue-black
    "cold": 0x002850,             # Dark blue
    "warm_start": 0x0096C8,       # Cyan
    "hot_mid": 0x00C864,          # Green
    "hot_high": 0xC8C800,         # Yellow
    "hottest": 0xFF6400,          # Orange
    "text_normal": 0x00D4FF,      # Cyan text
    "text_pressed": 0xFFFFFF,     # White text
}

# Button layout
BUTTON_SIZE = 70
BUTTON_SPACING = 15
BUTTON_START_X = 250
BUTTON_START_Y = 100

# =============================================================================
# LVGL Thermal Renderer
# =============================================================================

class LVGLThermalRenderer:
    """Renders thermal effects using LVGL canvas."""
    
    def __init__(self, canvas: lv.canvas, width: int, height: int):
        self.canvas = canvas
        self.width = width
        self.height = height
        self.color_mapper = ThermalColorMapper(ThermalPalette.SPLINTER_CELL)
        
        # Create canvas buffer (ARGB8888 format)
        self.buffer_size = width * height * 4
        self.buffer = bytearray(self.buffer_size)
        lv.canvas.set_buffer(canvas, self.buffer, width, height, lv.FORMAT.ARGB8888)
        
        # Initialize with background
        self._clear_canvas()
    
    def _clear_canvas(self):
        """Fill canvas with background color."""
        bg_color = lv.color_hex(COLORS["background"])
        lv.canvas.fill_bg(self.canvas, bg_color, lv.OPA.COVER)
    
    def draw_thermal_glow(self, x: int, y: int, radius: int, intensity: float, time_offset: float = 0):
        """Draw concentric glow rings for thermal effect."""
        if intensity < 0.02:
            return
        
        # Calculate color based on intensity
        r, g, b = self.color_mapper.map_to_rgb(intensity)
        color = lv.color_make(r, g, b)
        
        # Draw multiple rings for glow effect
        num_rings = 5
        for ring_idx in range(num_rings):
            ring_radius = radius + ring_idx * 3
            ring_intensity = intensity * (1 - ring_idx / num_rings) ** 2
            
            if ring_intensity < 0.02:
                continue
            
            # Adjust alpha based on ring position
            alpha = int(ring_intensity * 255)
            
            # Draw circle
            lv.canvas.draw_circle(
                self.canvas,
                x, y,
                ring_radius,
                color,
                alpha
            )
    
    def render_frame(self, keypad: ThermalKeypad, current_time: float):
        """Render a complete frame of the thermal pinpad."""
        self._clear_canvas()
        
        # Draw each button with thermal effect
        for button in keypad.buttons:
            # Calculate glow intensity based on time since last press
            if button.pressed_at > 0:
                elapsed = current_time - button.pressed_at
                decay_progress = elapsed / (keypad.config.decay_time_ms / 1000)
                intensity = math.exp(-decay_progress * 3)
            else:
                intensity = 0
            
            # Draw thermal glow
            self.draw_thermal_glow(
                button.x + BUTTON_SIZE // 2,
                button.y + BUTTON_SIZE // 2,
                BUTTON_SIZE // 2,
                intensity,
                current_time
            )


class LVGLThermalPinpad:
    """Main LVGL thermal pinpad application."""
    
    def __init__(self):
        # Initialize LVGL
        lv.init()
        
        # Create display driver (SDL for desktop, framebuffer for RPi)
        self._init_display()
        
        # Create main screen
        self.screen = lv.obj(lv.scr_act())
        self.screen.set_size(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        self.screen.center()
        
        # Set dark background
        self.screen.set_style_bg_color(lv.color_hex(COLORS["background"]), 0)
        self.screen.set_style_bg_opa(lv.OPA.COVER, 0)
        
        # Create title
        self._create_title()
        
        # Create canvas for thermal effects
        self.canvas = lv.canvas(self.screen)
        self.canvas.set_size(300, 300)
        self.canvas.align(lv.ALIGN.CENTER, 0, 30)
        
        # Initialize renderer
        self.renderer = LVGLThermalRenderer(self.canvas, 300, 300)
        
        # Create button grid
        self._create_buttons()
        
        # Create PIN display
        self._create_pin_display()
        
        # Initialize thermal keypad state
        self.config = ThermalConfig()
        self.keypad = ThermalKeypad(self.config)
        self._setup_button_positions()
        
        # State
        self.pin = ""
        self.demo_mode = False
        self.last_demo_time = 0
        
        # Start animation timer
        self._start_animation_timer()
    
    def _init_display(self):
        """Initialize display driver."""
        # For desktop development with SDL2
        try:
            # Try SDL driver first
            import SDL
            disp_drv = lv.sdl_window_create(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        except ImportError:
            # Fallback: Create basic display
            print("Note: SDL driver not available")
            print("For touchscreen on RPi, configure framebuffer driver")
    
    def _create_title(self):
        """Create title label."""
        title = lv.label(self.screen)
        title.set_text("THERMAL PINPAD")
        title.set_style_text_color(lv.color_hex(COLORS["text_normal"]), 0)
        title.set_style_text_font(lv.font_montserrat_24, 0)
        title.align(lv.ALIGN.TOP_MID, 0, 20)
        
        subtitle = lv.label(self.screen)
        subtitle.set_text("Splinter Cell Thermal Vision")
        subtitle.set_style_text_color(lv.color_hex(0x006080), 0)
        subtitle.align(lv.ALIGN.TOP_MID, 0, 50)
    
    def _create_buttons(self):
        """Create button grid."""
        self.buttons = {}
        labels = [
            ["1", "2", "3"],
            ["4", "5", "6"],
            ["7", "8", "9"],
            ["CLR", "0", "ENT"]
        ]
        
        for row_idx, row in enumerate(labels):
            for col_idx, label in enumerate(row):
                btn = lv.btn(self.screen)
                btn.set_size(BUTTON_SIZE, BUTTON_SIZE)
                
                x = BUTTON_START_X + col_idx * (BUTTON_SIZE + BUTTON_SPACING)
                y = BUTTON_START_Y + row_idx * (BUTTON_SIZE + BUTTON_SPACING)
                btn.set_pos(x, y)
                
                # Style
                btn.set_style_bg_color(lv.color_hex(COLORS["cold"]), 0)
                btn.set_style_bg_opa(lv.OPA.COVER, 0)
                btn.set_style_radius(10, 0)
                btn.set_style_border_width(2, 0)
                btn.set_style_border_color(lv.color_hex(COLORS["warm_start"]), 0)
                
                # Label
                btn_label = lv.label(btn)
                btn_label.set_text(label)
                btn_label.set_style_text_color(lv.color_hex(COLORS["text_normal"]), 0)
                btn_label.center()
                
                # Event handler
                btn.add_event_cb(
                    lambda e, l=label: self._on_button_click(l),
                    lv.EVENT.CLICKED,
                    None
                )
                
                self.buttons[label] = btn
    
    def _create_pin_display(self):
        """Create PIN entry display."""
        self.pin_label = lv.label(self.screen)
        self.pin_label.set_text("____")
        self.pin_label.set_style_text_color(lv.color_hex(COLORS["text_normal"]), 0)
        self.pin_label.set_style_text_font(lv.font_montserrat_28, 0)
        self.pin_label.align(lv.ALIGN.CENTER, 0, -80)
    
    def _setup_button_positions(self):
        """Setup button positions in thermal keypad."""
        for label, btn in self.buttons.items():
            x = BUTTON_START_X + list(self.buttons.keys()).index(label) % 3 * (BUTTON_SIZE + BUTTON_SPACING)
            y = BUTTON_START_Y + list(self.buttons.keys()).index(label) // 3 * (BUTTON_SIZE + BUTTON_SPACING)
            # Store position in thermal button
            thermal_btn = self.keypad.get_button(label)
            if thermal_btn:
                thermal_btn.x = x
                thermal_btn.y = y
    
    def _on_button_click(self, label: str):
        """Handle button click."""
        current_time = time.time()
        
        # Update thermal state
        thermal_btn = self.keypad.get_button(label)
        if thermal_btn:
            thermal_btn.press(current_time * 1000)
        
        # Handle PIN entry
        if label == "CLR":
            self.pin = ""
        elif label == "ENT":
            self._verify_pin()
            self.pin = ""
        elif len(self.pin) < 4:
            self.pin += label
        
        # Update display
        self.pin_label.set_text(self.pin.ljust(4, "_"))
        
        # Visual feedback
        btn = self.buttons.get(label)
        if btn:
            btn.set_style_bg_color(lv.color_hex(COLORS["hottest"]), 0)
            # Reset after delay (would use timer in production)
    
    def _verify_pin(self):
        """Verify entered PIN."""
        # Placeholder - in production, check against actual code
        if self.pin == "1234":
            self.pin_label.set_text("ACCESS")
            self.pin_label.set_style_text_color(lv.color_hex(COLORS["hot_mid"]), 0)
        else:
            self.pin_label.set_text("DENIED")
            self.pin_label.set_style_text_color(lv.color_hex(COLORS["hottest"]), 0)
    
    def _start_animation_timer(self):
        """Start animation update timer."""
        self.timer = lv.timer_create(
            self._update_animation,
            33,  # ~30 FPS
            None
        )
    
    def _update_animation(self, timer):
        """Update animation frame."""
        current_time = time.time()
        
        # Render thermal effects
        self.renderer.render_frame(self.keypad, current_time)
        
        # Update button colors based on thermal state
        for label, btn in self.buttons.items():
            thermal_btn = self.keypad.get_button(label)
            if thermal_btn and thermal_btn.pressed_at > 0:
                elapsed = current_time - thermal_btn.pressed_at / 1000
                decay_progress = elapsed / (self.config.decay_time_ms / 1000)
                intensity = math.exp(-decay_progress * 3)
                
                r, g, b = self.color_mapper.map_to_rgb(intensity)
                btn.set_style_bg_color(lv.color_make(r, g, b), 0)
        
        # Demo mode
        if self.demo_mode and current_time - self.last_demo_time > 8:
            self._demo_press()
            self.last_demo_time = current_time
    
    def _demo_press(self):
        """Press random button for demo."""
        import random
        labels = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"]
        label = random.choice(labels)
        self._on_button_click(label)
    
    def run(self):
        """Run the application."""
        # For SDL driver, this handles the event loop
        # For RPi framebuffer, you may need custom loop
        while True:
            lv.task_handler()
            time.sleep(0.005)


# =============================================================================
# Entry Point
# =============================================================================

if __name__ == "__main__":
    print("Thermal Pinpad - LVGL Demo")
    print("Target: Raspberry Pi with 7\" touchscreen")
    print()
    print("Controls:")
    print("  Touch buttons to enter PIN")
    print("  CLR - Clear PIN")
    print("  ENT - Submit PIN")
    print()
    
    try:
        app = LVGLThermalPinpad()
        app.run()
    except KeyboardInterrupt:
        print("\nExiting...")
    except Exception as e:
        print(f"Error: {e}")
        print()
        print("Make sure LVGL is installed:")
        print("  pip install lvgl")
        print("  pip install \"lvgl[SDL]\"  # For desktop development")
