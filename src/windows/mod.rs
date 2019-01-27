use std::sync::mpsc;
use gl::types::GLfloat;
use glfw::{ Context, Glfw, Key, Action };

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
        let glfw_handle = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        println!("Intializing GLFW window and event channel");
        let (mut glfw_window_handle, glfw_events_receiver) = glfw_handle.create_window(300, 300, "Test Window", glfw::WindowMode::Windowed)
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

    pub fn init_event_loop<F>(&mut self, handler: F) where F: Fn(&mut glfw::Window, glfw::WindowEvent) {      

        while !self.glfw_window_handle.should_close() {
            
            self.glfw_handle.poll_events();
            for (_, event) in glfw::flush_messages(&self.glfw_events_receiver) {
                handler(&mut self.glfw_window_handle, event);
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.glfw_window_handle.swap_buffers();

            if !self.initialization_finished {
                let (cur_x, cur_y) = self.glfw_window_handle.get_pos();

                self.glfw_window_handle.set_pos(0, 0);
                self.glfw_window_handle.set_pos(cur_x, cur_y);
                self.initialization_finished = true;
            }
            
        }
    }
}

pub fn init_window() {

    GameWindow::new((0.3, 0.3, 0.5, 1.0), true, Some(handle_window_event));

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    println!("{:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
