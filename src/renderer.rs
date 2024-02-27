use std::collections::HashMap;
use std::ffi::{c_void, CString};
use crate::color::Color;
use std::fs::read_to_string;

pub struct Renderer {
    shaders: HashMap<String, u32>,
    active_shader: u32,
    vbo: u32,
    vao: u32,
    vertices: Vec<f32>,
    projection_matrix: Matrix4,
}

impl Renderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW); // Allocate buffer, but don't fill it yet
            
            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 7 * std::mem::size_of::<f32>() as gl::types::GLsizei, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);
            // Color attribute
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 7 * std::mem::size_of::<f32>() as gl::types::GLsizei, (3 * std::mem::size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
        }

        let projection_matrix = Matrix4::orthographic(0.0, window_width, window_height, 0.0, -1.0, 1.0);
        
        Renderer {
            shaders: HashMap::new(),
            active_shader: 0, // Set using `use_shader`
            vbo,
            vao,
            vertices: Vec::new(),
            projection_matrix,
        }
    }

    pub fn draw_vertex(&mut self, position: (f32, f32), color: &Color) {
        self.vertices.extend_from_slice(&[
            position.0, position.1, 0.0, // X, Y, Z
            color.r, color.g, color.b, color.a, // R, G, B, A
        ]);
    }

    pub fn draw_triangle(&mut self, vertices: [(f32, f32); 3], color: &Color) {
        for &vertex in &vertices {
            self.draw_vertex(vertex, color);
        }
    }

    pub fn draw_triangle_colors(&mut self, vertices: [(f32, f32); 3], colors: [&Color; 3]) {
        let vertices_and_colors = vertices.iter().zip(colors.iter());

        for (&vertex, &color) in vertices_and_colors {
            self.draw_vertex(vertex, color);
        }
    }

    pub fn draw_quad(&mut self, position: (f32, f32), size: (f32, f32), color: &Color) {
        let (x, y) = position;
        let (width, height) = size;

        let top_left = (x, y);
        let top_right = (x + width, y);
        let bottom_left = (x, y + height);
        let bottom_right = (x + width, y + height);

        self.draw_triangle([top_left, bottom_left, top_right], color);
        self.draw_triangle([top_right, bottom_left, bottom_right], color);
    }

    pub fn draw_quad_colors(&mut self, position: (f32, f32), size: (f32, f32), colors: [&Color; 4]) {
        let (x, y) = position;
        let (width, height) = size;

        let top_left = (x, y);
        let top_right = (x + width, y);
        let bottom_left = (x, y + height);
        let bottom_right = (x + width, y + height);

        // First triangle (top left, bottom left, top right)
        self.draw_triangle_colors([top_left, bottom_left, top_right], [&colors[0], &colors[1], &colors[2]]);

        // Second triangle (top right, bottom left, bottom right)
        self.draw_triangle_colors([top_right, bottom_left, bottom_right], [&colors[2], &colors[1], &colors[3]]);
    }


    // SHADERS 
    pub fn init_shaders(&mut self) {
        let simple_vert = compile_shader("./src/shaders/simple.vert", gl::VERTEX_SHADER);
        let simple_frag = compile_shader("./src/shaders/simple.frag", gl::FRAGMENT_SHADER);
        let simple_shader = link_program(simple_vert, simple_frag);

        let gray_frag = compile_shader("./src/shaders/gray.frag", gl::FRAGMENT_SHADER);
        let gray_shader = link_program(simple_vert, gray_frag);
        
        self.shaders.insert("simple".to_string(), simple_shader);
        self.shaders.insert("gray".to_string(), gray_shader);
    }

    pub fn use_shader(&mut self, name: &str) {
        if let Some(&shader_program) = self.shaders.get(name) {
            self.active_shader = shader_program;
        } else {
            eprintln!("Shader named '{}' not found!", name);
        }
    }
    
    pub fn flush(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
                self.vertices.as_ptr() as *const c_void, 
                gl::DYNAMIC_DRAW);
            gl::UseProgram(self.active_shader);

            let loc = gl::GetUniformLocation(self.active_shader, CString::new("projectionMatrix").unwrap().as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, self.projection_matrix.elements.as_ptr());

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32 / 7); // 7 components per vertex

            self.vertices.clear();
        }
    }

    pub fn update_projection_matrix(&mut self, width: f32, height: f32) {
        self.projection_matrix = Matrix4::orthographic(0.0, width, height, 0.0, -1.0, 1.0);
    }
}


pub fn compile_shader(path: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader_src = read_to_string(path).expect("Failed to read shader file");
    let shader_c_str = CString::new(shader_src.as_bytes()).unwrap();
    let shader = unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &shader_c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let error = CString::new(vec![b' '; len as usize]).unwrap();
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            panic!("Failed to compile shader: {}", error.to_string_lossy());
        }

        shader
    };

    shader
}

pub fn link_program(vert_shader: gl::types::GLuint, frag_shader: gl::types::GLuint) -> gl::types::GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);
        gl::LinkProgram(program);

        let mut success: gl::types::GLint = 1;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let error = CString::new(vec![b' '; len as usize]).unwrap();
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            panic!("Failed to link program: {}", error.to_string_lossy());
        }

        gl::DeleteShader(vert_shader);
        gl::DeleteShader(frag_shader);

        program
    }
}


// LINEAR ALGEBRA
pub struct Matrix4 {
    elements: [f32; 16], // Column-major order to match OpenGL's expectation
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut result = Self::new();

        result.elements[0] = 2.0 / (right - left);
        result.elements[5] = 2.0 / (top - bottom);
        result.elements[10] = -2.0 / (far - near);
        result.elements[12] = -(right + left) / (right - left);
        result.elements[13] = -(top + bottom) / (top - bottom);
        result.elements[14] = -(far + near) / (far - near);

        result
    }
}

