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
pub mod mr_robot;
pub mod sevastolink;
pub mod thermal;

// Re-export thermal types
pub use color::{ColorStop, ThermalColorMapper, ThermalPalette};
pub use thermal::{
    ring_intensity, ThermalButton, ThermalConfig, ThermalKeypad,
    BUTTON_LABELS as THERMAL_BUTTON_LABELS,
};

// Re-export mr_robot types
pub use mr_robot::{MrRobotKeypad, TerminalButton, TerminalConfig, MR_ROBOT_BUTTONS};

// Re-export sevastolink types
pub use sevastolink::{
    cursor_blink, lerp_color, AuthState, PressResult, SevastolinkConfig, SevastolinkKeypad,
    BUTTON_LABELS as SEVASTOLINK_BUTTON_LABELS,
};
