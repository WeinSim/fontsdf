use std::ffi::{CStr, CString};

pub struct ShaderProgram {
    program_id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Self, String> {
        let vertex_shader = Shader::compile(vertex_source, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::compile(fragment_source, gl::FRAGMENT_SHADER)?;
        let program_id = unsafe { gl::CreateProgram() };
        let shader = Self { program_id };
        unsafe {
            gl::AttachShader(program_id, vertex_shader.shader_id);
            gl::AttachShader(program_id, fragment_shader.shader_id);
            gl::LinkProgram(program_id);
        }
        let mut success = 0;
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success) };
        if success == gl::FALSE as i32 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = vec![0u8; len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
            }
            return Err(unsafe {
                CStr::from_ptr(buffer.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned()
            });
        }
        Ok(shader)
    }

    pub fn start(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn stop(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

struct Shader {
    shader_id: u32,
}

impl Shader {
    fn compile(source: &str, shader_type: u32) -> Result<Self, String> {
        let source =
            CString::new(source).map_err(|_| "Shader source contains a null byte".to_string())?;
        let shader_id = unsafe { gl::CreateShader(shader_type) };
        let shader = Self { shader_id };
        unsafe {
            gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }
        let mut success = 0;
        unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success) };
        if success == gl::FALSE as i32 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = vec![0u8; len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
            }
            return Err(unsafe {
                CStr::from_ptr(buffer.as_ptr() as *const i8)
                    .to_string_lossy()
                    .into_owned()
            });
        }
        Ok(shader)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}
