use crate::{
    gfx::GraphicLibrary,
    model::Model
};

#[derive(Debug,Clone)]
pub struct Body {
    position: [f32; 3],
    velocity: [f32; 3],
    orientation: [f32; 4],
    model: Model
}

impl Body {
    pub fn new(
        position: [f32; 3],
        velocity: [f32; 3],
        orientation: [f32; 4],
        model: Model
    ) -> Body {
        Body {
            position,
            velocity,
            orientation,
            model
        }
    }
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }
    pub fn apply_time_step(&mut self, time_step: f32) {
        self.position[0] += self.velocity[0] * time_step;
        self.position[1] += self.velocity[1] * time_step;
        self.position[2] += self.velocity[2] * time_step;
    }
    pub fn flip_velocity(&mut self) {
        self.velocity[0] = -self.velocity[0];
        self.velocity[1] = -self.velocity[1];
        self.velocity[2] = -self.velocity[2];
    }
    pub fn draw(&self, target: &mut glium::Frame, library: &GraphicLibrary, view: [[f32;4]; 4], perspective: [[f32;4]; 4],
        u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        self.model.draw(target, library, self.position, self.orientation, view, perspective, u_light, program, params);
    }
}