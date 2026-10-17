use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    str::FromStr,
};

use strum_macros::{Display, EnumString};

pub struct ShaderProgram {
    program_id: u32,
    uniforms: HashMap<String, UniformVariable>,
}

impl ShaderProgram {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Self, String> {
        let mut uniforms = HashMap::new();
        let (vertex, vertex_uniforms) = Shader::compile(vertex_source, gl::VERTEX_SHADER)?;
        uniforms.extend(vertex_uniforms);
        let (fragment, fragment_uniforms) = Shader::compile(fragment_source, gl::FRAGMENT_SHADER)?;
        uniforms.extend(fragment_uniforms);
        let program_id = unsafe { gl::CreateProgram() };
        let mut shader = Self {
            program_id,
            uniforms,
        };
        unsafe {
            gl::AttachShader(program_id, vertex.shader_id);
            gl::AttachShader(program_id, fragment.shader_id);
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
        let mut attribute_number: u32 = 0;
        for vbo in &vertex.vbos {
            unsafe {
                gl::BindAttribLocation(
                    shader.program_id,
                    attribute_number,
                    vbo.attribute_name.as_ptr() as *const i8,
                );
            }
            attribute_number += vbo.get_num_attributes() as u32;
        }
        for (name, uniform) in &mut shader.uniforms {
            uniform.location = Some(unsafe {
                gl::GetUniformLocation(shader.program_id, name.as_ptr() as *const i8)
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

    pub fn load_uniform_1i(&self, name: &str, value: i32) -> Result<(), String> {
        self.load_uniform(name, value, Datatype::Int, gl::Uniform1i)
    }

    pub fn load_uniform_1f(&self, name: &str, value: f32) -> Result<(), String> {
        self.load_uniform(name, value, Datatype::Float, gl::Uniform1f)
    }

    pub fn load_uniform_2f(&self, name: &str, value: (f32, f32)) -> Result<(), String> {
        self.load_uniform(name, value, Datatype::Vec2, |l, v| unsafe {
            gl::Uniform2f(l, v.0, v.1)
        })
    }

    pub fn load_uniform_3f(&self, name: &str, value: (f32, f32, f32)) -> Result<(), String> {
        self.load_uniform(name, value, Datatype::Vec3, |l, v| unsafe {
            gl::Uniform3f(l, v.0, v.1, v.2)
        })
    }

    // TODO continue: implement other load_uniform_xxx functions or think of a better interface than
    // what I am building here

    fn load_uniform<T>(
        &self,
        name: &str,
        value: T,
        expected_datatype: Datatype,
        load_func: unsafe fn(i32, T) -> (),
    ) -> Result<(), String> {
        let uniform = self
            .uniforms
            .get(name)
            .ok_or_else(|| format!("uniform variable {name} doesn't exist"))?;
        if uniform.datatype != expected_datatype {
            return Err(format!(
                "invalid uniform datatype: expected {}, got {}",
                expected_datatype, uniform.datatype
            ));
        }
        let location = uniform
            .location
            .expect("uniform location should be set in ShaderProgram::new");
        unsafe { load_func(location, value) };
        Ok(())
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
    vbos: Vec<Vbo>,
}

impl Shader {
    fn compile(
        source: &str,
        shader_type: u32,
    ) -> Result<(Self, HashMap<String, UniformVariable>), String> {
        let (uniforms, vbos) = Self::parse_shader_source(source)?;
        let source =
            CString::new(source).map_err(|_| "Shader source contains a null byte".to_string())?;
        let shader_id = unsafe { gl::CreateShader(shader_type) };
        let shader = Self { shader_id, vbos };
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
        Ok((shader, uniforms))
    }

    fn parse_shader_source(
        source: &str,
    ) -> Result<(HashMap<String, UniformVariable>, Vec<Vbo>), String> {
        let mut uniform_variables: HashMap<String, UniformVariable> = HashMap::new();
        let mut attribute_number: usize = 0;
        let mut vbos: Vec<Vbo> = Vec::new();
        for line in source.lines() {
            // ignore comments
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            // attributes
            let parts: Vec<_> = line.split(" ").collect();
            if let Some(in_index) = line.split(" ").position(|p| *p == *"in") {
                let attribute_name = parts.get(in_index + 2).unwrap();
                let semicolon_index = attribute_name.char_indices().find(|(_, c)| *c == ';');
                let attribute_name = match semicolon_index {
                    Some((i, _)) => &attribute_name[..i],
                    None => attribute_name,
                }
                .to_owned();
                let datatype = Datatype::from_str(parts.get(in_index + 1).unwrap())
                    .map_err(|_| format!("Invalid attribute datatype: {}", parts[1]))?;
                let vbo = Vbo {
                    attribute_name,
                    attribute_number,
                    datatype,
                };
                attribute_number += vbo.get_num_attributes();
                vbos.push(vbo);
            }
            // uniform variables
            if line.contains("uniform") {
                let parts: Vec<_> = line.split(" ").collect();
                let datatype = Datatype::from_str(parts[1])
                    .map_err(|_| format!("Invalid datatype: \"{}\"!", parts[1]))?;
                let name = parts[2].replace(";", "");
                if let Some(open_index) = name.find('[') {
                    let close_index = name.find(']').unwrap();
                    let base_name = &name[..open_index];
                    let len = &name[open_index + 1..close_index];
                    let len: usize = len
                        .parse()
                        .map_err(|_| format!("invalid array length: {len}"))?;
                    for i in 0..len {
                        let name = format!("{base_name}[{i}]");
                        // not sure if cloning is neccessary here
                        uniform_variables
                            .insert(name.clone(), UniformVariable::new(name, datatype));
                    }
                } else {
                    uniform_variables.insert(name.clone(), UniformVariable::new(name, datatype));
                }
            }
        }
        Ok((uniform_variables, vbos))
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumString, Display)]
enum Datatype {
    #[strum(to_string = "int")]
    Int,
    #[strum(to_string = "float")]
    Float,
    #[strum(to_string = "vec2")]
    Vec2,
    #[strum(to_string = "vec3")]
    Vec3,
    #[strum(to_string = "vec4")]
    Vec4,
    #[strum(to_string = "mat2")]
    Mat2,
    #[strum(to_string = "mat3")]
    Mat3,
    #[strum(to_string = "mat4")]
    Mat4,
}

// impl Datatype {
//     fn coordinate_size(self) -> i32 {
//         match self {
//             Self::Int => 1,
//             Self::Float => 1,
//             Self::Vec2 => 2,
//             Self::Vec3 => 3,
//             Self::Vec4 => 4,
//             Self::Mat2 => 2,
//             Self::Mat3 => 3,
//             Self::Mat4 => 4,
//         }
//     }
// }

struct Vbo {
    attribute_name: String,
    attribute_number: usize,
    datatype: Datatype,
}

impl Vbo {
    fn get_num_attributes(&self) -> usize {
        match self.datatype {
            Datatype::Mat2 => 2,
            Datatype::Mat3 => 3,
            Datatype::Mat4 => 4,
            _ => 1,
        }
    }
}

struct UniformVariable {
    name: String,
    datatype: Datatype,
    location: Option<i32>,
}

impl UniformVariable {
    fn new(name: String, datatype: Datatype) -> Self {
        Self {
            name,
            datatype,
            location: None,
        }
    }
}
