use gl;
use std::ffi::{ CString };
use gl::types::{ GLint };

pub mod shader;
pub mod shader_program;

pub const GL_COMPILE_FAILURE: GLint = 0;

pub fn create_gl_log_buffer(capacity: usize) -> CString {

    // allocate buffer of correct size
    let mut byte_buffer: Vec<u8> = Vec::with_capacity(capacity);

    // fill it with len space bytes
    byte_buffer.extend([b' '].iter().cycle().take(capacity));

    // convert buffer to CString
    let buffer_to_string: CString = unsafe { CString::from_vec_unchecked(byte_buffer) };

    buffer_to_string

}

