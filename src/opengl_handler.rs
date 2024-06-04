use std::{ffi::CString, fs};
use crate::triangles::TriangleMesh;
use glm::{self, Vector3};
use crate::set_uniform::{set_uniform, UniformType};
use crate::texture::Texture;

struct GlBuffer {
    id : u32,
    target : gl::types::GLenum,
    offset : usize,
}

impl GlBuffer {
    fn new(buffer_id : u32, target : gl::types::GLenum, offset : usize) -> Self {
        let mut id = buffer_id;

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(target, id);
        }

        GlBuffer { id, target, offset }
    }

    fn set_data<T>(&mut self, data : &Vec<T>, usage : gl::types::GLenum) {
        unsafe {
            //TODO : clear buffer data
            gl::BufferData(
                self.target,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const gl::types::GLvoid,
                usage
            );
        }

        self.offset = data.len() * std::mem::size_of::<T>();
    }
}

pub struct CameraHandler {
    transform_mat : glm::Matrix4<f32>
}

impl CameraHandler {
    pub fn new() -> Self {
        let identity_mat : glm::Matrix4<f32> = glm::mat4(
            1., 0., 0., 0., 
            0., 1., 0., 0., 
            0., 0., 1., 0., 
            0., 0., 0., 1.);

        CameraHandler { transform_mat : identity_mat }
    }

    pub fn perspective(fov_rad : f32, aspect : f32, near : f32, far : f32) -> Self {
        let mut transform_mat = glm::ext::perspective(fov_rad, aspect, near, far);
    
        CameraHandler { transform_mat }
    }

    pub fn translate(&mut self, x : f32, y : f32, z : f32) {
        self.transform_mat = glm::ext::translate(&self.transform_mat, Vector3::new(x, y, z));
    }

    pub fn scale(&mut self, x : f32,y : f32, z : f32) {
        self.transform_mat = glm::ext::scale(&self.transform_mat, Vector3::new(x, y, z));
    }

    pub fn rotate(&mut self, angle : f32, axis : [f32;3]) {
        self.transform_mat = glm::ext::rotate(&self.transform_mat, angle, Vector3::new(axis[0], axis[1], axis[2]));
    }
    
}

pub struct OpenGLHandler {
    shader_program : u32,
    vbo : Option<GlBuffer>,
    ebo : Option<GlBuffer>,
    num_triangles : u32,
    pub camera_handler : CameraHandler
}

impl OpenGLHandler {
    pub fn new() -> Self {
        OpenGLHandler {  
            shader_program : 0,
            vbo : None,
            ebo : None,
            num_triangles : 0,
            camera_handler : CameraHandler::new()
        }
    }

    fn load_shader(&self, source_path: &str, shader_type: u32) -> u32 {
        let source = fs::read_to_string(source_path).expect("Failed to read shader file");
        let shader = unsafe {gl::CreateShader(shader_type)};
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }
        
        let mut success = gl::FALSE as i32;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }
        if success != gl::TRUE as i32 {
            let mut log_length = 0;
            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
                let log = vec![0u8; log_length as usize];
                gl::GetShaderInfoLog(shader, log_length, std::ptr::null_mut(), log.as_ptr() as *mut i8);
                let log_string = String::from_utf8_lossy(&log);
                panic!("Failed to compile shader: {}", log_string);
            }
        } else {
            println!("Shader compiled successfully!")
        }
    
        shader
    }
    
    pub fn init_shaders(&mut self) {
        // Load and compile shaders
        let vertex_shader = self.load_shader("shaders/vertex.glsl", gl::VERTEX_SHADER);
        let fragment_shader = self.load_shader("shaders/fragment.glsl", gl::FRAGMENT_SHADER);
    
        // Create shader program
        let shader_program = unsafe { gl::CreateProgram() };
    
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            gl::UseProgram(shader_program);
        }

        self.shader_program = shader_program;
    }
    
    pub fn init_buffers(&mut self, triangle_mesh : Option<&TriangleMesh>) {
        let mut vbo = GlBuffer::new(0, gl::ARRAY_BUFFER, 0);
        let mut ebo = GlBuffer::new(1, gl::ELEMENT_ARRAY_BUFFER, 0);

        if let Some(tri_mesh) = triangle_mesh {
            vbo.set_data(&tri_mesh.verticies, gl::STATIC_DRAW);
            ebo.set_data(&tri_mesh.vertex_indicies, gl::STATIC_DRAW);
            tri_mesh.enable_vertex_attributes();
            self.num_triangles = (tri_mesh.verticies.len() / 3) as u32;
        }

        self.vbo = Some(vbo);
        self.ebo = Some(ebo);

        unsafe {gl::Enable(gl::DEPTH_TEST)};
    }

    pub fn init_textures(&self, tex_path : Option<&str>) {
        if let Some(path) = tex_path {
            Texture::load(path)
        } else {
            Texture::load("textures/missing.jpg")
        }
    }

    pub fn draw(&self) {
        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);

            set_uniform(self.shader_program, "texture0", UniformType::INT(0));
            set_uniform(self.shader_program, "transformMatrix", UniformType::MAT4(self.camera_handler.transform_mat));
        
            // gl::DrawElements(gl::TRIANGLES, self.num_indicies as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_triangles as i32);
        }
    }
    
}