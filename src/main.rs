extern crate glutin;
extern crate gl;

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use std::ffi::CString;

use std::fs;
use std::time::{Duration, Instant};

mod set_uniform;
mod opengl_handler;

use opengl_handler::OpenGLHandler;

fn main() {
    // Define the size of the viewport (width and height in pixels)
    let mut width = 1000;   
    let mut height = 700; 

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGL Window");
    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let opengl_handler = OpenGLHandler::new();
    opengl_handler.init_shaders();
    opengl_handler.init_buffers();

    //Set uinform values
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    (width, height) = new_size.into();
                    unsafe {
                        gl::Viewport(0, 0, width as i32, height as i32);
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let start = Instant::now();

                // Clear the color buffer
                unsafe { 
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::Clear(gl::DEPTH_BUFFER_BIT);
                    // Draw the triangle
                    gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_SHORT, std::ptr::null());
                }

                // Swap buffers if using double buffering
                context.swap_buffers().unwrap();

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();

                context.window().set_title(&format!("FPS: {}", fps.round()));
            }
            _ => (),
        }

    });    
}