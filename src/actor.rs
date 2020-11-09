
use crate::album::Album;

#[derive(Debug,Clone)]
pub struct Actor {
    position: [f32; 3],
    orientation: [f32; 4],
    scale: [f32; 3],
    object_key: String,
    texture_key: String
}

impl Actor {
    pub fn new(
        position: [f32; 3],
        orientation: [f32; 4],
        scale: [f32; 3],
        object_key: String,
        texture_key: String
    ) -> Actor {
        Actor {
            position,
            orientation,
            scale,
            object_key,
            texture_key
        }
    }
    pub fn position_to(&mut self, translation: [f32; 3]) {
        self.position = translation;
    }
    pub fn orientation_to(&mut self, quaternion: [f32; 4]) {
        self.orientation = quaternion;
    }
    pub fn draw(&self, target: &mut glium::Frame, album: &Album, view: [[f32;4]; 4], perspective: [[f32;4]; 4],
                u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        album.draw(target, &self.object_key, &self.texture_key, self.position, self.orientation, self.scale,
                   view, perspective, u_light, program, params);
    }
    
}