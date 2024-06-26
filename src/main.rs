extern crate glutin;
extern crate gl;

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use moving::center_obj_fn;
use std::time::Instant;

mod set_uniform;
mod opengl_handler;
mod triangles;
mod obj_parser;
mod texture;
mod moving;

use obj_parser::{obj_to_mesh, FaceLayout};
use opengl_handler::{CameraHandler, OpenGLHandler};

fn main() {
    // Define the size of the viewport (width and height in pixels)
    let mut width = 1000;   
    let mut height = 1000; 

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGL Window");
    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let face_layout = FaceLayout::new(Some(0), Some(1), None);

    let obj = "objects/Scaniverse.obj";
    let tex = "textures/Scaniverse.jpg";

    let triangles = obj_to_mesh(obj, &face_layout);
    let tex_path = Some(tex);

    let mut opengl_handler = OpenGLHandler::new();
    opengl_handler.init_shaders();
    opengl_handler.init_buffers(Some(&triangles));
    opengl_handler.init_textures(tex_path);

    let movement_fn = center_obj_fn(obj, 0., 0.5, -2.2);
    
    let fov = 3.1415 / 3.;
    let (n, f) = (0.1, 10.);
    
    opengl_handler.camera_handler = CameraHandler::perspective(fov, width as f32 / height as f32, n, f);
    movement_fn(&mut opengl_handler.camera_handler);

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

                        opengl_handler.camera_handler = CameraHandler::perspective(fov, width as f32 / height as f32, n, f);
                        movement_fn(&mut opengl_handler.camera_handler);                    
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let start = Instant::now();

                opengl_handler.draw();
                opengl_handler.camera_handler.rotate(-t, [0., 1., 0.]);
                context.swap_buffers().unwrap();

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();

                t = 1.0 / fps as f32;

                context.window().set_title(&format!("FPS: {}", fps.round()));
            }
            _ => (),
        }

    });    
}
