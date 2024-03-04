//! Safe language bindings for raylib.
//! 
//! # Examples
//! Minimal window example:
//! ```
//! use raylib::prelude::*;
//! fn main() {
//!     let mut rl = Raylib::init_window(800, 800, "Rusty Raylib", 60);
//!     while !rl.window_should_close() {
//!         let mut draw = rl.begin_drawing();
//!         draw.clear_background(Color::RAYWHITE);
//!         # break
//!     }
//! }
//! ```

pub mod ffi;

pub mod core;
pub mod math;
pub mod shapes;
pub mod textures;
pub mod text;
pub mod model;

mod cstr_macro;

pub mod prelude;
