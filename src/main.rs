mod render_gl;

extern crate sdl2;
extern crate gl;

use crate::render_gl::shader;
use shader::{VERTEX_SHADER, FRAGMENT_SHADER};


fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Rust OpenGl Playground", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let vertex_shader = shader::new(VERTEX_SHADER)
        .from_string("
            #version 450 core

            layout (location = 0) in vec3 Position;

            void main() {
                gl_Position = vec4(Position, 1.0);
            }
         ")
         .build()
         .unwrap();

    let fragment_shader = shader::new(FRAGMENT_SHADER)
        .from_string("
            #version 450 core

            out vec4 Color;

            void main()
            {
                Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        ")
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

        // render window contents here
    }

    println!("Window closed.")
}
