use crate::gfx::{
    Graphic,
    GraphicLibrary
};

#[derive(Debug,Clone)]
pub struct Body {
    mass: f32,
    target: [f32; 3],
    position: [f32; 3],
    velocity: [f32; 3],
    orientation: [f32; 4],
    model: Graphic
}
#[allow(dead_code)]
impl Body {
    pub fn new(
        mass: f32,
        target: [f32; 3],
        position: [f32; 3],
        velocity: [f32; 3],
        orientation: [f32; 4],
        model: Graphic
    ) -> Body {
        Body {
            mass,
            target,
            position,
            velocity,
            orientation,
            model
        }
    }
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }
    pub fn get_target(&self) -> [f32;3] {
        self.target
    }
    pub fn set_target(&mut self, target: [f32; 3]) {
        self.target = target;
    }
    pub fn apply_time_step(&mut self, time_step: f32) {
        self.move_towards_target(time_step);
        self.apply_velocity(time_step);
    }
    fn apply_velocity(&mut self, time_step: f32) {
        self.position[0] += self.velocity[0] * time_step;
        self.position[1] += self.velocity[1] * time_step;
        self.position[2] += self.velocity[2] * time_step;
    }
    fn apply_acceleration(&mut self, acceleration: [f32;3], time_step: f32) {
        self.velocity[0] += acceleration[0] * time_step;
        self.velocity[1] += acceleration[1] * time_step;
        self.velocity[2] += acceleration[2] * time_step;
    }
    fn move_towards_target(&mut self, time_step: f32) {
        if self.get_distance(self.target) < 0.1 {
            self.apply_acceleration(
                [
                    -self.velocity[0],
                    -self.velocity[1],
                    -self.velocity[2]
                ],
                time_step);
        } else {
            self.apply_acceleration(self.towards_target(), time_step);
        }
    }
    fn towards_stop(&self) -> [f32; 3] {
        let x = self.velocity[0];
        let y = self.velocity[1];
        let z = self.velocity[2];
        let l = (x * x + y * y + z * z).sqrt();
        [
            -(x / l),
            -(y / l),
            -(z / l)
        ]
    }
    fn towards_target(&self) -> [f32; 3] {
        let x = self.target[0] - self.position[0]; 
        let y = self.target[1] - self.position[1];
        let z = self.target[2] - self.position[2];
        let l = (x * x + y * y + z * z).sqrt();
        [
            x / l,
            y / l,
            z / l
        ]
    }
    fn get_distance(&self, target: [f32; 3]) -> f32 {
        let x = self.position[0] - target[0];
        let y = self.position[1] - target[1];
        let z = self.position[2] - target[2];
        (x * x + y * y + z * z).sqrt()

    }
    pub fn draw(&self, target: &mut glium::Frame, library: &GraphicLibrary, view: [[f32;4]; 4], perspective: [[f32;4]; 4],
        u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        self.model.draw(target, library, self.position, self.orientation, view, perspective, u_light, program, params);
    }
}