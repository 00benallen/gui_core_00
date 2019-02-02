//Glutin imports
use glutin::dpi::LogicalSize;
use glutin::{ GlRequest, GlWindow };
use glutin::Api::OpenGl;

//Gfx imports
pub type ColorFormat = gfx::format::Srgba8; //tuple for color representation
pub type DepthFormat = gfx::format::DepthStencil; //not sure what a stencil is just yet

use gfx;
use gfx_device_gl::{ Device as GlDevice, Factory, Resources };
use gfx_core::handle::{ RenderTargetView, DepthStencilView };
use gfx_core::format::{ R8_G8_B8_A8, Srgb, D24_S8, Unorm };
use gfx::traits::FactoryExt;
use gfx::Device;


// Create a GFX pipeline
gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

//Scale to screen coords
const SCALE_TO_SCREEN_TRANSFORM: Transform = Transform {
        transform: [[0.000694444444444, 0.0, 0.0, 0.0],
                    [0.0, 0.001111111111111, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]]
};

//Define some constant data to draw, origin is in centre of screen, screen is 1.0x1.0 units at all times
//TODO remove, just for testing
const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -50.0, -50.0], uv: [1.0, 0.0] }, //bottom-left
    Vertex { pos: [  50.0, -50.0], uv: [0.0, 1.0] }, //bottom-right
    Vertex { pos: [  0.0,  50.0], uv: [1.0, 1.0] }, //top
];

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct GfxWindowHandles {

    window: GlWindow,
    device: GlDevice,
    factory: Factory,
    render_target: RenderTargetView<Resources, ( R8_G8_B8_A8, Srgb)>,
    depth_stencil: DepthStencilView<Resources, ( D24_S8, Unorm )>

}

pub struct GuiApplication00 {

    handles: GfxWindowHandles,
    vertex_buffer: Vec<Vertex>

}

#[allow(dead_code)]
impl GuiApplication00 {
    pub fn new(
        window_title: String, 
        dimensions: LogicalSize) -> Result<GuiApplication00, GuiInitError> {

        //init events loop for window
        let events_loop = glutin::EventsLoop::new();

        //initialize window, use inputted name
        let window_handles: GfxWindowHandles = init_window(window_title, &events_loop, dimensions)?;

        //intialize pipeline, vertex buffer will be empty, transformation matrix will just scale to screen coords
        let vertex_buffer = vec![];

        let mut new_app = GuiApplication00 { handles: window_handles, vertex_buffer };
    
        //TODO start event loop on its own thread so startup thread closes
        new_app.init_event_loop(events_loop)?;

        Ok(new_app)
    }

    fn init_event_loop(
        &mut self, 
        mut events_loop: glutin::EventsLoop) -> Result<(), GuiInitError> {

        //initialize pipeline, hardcode this for now, figure out if we need multiple ones or something
        let pso = self.handles.factory.create_pipeline_simple(
            include_bytes!("shaders/vert_shader.glslv"), //TODO, remove relative path?
            include_bytes!("shaders/frag_shader.glslf"),
            pipe::new()
        ).unwrap();

        //initialize command encoder
        let mut encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer> = self.handles.factory.create_command_buffer().into();

        let (gfx_vertex_buffer, slice) = self.handles.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let transform_buffer = self.handles.factory.create_constant_buffer(1);
        

        //Setup the texture and sampler to be sent into the pipeline
        //for now we will assume all shapes are textured at this level, maybe we will need to sometimes dynamically create textures
        let sampler = self.handles.factory.create_sampler_linear();
        let texture = gfx_load_texture(&mut self.handles.factory); //TODO: just setting up one texture for now

        let data = pipe::Data { //actually bind the data we want into the pipeline, so the encoder has it to execute
            vbuf: gfx_vertex_buffer,
            tex: (texture, sampler),
            transform: transform_buffer,
            out: self.handles.render_target.clone(),
        };

        let mut running = true;
        while running {
            events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent { event, .. } = event {
                    match event {
                        glutin::WindowEvent::CloseRequested |
                        glutin::WindowEvent::KeyboardInput {
                            input: glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                            }, ..
                        } => running = false,
                        _ => {}
                    }
                }
            });

            // Put in main loop before swap buffers and device clean-up method
            encoder.clear(&self.handles.render_target, BLACK); //clear the framebuffer with a color(color needs to be an array of 4 f32s, RGBa)
            encoder.update_buffer(&data.transform, &[SCALE_TO_SCREEN_TRANSFORM], 0).expect("Updating encoder buffer failed."); //update buffers
            encoder.draw(&slice, &pso, &data); // draw commands with buffer data and attached pso
            encoder.flush(&mut self.handles.device); // execute draw commands

            self.handles.window.swap_buffers().unwrap();
            self.handles.device.cleanup();
        }

        Ok(())

    }
}

fn init_window(
        window_title: String, 
        events_loop: &glutin::EventsLoop,
        dimensions: LogicalSize) -> Result<GfxWindowHandles, GuiInitError> {

        let windowbuilder = glutin::WindowBuilder::new()
        .with_title(window_title)
        .with_dimensions(dimensions);

        let contextbuilder = glutin::ContextBuilder::new()
            .with_gl(GlRequest::Specific(OpenGl,(3,2)))
            .with_vsync(true);
        let (window, device, factory, render_target, depth_stencil) =
            match gfx_window_glutin::init::<ColorFormat, DepthFormat>(windowbuilder, contextbuilder, &events_loop) {

                Ok(handles) => handles,
                Err(err) => {
                    eprintln!("gfx_window_glutin::init failed, cause: {}", err);
                    return Err(GuiInitError::WindowInit);
                }

            };
        
        return Ok(GfxWindowHandles { window, device, factory, render_target, depth_stencil });
}

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum GuiInitError {

    EventLoopInit,
    WindowInit


}

impl Error for GuiInitError { }

impl Display for GuiInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {

        match self {
            GuiInitError::EventLoopInit => write!(f, "[GuiInitError]: Event loop could not be initialized"),
            GuiInitError::WindowInit => write!(f, "[GuiInitError]: Window and its handles could not be initialized"),
        }
        

    }
}

fn gfx_load_texture<F, R>(factory: &mut F) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
            where F: gfx::Factory<R>, R: gfx::Resources {

    use gfx::format::Rgba8;
    let img = image::open("test.jpg").unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, gfx::texture::Mipmap::Provided, &[&img]).unwrap();
    view
}