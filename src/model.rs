
use crate::gfx::GraphicLibrary;

#[derive(Debug,Clone)]
pub struct Model {
    scale: [f32; 3],
    object_key: String,
    texture_key: String
}
#[allow(dead_code)]
impl Model {
    pub fn new(
        scale: [f32; 3],
        object_key: String,
        texture_key: String
    ) -> Model {
        Model {
            scale,
            object_key,
            texture_key
        }
    }
    pub fn draw(&self, target: &mut glium::Frame, library: &GraphicLibrary, position: [f32;3], orientation: [f32;4], view: [[f32;4]; 4], perspective: [[f32;4]; 4],
                u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        library.draw(target, &self.object_key, &self.texture_key, position, orientation, self.scale,
                   view, perspective, u_light, program, params);
    }
}