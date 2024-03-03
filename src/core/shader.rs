use std::{ffi::{CString, CStr, c_void}, ptr::null};
use crate::{ffi::{self, ShaderUniformDataType, Matrix}, prelude::{Vector2, Vector3, Vector4, Texture}};

use super::Raylib;

/// # Shader creation functions (module: [rcore])
/// 
/// ---
impl Raylib {
    /// Reads code for a shader from the given files, and loads them into raylib.
    /// The default shader is used for `None`s.
    /// Returns an error if there is a problem with reading the given files.
    /// 
    /// # Panics
    /// Panics if the given files contain null bytes in them.
    /// 
    /// # Example
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(500, 500, "shader test", 60);
    /// let shader = rl.load_shader(None, Some("assets/mandelbrot.glsl")).unwrap();
    /// let resolution_uniform = shader.get_uniform("resolution");
    /// let camera_pos_uniform = shader.get_uniform("camera_pos");
    /// let camera_pos = Vector2::ZERO;
    /// while !rl.window_should_close() {
    ///     let mut draw = rl.begin_drawing();
    ///     shader.set_uniform_value(resolution_uniform, rl.get_screen_size());
    ///     shader.set_uniform_value(camera_pos_uniform, camera_pos);
    ///     let mut draw = draw.begin_shader_mode(&shader);
    ///     unsafe { raylib::ffi::DrawRectangle(0, 0, rl.get_screen_width() as i32, rl.get_screen_height() as i32, Color::WHITE) }
    ///     # break
    /// }
    /// ```
    pub fn load_shader<P: AsRef<std::path::Path>>(&self, vs_file_name: Option<P>, fs_file_name: Option<P>) -> std::io::Result<Shader> {
        let vs = vs_file_name.map(std::fs::read).transpose()?;
        let fs = fs_file_name.map(std::fs::read).transpose()?;

        let vs = vs.map(CString::new).transpose().expect("a vertex shader file without nulls");
        let fs = fs.map(CString::new).transpose().expect("a fragment shader file without nulls");

        Ok(self.load_shader_from_memory(vs.as_deref(), fs.as_deref()))
    }

    /// Loads the code for a shader from the given CStrings
    /// The default shader is used for `None`s.
    pub fn load_shader_from_memory(&self, vs_code: Option<&CStr>, fs_code: Option<&CStr>) -> Shader {
        let shader = unsafe {
            ffi::LoadShaderFromMemory(
                vs_code.map_or(null(), |s| s.as_ptr()),
                fs_code.map_or(null(), |s| s.as_ptr())
            )
        };

        Shader(shader)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uniform(i32);

#[derive(Debug, Clone)]
pub struct Shader(ffi::Shader);

impl Shader {
    pub fn is_ready(&self) -> bool {
        unsafe { ffi::IsShaderReady(self.0) }
    }

    /// Gets the location of a shader uniform
    /// # Panics
    /// Panics if the given string contains null bytes
    pub fn get_uniform(&self, name: &str) -> Uniform {
        self.get_uniform_cstr(CString::new(name).expect("a string without null bytes").as_c_str())
    }

    /// Gets the location of a shader attribute
    /// # Panics
    /// Panics if the given string contains null bytes
    pub fn get_attribute(&self, name: &str) -> Uniform {
        self.get_attribute_cstr(CString::new(name).expect("a string without null bytes").as_c_str())
    }

    /// Gets the location of a shader uniform
    pub fn get_uniform_cstr(&self, name: &CStr) -> Uniform {
        Uniform( unsafe { ffi::GetShaderLocation(self.0, name.as_ptr()) } )
    }

    /// Gets the location of a shader attribute
    pub fn get_attribute_cstr(&self, name: &CStr) -> Uniform {
        Uniform( unsafe { ffi::GetShaderLocationAttrib(self.0, name.as_ptr()) } )
    }

    /// Returns the inner C raylib shader object
    pub unsafe fn get_ffi_shader(&self) -> ffi::Shader {
        self.0
    }

    /// Sets a single shader value without checking for type.
    /// If you know the type of your data, prefer using `Shader::set_value`.
    ///
    /// SAFETY: Causes undefined behaviour if the value pointed to by `value` does not posses the exact size described by `ty`.
    pub unsafe fn set_value_unchecked(&self, uniform: Uniform, value: *const c_void, ty: ShaderUniformDataType) {
        unsafe { ffi::SetShaderValue(self.0, uniform.0, value, ty as i32) }
    }

    /// Sets multiple values without checking for type.
    /// If you know the type of your data, prefer using `Shader::set_value` with a slice.
    ///
    /// SAFETY: Causes undefined behaviour if the array `value` does not posses the exact size described by `ty` times `count`.
    pub unsafe fn set_value_unchecked_v(&self, uniform: Uniform, value: *const c_void, ty: ShaderUniformDataType, count: i32) {
        unsafe { ffi::SetShaderValueV(self.0, uniform.0, value, ty as i32, count) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { ffi::UnloadShader(self.0) }
    }
}

pub unsafe trait ShaderDataType<T> {
    const DATA_TYPE: ShaderUniformDataType;
}

pub unsafe trait ShaderValue<T> {
    fn set_uniform_value(&self, uniform: Uniform, v: T);
}

// Floating point uniforms
unsafe impl ShaderDataType<f32> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Float;
}

unsafe impl ShaderDataType<[f32; 2]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec2;
}

unsafe impl ShaderDataType<[f32; 3]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec3;
}

unsafe impl ShaderDataType<[f32; 4]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec4;
}

unsafe impl ShaderDataType<Vector2> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec2;
}

unsafe impl ShaderDataType<Vector3> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec3;
}

unsafe impl ShaderDataType<Vector4> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Vec4;
}

// Integer uniforms
unsafe impl ShaderDataType<i32> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Int;
}

unsafe impl ShaderDataType<[i32; 2]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Ivec2;
}

unsafe impl ShaderDataType<[i32; 3]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Ivec3;
}

unsafe impl ShaderDataType<[i32; 4]> for Shader {
    const DATA_TYPE: ShaderUniformDataType = ShaderUniformDataType::Ivec4;
}

// Generic impls and other types
unsafe impl<T> ShaderValue<T> for Shader where Shader: ShaderDataType<T> {
    fn set_uniform_value(&self, uniform: Uniform, v: T) {
        unsafe {
            self.set_value_unchecked(uniform, &v as *const T as *const c_void, <Self as ShaderDataType<T>>::DATA_TYPE)
        }
    }
}

unsafe impl<T> ShaderValue<&[T]> for Shader where Shader: ShaderDataType<T> {
    fn set_uniform_value(&self, uniform: Uniform, v: &[T]) {
        unsafe {
            self.set_value_unchecked_v(uniform, v.as_ptr() as *const c_void, <Self as ShaderDataType<T>>::DATA_TYPE, v.len() as i32) 
        }
    }
}

unsafe impl ShaderValue<Matrix> for Shader {
    fn set_uniform_value(&self, uniform: Uniform, v: Matrix) {
        unsafe { ffi::SetShaderValueMatrix(self.0, uniform.0, v) }
    }
}

unsafe impl ShaderValue<Texture> for Shader {
    fn set_uniform_value(&self, uniform: Uniform, v: Texture) {
        unsafe { ffi::SetShaderValueTexture(self.0, uniform.0, v.get_ffi_texture()) }
    }
}
