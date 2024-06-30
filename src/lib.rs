//! Safe language bindings for raylib.
//! 
//! # Examples
//! Minimal hello world example:
//! ```
//! use raylib::prelude::*;
//! fn main() {
//!     let rl = &mut init_window(800, 800, "Rusty Raylib", 60);
//!     while !window_should_close(rl) {
//!         begin_drawing(rl, |rl| {
//!             clear_background(rl, Color::RAYWHITE);
//!             rl.text(rl.default_font(), "Hello, world!", vec2(20.0, 20.0), 20.0, Color::BLACK);
//!         });
//!         # break
//!     }
//! }
//! ```

pub mod ffi;

pub mod core;
pub mod math;
pub mod shapes;
pub mod collisions;
pub mod textures;
pub mod text;
pub mod model;

mod cstr_macro;

pub mod prelude;
