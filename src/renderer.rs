use std::ffi::{c_void, CString};
use crate::color::Color;

pub struct Renderer {
    shader_program: u32,
    vbo: u32,
    vao: u32,
    vertices: Vec<f32>,
    projection_matrix: Matrix4,
}

impl Renderer {
    pub fn new(shader_program: u32, window_width: f32, window_height: f32) -> Self {
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
            shader_program,
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

    pub fn flush(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
                self.vertices.as_ptr() as *const c_void, 
                gl::DYNAMIC_DRAW);

            gl::UseProgram(self.shader_program);

            let loc = gl::GetUniformLocation(self.shader_program, CString::new("projectionMatrix").unwrap().as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, self.projection_matrix.elements.as_ptr());

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32 / 7); // 7 components per vertex

            self.vertices.clear(); // Clear vertices after flushing
        }
    }

    pub fn update_projection_matrix(&mut self, width: f32, height: f32) {
        self.projection_matrix = Matrix4::orthographic(0.0, width, height, 0.0, -1.0, 1.0);
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

