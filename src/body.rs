use crate::gfx::{
    Graphic,
    GraphicLibrary
};
use navmesh::{
    NavQuery,
    NavPathMode,
    NavVec3,
    NavMesh
};
#[derive(Debug,Clone)]
pub struct Body {
    mass: f32,
    waypoint: Box<Vec<NavVec3>>,
    position: NavVec3,
    velocity: NavVec3,
    orientation: [f32; 4],
    model: Graphic
}
#[allow(dead_code)]
impl Body {
    pub fn new(
        mass: f32,
        position: [f32; 3],
        velocity: [f32; 3],
        orientation: [f32; 4],
        model: Graphic
    ) -> Body {
        Body {
            mass,
            waypoint: Box::new(<Vec::<NavVec3>>::new()),
            position: position.into(),
            velocity: velocity.into(),
            orientation,
            model
        }
    }
    pub fn draw(&self, target: &mut glium::Frame, library: &GraphicLibrary, view: [[f32;4]; 4], perspective: [[f32;4]; 4],
        u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        self.model.draw(target, library,
            [
                self.position.x,
                self.position.y,
                self.position.z
            ],
            self.orientation, view, perspective, u_light, program, params);
    }

    pub fn set_waypoint(&mut self,navmesh: &NavMesh, waypoint: NavVec3) {
        self.waypoint.clear();
        self.waypoint.append(
            &mut navmesh.find_path(
                self.position,
                waypoint,
                NavQuery::Accuracy,
                NavPathMode::MidPoints
        ).unwrap());
    }
    pub fn update_time_step(&mut self, nav_mesh: &NavMesh, time_step: f32) {
        let acceleration = self.update_waypoint(nav_mesh, time_step);
        self.velocity = self.velocity + acceleration * time_step;
        self.position = self.position + self.velocity * time_step;
    }

    fn update_waypoint(&mut self, nav_mesh: &NavMesh, time_step: f32) -> NavVec3 {
        let max_velocity = 1.0;
        let future = self.position + self.velocity * time_step;
        let n = self.waypoint.len();
        // come to a stop if no waypoint
        if n == 0 {
            get_normalized_distance_vector(future,self.position)
        } else {
            // plan movement based on velocity and waypoint
            let waypoint = self.waypoint[0];
            let velocity_scalar = get_distance_scalar((0.0,0.0,0.0).into(),self.velocity);
            let velocity_correction = get_normalized_distance_vector((0.0,0.0,0.0).into(), self.velocity);
            let velocity_desired = get_normalized_distance_vector(self.position,waypoint);
            let planned_vector = velocity_desired * (max_velocity - velocity_scalar).max(0.0)
                - velocity_correction * velocity_scalar.min(1.0);
            let predicted = future + planned_vector * time_step;
            let distance_waypoint = get_distance_scalar(self.position, waypoint);
            // remove waypoint if passed
            if distance_waypoint <= get_distance_scalar(self.position, predicted) {
                let m = get_distance_scalar((0.0,0.0,0.0).into(),self.velocity);
                if n > 1 || m < time_step {
                    self.waypoint.remove(0);
                }
            }
            planned_vector
        }
    }
}
fn get_normalized_distance_vector(vec1: NavVec3, vec2: NavVec3) -> NavVec3 {
    let d = vec2 - vec1;
    let w = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
    if w == 0.0 {
        (0.0,0.0,0.0).into()
    } else {
        d / w
    }
}
fn get_distance_scalar(vec1: NavVec3, vec2: NavVec3) -> f32 {
    let d = vec2 - vec1;
    (d.x * d.x + d.y * d.y + d.z * d.z).sqrt()
}