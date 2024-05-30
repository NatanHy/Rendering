use gl::types::GLuint;
use std::ffi::CString;

pub enum UniformType {
    INT(i32),
    FLOAT(f32),
    VEC2([f32;2]),   
    VEC3([f32;3]),
    VEC4([f32;4])
}

pub fn set_uniform(shader_program : GLuint, uniform_name : &str, uniform_value : UniformType) {
    // Get the location of the uniform variable in the shader program
    let uniform_location = unsafe {
        gl::GetUniformLocation(shader_program, CString::new(uniform_name).unwrap().as_ptr())
    };

    // Check if the uniform location is valid (-1 means not found)
    if uniform_location != -1 {
        unsafe {
            match uniform_value {
                UniformType::VEC2(x) => gl::Uniform2f(uniform_location, x[0], x[1]),
                UniformType::VEC3(x) => gl::Uniform3f(uniform_location, x[0], x[1], x[2]),
                UniformType::VEC4(x) => gl::Uniform4f(uniform_location, x[0], x[1], x[2], x[3]),
                UniformType::FLOAT(x) => gl::Uniform1f(uniform_location, x),
                UniformType::INT(x) => gl::Uniform1i(uniform_location, x),
            }
            
        }
    } else {
        println!("Uniform location {} not found", uniform_name);
    } 
}