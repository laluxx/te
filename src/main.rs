extern crate gl;
extern crate glfw;

mod color;
use crate::color::Color;
mod renderer;
use crate::renderer::Renderer;
mod font;
use crate::font::FontAtlas;

use glfw::{Action, Context, Key};
use std::fs::read_to_string;
use std::ffi::CString;

fn main() {
    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, events) = glfw.create_window(800, 600, "Rust Terminal", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::None);

    window.set_framebuffer_size_callback(|_, width, height| {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    });

    // Shader compilation and program linking
    let simple_vert = compile_shader("./src/shaders/simple.vert", gl::VERTEX_SHADER);
    let simple_frag = compile_shader("./src/shaders/simple.frag", gl::FRAGMENT_SHADER);
    let simple_shader = link_program(simple_vert, simple_frag);

    let mut renderer = Renderer::new(simple_shader, 800.0, 600.0);
    
    let background_color = Color::from_hex("#090909");
    let quad_color = Color::from_hex("#514B8E");

    while !window.should_close() {
        glfw.poll_events();
        unsafe {
            gl::ClearColor(background_color.r, background_color.g, background_color.b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let (width_i32, height_i32) = window.get_framebuffer_size();
        let width = width_i32 as f32;
        let height = height_i32 as f32;

        renderer.update_projection_matrix(width, height);

        let quad_position = (0.0, height - 21.0);
        
        renderer.draw_quad(quad_position, (width, 21.0), &quad_color);

        renderer.draw_triangle_colors(
            [(100.0, 100.0), (400.0, 100.0), (250.0, 300.0)],
            [&Color::from_hex("#A3212C"), &Color::from_hex("#933484"), &Color::from_hex("#00A67D")]
        );

        renderer.flush();
        
        renderer.draw_quad_colors(
            ((width - 100.0) / 2.0, (height - 100.0) / 2.0),
            (100.0, 100.0),
            [
                &Color::from_hex("#00A67D"), // Top left
                &Color::from_hex("#933484"), // Bottom left
                &Color::from_hex("#982320"), // Top right
                &Color::from_hex("#FFAA00")  // Bottom right
            ]
        );

        renderer.flush();

        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {}
            }
        }
    }
}


fn compile_shader(path: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
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

fn link_program(vert_shader: gl::types::GLuint, frag_shader: gl::types::GLuint) -> gl::types::GLuint {
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



