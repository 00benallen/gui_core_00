use crate::app::Vertex2D;
use gfx::handle::ShaderResourceView;

pub struct Color {

    rgb_value: [f32; 4]

}

pub type Texture = ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;

impl Color {

    pub fn new(r: f32, g: f32, b: f32, alpha: f32) -> Option<Color> {
        
        if r > 255.0 || g > 255.0 || b > 255.0 {
            return None;
        } else if r < 0.0 || g < 0.0 || b < 0.0 {
            return None;
        } else {
            return Some(Color {rgb_value: [r, g, b, alpha]});
        }

    }

    pub fn get_rgb_value(&self) -> [f32; 4] {

        return self.rgb_value;

    }

}

pub trait Shape {

    fn vertices(&self) -> Vec<Vertex2D>;

    fn color(&self) -> &Option<Color>;

    fn texture(&self) -> &Option<Texture>;

}

pub struct Triangle {

    vertices: [Vertex2D; 3],
    color: Option<Color>,
    texture: Option<Texture>

}

impl Triangle {

    pub fn new(v1: Vertex2D, v2: Vertex2D, v3: Vertex2D, color: Option<Color>, texture: Option<Texture>) -> Triangle {

        let vertices = [v1, v2, v3];

        Triangle { vertices, color, texture }

    }

}

impl Shape for Triangle {

    fn vertices(&self) -> Vec<Vertex2D> {
        let mut vec: Vec<Vertex2D> = Vec::new();
        vec.extend_from_slice(&self.vertices);
        vec
    }

    fn color(&self) -> &Option<Color> {
        &self.color
    }

    fn texture(&self) -> &Option<Texture> {
        &self.texture
    }


}

