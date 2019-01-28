use gl;
use gl::types::{ GLuint };

use crate::shaders::shader;
use crate::shaders::shader::SupportedShaderType;

use crate::shaders::{ GL_COMPILE_FAILURE, create_gl_log_buffer };

pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {

    pub fn new(vertex_shader: shader::Shader, fragment_shader: shader::Shader) -> ShaderProgram {

        match (vertex_shader.kind(), fragment_shader.kind()) {

            (SupportedShaderType::Vertex, SupportedShaderType::Fragment) => {
                let program_id = unsafe { gl::CreateProgram() };

                ShaderProgram::init_gl_program(program_id, vertex_shader.id(), fragment_shader.id());

                ShaderProgram { id: program_id }
            }
            _ => { panic!("Unsupported Shader type pair, cannot create new ShaderProgram"); }
        }
    }

    fn init_gl_program(program_id: u32, vert_id: GLuint, frag_id: GLuint) {
        unsafe { gl::AttachShader(program_id, vert_id); }
        unsafe { gl::AttachShader(program_id, frag_id); }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == GL_COMPILE_FAILURE {
            ShaderProgram::handle_gl_init_failure(program_id);
        }

        unsafe { gl::DetachShader(program_id, vert_id); }
        unsafe { gl::DetachShader(program_id, frag_id); }
    }

    fn handle_gl_init_failure(program_id: u32) {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error_log = create_gl_log_buffer(len as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error_log.as_ptr() as *mut gl::types::GLchar
            );
        }

        panic!("ShaderProgram creation failed: {}", error_log.to_owned().to_string_lossy());
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {

        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}