
use crate::album::Album;

#[derive(Debug,Clone)]
pub struct Actor {
    position: [f32; 3],
    orientation: [f32; 4],
    scale: [f32; 3],
    object_key: String,
    texture_key: String
}
#[allow(dead_code)]
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
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }
    pub fn position_to(&mut self, translation: [f32; 3]) {
        self.position = translation;
    }
    pub fn position_by(&mut self, translation: [f32; 3]) {
        self.position[0] += translation[0];
        self.position[1] += translation[1];
        self.position[2] += translation[2];
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