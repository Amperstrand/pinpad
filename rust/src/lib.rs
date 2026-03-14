//! # Pinpad Library
//!
//! A `no_std` compatible library implementing various movie/game pinpad effects
//! for embedded-graphics.
//!
//! ## Modules
//!
//! - **thermal**: Splinter Cell (2002) thermal vision keypad effect
//! - **mr_robot**: Mr. Robot (TV series) terminal pinpad
//! - **sevastolink**: Alien: Isolation (2014) Sevastolink terminal pinpad
//!
//! ## Features
//!
//! - Thermal button state management with exponential decay
//! - Sevastolink terminal with CRT effects and authentication
//! - Mr. Robot fsociety terminal aesthetic
//! - Multiple color palettes
//! - `no_std` compatible for embedded targets
//! - Optional PC simulator with `simulator` feature
//!
//! ## Example - Thermal
//!
//! ```rust
//! use thermal_pinpad::{ThermalKeypad, ThermalConfig, ThermalPalette, ThermalColorMapper};
//!
//! // Create a keypad with default config
//! let mut keypad = ThermalKeypad::new();
//!
//! // Press some buttons (timestamps in milliseconds)
//! keypad.press('1', 0);
//! keypad.press('2', 500);
//! keypad.press('3', 1000);
//! keypad.press('4', 1500);
//!
//! // Check intensities after 2 seconds
//! let intensities = keypad.intensities(2000);
//! for (label, intensity) in intensities {
//!     if intensity > 0.02 {
//!         println!("Button {} has intensity {:.2}", label, intensity);
//!     }
//! }
//!
//! // Map intensity to color
//! let mapper = ThermalColorMapper::with_palette(ThermalPalette::SplinterCell);
//! let color = mapper.intensity_to_rgb(0.8);
//! ```
//!
//! ## Example - Sevastolink
//!
//! ```rust
//! use thermal_pinpad::{SevastolinkKeypad, SevastolinkConfig, AuthState, PressResult};
//!
//! // Create a Sevastolink terminal
//! let mut keypad = SevastolinkKeypad::new();
//!
//! // Enter a code
//! keypad.press('1', 0);
//! keypad.press('2', 100);
//! keypad.press('3', 200);
//! keypad.press('4', 300);
//!
//! // Submit for verification
//! keypad.press('E', 400);
//! assert_eq!(keypad.auth_state(), AuthState::Verifying);
//!
//! // Complete verification
//! keypad.verify_complete(true, 1500);
//! assert_eq!(keypad.auth_state(), AuthState::Success);
//! ```

#![no_std]

pub mod color;
pub mod deadspace;
pub mod deusex;
pub mod mr_robot;
pub mod nostromo;
pub mod sevastolink;
pub mod thermal;
pub mod tron;
pub mod wargames;

// Re-export thermal types
pub use color::{ColorStop, ThermalColorMapper, ThermalPalette};
pub use thermal::{
    ring_intensity, ThermalButton, ThermalConfig, ThermalKeypad,
    BUTTON_LABELS as THERMAL_BUTTON_LABELS,
};

pub use tron::{
    button_bevel_points, button_index, flicker_opacity, AuthState as TronAuthState,
    CircuitTraceSegment as TronCircuitTraceSegment, PressResult as TronPressResult, TronConfig,
    TronFrame, TronKeypad, BUTTON_LABELS as TRON_BUTTON_LABELS,
    CIRCUIT_TRACES as TRON_CIRCUIT_TRACES, DEEP_BLACK as TRON_DEEP_BLACK,
    GRID_CYAN as TRON_GRID_CYAN, NEON_BLUE as TRON_NEON_BLUE, NEON_ORANGE as TRON_NEON_ORANGE,
    NEON_WHITE as TRON_NEON_WHITE,
};

pub use deadspace::colors as deadspace_colors;
pub use deadspace::DeadSpaceConfig;

pub use nostromo::{
    NostromoButton, NostromoConfig, NostromoKeypad, NostromoStatus,
    BUTTON_BLUE as NOSTROMO_BUTTON_BLUE, BUTTON_RED as NOSTROMO_BUTTON_RED,
    INDICATOR_AMBER as NOSTROMO_INDICATOR_AMBER, INDICATOR_GREEN as NOSTROMO_INDICATOR_GREEN,
    NOSTROMO_BUTTON_LABELS, PANEL_BACKGROUND as NOSTROMO_PANEL_BACKGROUND,
    TEXT_COLOR as NOSTROMO_TEXT_COLOR, WEAR_COLOR as NOSTROMO_WEAR_COLOR,
};

pub use deusex::{
    AnimationState as DeusExAnimationState, AuthState as DeusExAuthState, DeusExConfig,
    DeusExKeypad, PressResult as DeusExPressResult, ALERT_RED as DEUSEX_ALERT_RED,
    AMBER as DEUSEX_AMBER, BACKGROUND as DEUSEX_BACKGROUND, BUTTON_LABELS as DEUSEX_BUTTON_LABELS,
    CYAN as DEUSEX_CYAN, DARK_GOLD as DEUSEX_DARK_GOLD, PRIMARY_GOLD as DEUSEX_PRIMARY_GOLD,
};

// Re-export mr_robot types
pub use mr_robot::{MrRobotKeypad, TerminalButton, TerminalConfig, MR_ROBOT_BUTTONS};

// Re-export sevastolink types
pub use sevastolink::{
    cursor_blink, lerp_color, AuthState, PressResult, SevastolinkConfig, SevastolinkKeypad,
    BUTTON_LABELS as SEVASTOLINK_BUTTON_LABELS,
};

pub use wargames::{
    CursorState, LineStyle, TerminalBuffer, TerminalLine, WargamesConfig, WargamesTerminal,
    BUFFER_CAPACITY as WARGAMES_BUFFER_CAPACITY, BUFFER_LINE_CAP as WARGAMES_BUFFER_LINE_CAP,
    INPUT_CAPACITY as WARGAMES_INPUT_CAPACITY, MENU_LINES as WARGAMES_MENU_LINES,
};
