pub mod ffi;

mod core;
mod math;
mod shapes;

pub use ffi::{Color, Vector2, Vector3, Vector4, Matrix};

pub use core::{Raylib, window::ConfigFlags, draw::DrawHandle, shader::{Shader, Uniform, ShaderValue, ShaderDataType}};
