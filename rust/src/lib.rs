//! # Thermal Pinpad
//!
//! A `no_std` compatible library implementing the Splinter Cell (2002) thermal
//! vision keypad effect for embedded-graphics.
//!
//! ## Features
//!
//! - Thermal button state management with exponential decay
//! - Multiple color palettes matching the game's thermal vision
//! - `no_std` compatible for embedded targets
//! - Optional PC simulator with `simulator` feature
//!
//! ## Example
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

#![no_std]

pub mod color;
pub mod thermal;

// Re-export main types for convenience
pub use color::{ColorStop, ThermalColorMapper, ThermalPalette};
pub use thermal::{ring_intensity, ThermalButton, ThermalConfig, ThermalKeypad, BUTTON_LABELS};
