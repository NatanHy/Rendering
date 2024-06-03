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
mod obj_parser;

use obj_parser::obj_to_mesh;
use opengl_handler::OpenGLHandler;

fn init_triangles() -> TriangleMesh {
    let verts = vec![
        -0.2, 0.2, 0.2,
         1.0, 0.0, 0.0,

         0.2, 0.2, 0.2,
        0.0, 1.0, 0.0,

        -0.2, -0.2, 0.2,
        1.0, 1.0, 0.0,

         0.2, -0.2, 0.2,
        0.0, 0.0, 1.0,

        -0.2, 0.2, -0.2,
         1.0, 0.0, 1.0,

         0.2, 0.2, -0.2,
        0.0, 1.0, 1.0,

        -0.2, -0.2, -0.2,
        1.0, 1.0, 1.0,

         0.2, -0.2, -0.2,
        0.0, 0.0, 1.0,
    ];

    let indicies = vec![
        2, 1, 0,
        2, 3, 1,
        0, 5, 4,
        0, 1, 5,
        3, 5, 1,
        3, 7, 5,
        0, 4, 2,
        2, 4, 6,
        2, 6, 3,
        3, 6, 7,
        4, 5, 6,
        7, 6, 5,
    ];

    TriangleMesh::from_array_indicies(verts, indicies)
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
    opengl_handler.init_buffers(Some(&obj_to_mesh("objects/fox_full.obj")));

    let mut t : f32 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

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

                opengl_handler.draw(t);
                context.swap_buffers().unwrap();

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();

                t += 1.0 / fps as f32;

                context.window().set_title(&format!("FPS: {}", fps.round()));
            }
            _ => (),
        }

    });    
}
