extern crate gl;
extern crate glfw;

mod color;
use crate::color::Color;
mod renderer;
use crate::renderer::Renderer;
mod font;
use crate::font::FontAtlas;

use glfw::{Action, Context, Key};

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

    let mut renderer = Renderer::new(800.0, 600.0);
    renderer.init_shaders();
    
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

        renderer.use_shader("simple");

        renderer.draw_triangle_colors(
            [(100.0, 100.0), (400.0, 100.0), (250.0, 300.0)],
            [&Color::from_hex("#A3212C"), &Color::from_hex("#933484"), &Color::from_hex("#00A67D")],
            [(100.0, 100.0), (400.0, 100.0), (250.0, 300.0)],
        );

        // renderer.flush();

        // renderer.use_shader("gray");

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





