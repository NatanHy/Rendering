extern crate glutin;
extern crate gl;

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use triangles::{Triangle, TriangleMesh};
use std::time::{Duration, Instant};

mod set_uniform;
mod opengl_handler;
mod triangles;

use opengl_handler::OpenGLHandler;

fn init_triangles() -> TriangleMesh {
    let tri = Triangle::new(
        [0., 1., -0.3],
        [1., 0., 0.],

        [-1., -1., -0.2],
        [0., 1., 0.],

        [1., -1., -0.5],
        [0., 0., 1.],
    );

    TriangleMesh::new(vec![tri])
}

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

    let mut opengl_handler = OpenGLHandler::new();
    opengl_handler.init_shaders();
    opengl_handler.init_buffers(Some(&init_triangles()));

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

                opengl_handler.draw();
                context.swap_buffers().unwrap();

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();

                context.window().set_title(&format!("FPS: {}", fps.round()));
            }
            _ => (),
        }

    });    
}
