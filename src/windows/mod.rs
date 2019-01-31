use std::sync::mpsc;
use gl::types::GLfloat;
use glfw::{ Context, Glfw, Key, Action, WindowHint, OpenGlProfileHint };
use crate::shaders;
use crate::shaders::shader_program::ShaderProgram;

pub type GLColor = (GLfloat, GLfloat, GLfloat, GLfloat);

pub struct GameWindow {

    glfw_handle: Glfw,
    glfw_window_handle: glfw::Window,
    glfw_events_receiver: mpsc::Receiver<(f64, glfw::WindowEvent)>,
    background_color: GLColor,
    initialization_finished: bool

}

impl GameWindow {

    pub fn new<F>(background_color: GLColor, auto_init: bool, handler: Option<F>) -> GameWindow
        where F: Fn(&mut glfw::Window, glfw::WindowEvent) {

        println!("Initializing GLFW");
        let glfw_handle = init_glfw_handle();

        println!("Intializing GLFW window and event channel");
        let (mut glfw_window_handle, glfw_events_receiver) = glfw_handle.create_window(2880, 1800, "Test Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

        glfw_window_handle.set_key_polling(true);
        glfw_window_handle.make_current();

        println!("Initializing OpenGL");
        let _gl = gl::load_with(|s| glfw_window_handle.get_proc_address(s) as *const std::os::raw::c_void);

        println!("Initializing GameWindow");
        let mut game_window = GameWindow { 
            glfw_handle, 
            glfw_window_handle, 
            glfw_events_receiver, 
            background_color, 
            initialization_finished: false };

        unsafe {
            println!("Setting background color in OpenGL");
            gl::ClearColor(
                game_window.background_color.0, 
                game_window.background_color.1, 
                game_window.background_color.2, 
                game_window.background_color.3);
        };

        if auto_init {
            match handler {
                Some(handler) => { 
                    println!("Auto-initializing event loop");
                    game_window.init_event_loop(handler);
                },
                None => eprintln!("Auto-init of event handler could not be completed because no event handler function was given")
            };
        } else {
            println!("Warning: Event loop not initialized, initialize manually to process GLFW window events");
        }

        

        game_window
    }

    fn init_event_loop<F>(&mut self, handler: F) where F: Fn(&mut glfw::Window, glfw::WindowEvent) {

        let shader_program = init_default_shader_program();

        let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0];

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        unsafe {
            gl::BindVertexArray(vao);
    
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        while !self.glfw_window_handle.should_close() {
        
            self.glfw_handle.poll_events();
            for (_, event) in glfw::flush_messages(&self.glfw_events_receiver) {
                handler(&mut self.glfw_window_handle, event);
            }

            unsafe {
                gl::Viewport(0, 0, 900, 700); // set viewport
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();
            unsafe {
                gl::BindVertexArray(vao);

                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // starting index in the enabled arrays
                    3 // number of indices to be rendered
                );
            }

            self.glfw_window_handle.swap_buffers();

            if !self.initialization_finished {
                let (cur_x, cur_y) = self.glfw_window_handle.get_pos();

                self.glfw_window_handle.set_pos(20, 20);
                self.glfw_window_handle.set_pos(cur_x, cur_y);
                self.initialization_finished = true;
            }
        }
        
    }
}

fn init_glfw_handle() -> Glfw {
    println!("Initializing GLFW");
    let mut glfw_handle = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_handle.window_hint(WindowHint::ContextVersion(3, 2));
    glfw_handle.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw_handle.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    glfw_handle
}

fn init_default_shader_program() -> ShaderProgram {

    use std::ffi::CString;

    let vert_shader = shaders::shader::Shader::new_vert(
        &CString::new(include_str!("../triangle.vert")).unwrap());

    let frag_shader = shaders::shader::Shader::new_frag(
        &CString::new(include_str!("../triangle.frag")).unwrap());  

    let shader_program = shaders::shader_program::ShaderProgram::new(vert_shader, frag_shader);

    shader_program.set_used();

    shader_program
}

//TODO remove from library both
pub fn test_init_window() {

    GameWindow::new((0.3, 0.3, 0.5, 1.0), true, Some(test_handle_window_event));

}

fn test_handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    println!("{:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
