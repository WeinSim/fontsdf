const QUAD_VERTICES: [f32; 12] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    // 
    0.0, 1.0,
    1.0, 0.0,
    1.0, 1.0,
];

pub struct Vao {
    vao_id: u32,
    vertex_count: usize,
    vbos: Vec<Vbo>,
}

impl Vao {
    fn new(vertex_count: usize) -> Self {
        let mut vao_id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
        }
        Self {
            vao_id,
            vertex_count,
            vbos: Vec::new(),
        }
    }

    pub fn create_quad() -> Result<Self, String> {
        let mut vao = Self::new(6);
        vao.add_float_vbo(2, &QUAD_VERTICES)?;
        vao.unbind();
        Ok(vao)
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn add_float_vbo(&mut self, coordinate_size: i32, data: &[f32]) -> Result<(), String> {
        let expected_len = coordinate_size as usize * self.vertex_count;
        let actual_len = data.len();
        if expected_len != actual_len {
            return Err(format!(
                "mismatched slice len: expected: {expected_len}, got: {actual_len}"
            ));
        }
        self.vbos.push(Vbo::new(
            self.vbos.len() as u32,
            coordinate_size,
            &QUAD_VERTICES,
        ));
        Ok(())
    }

    pub fn render(&self) {
        self.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count as i32);
        }
        self.unbind();
    }
}

pub struct Vbo {
    vbo_id: u32,
}

impl Vbo {
    fn new(attribute_number: u32, coordinate_size: i32, data: &[f32]) -> Self {
        let mut vbo_id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
        }
        let vbo = Self { vbo_id };
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                attribute_number,
                coordinate_size,
                gl::FLOAT,
                gl::FALSE,
                coordinate_size * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(attribute_number);
        }
        vbo.unbind();
        vbo
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
