use gl;
use std::ffi::{ CStr };
use gl::types::{ GLuint, GLint, GLchar, GLenum };
use crate::shaders::{ GL_COMPILE_FAILURE, create_gl_log_buffer };

#[derive(Copy, Clone)]
pub enum SupportedShaderType {
    Vertex,
    Fragment
}

impl SupportedShaderType {

    pub fn gl_value(&self) -> GLenum {

        match self {
            SupportedShaderType::Vertex => gl::VERTEX_SHADER,
            SupportedShaderType::Fragment => gl::FRAGMENT_SHADER
        }

    }

}

pub struct Shader {
    id: GLuint,
    kind: SupportedShaderType
}

impl Shader {

    pub fn new(src: &CStr, kind: SupportedShaderType) -> Shader {

        match load_shader_from_src(src, kind) {
            Ok(id) => Shader { id, kind },
            Err(log) => {
                panic!("Error creating new Shader: {}", log);
            }
        }

    }

    pub fn new_vert(src: &CStr) -> Shader {
        let kind = SupportedShaderType::Vertex;
        match load_shader_from_src(src, kind) {
            Ok(id) => Shader { id, kind },
            Err(log) => {
                panic!("Error creating new Vertex Shader: {}", log);
            }
        }

    }

    pub fn new_frag(src: &CStr) -> Shader {
        let kind = SupportedShaderType::Fragment;
        match load_shader_from_src(src, kind) {
            Ok(id) => Shader { id, kind },
            Err(log) => {
                panic!("Error creating new Fragment Shader: {}", log);
            }
        }

    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn kind(&self) -> SupportedShaderType {
        self.kind
    }

}

impl Drop for Shader {

    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }

}

pub fn load_shader_from_src(src: &CStr, kind: SupportedShaderType) -> Result<GLuint, String> {

    let compile_info = compile_shader_from_src(src, kind);

    if compile_info.compile_status == GL_COMPILE_FAILURE {

        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(compile_info.shader_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error_buffer = create_gl_log_buffer(len as usize + 1);

        unsafe {
            gl::GetShaderInfoLog(
                compile_info.shader_id,
                len,
                std::ptr::null_mut(),
                error_buffer.as_ptr() as *mut GLchar
            );
        }

        return Err(error_buffer.to_string_lossy().into_owned());
    } else {
        return Ok(compile_info.shader_id);
    }
}

pub struct ShaderCompileInfo {
    shader_id: GLuint,
    compile_status: GLint
}

//GLuint in result is the id of the compiled shader, GLint is the success code
pub fn compile_shader_from_src(src: &CStr, kind: SupportedShaderType) -> ShaderCompileInfo {

    let shader_id = unsafe { gl::CreateShader(kind.gl_value()) };

    unsafe {
        gl::ShaderSource(shader_id, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);
    }

    let mut compile_status: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);
    }

    ShaderCompileInfo { shader_id, compile_status } 

}