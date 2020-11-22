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
    pub fn update_time_step(&mut self, time_step: f32) {
        let acceleration = self.update_waypoint(time_step);
        self.velocity = self.velocity + acceleration * time_step;
        self.position = self.position + self.velocity * time_step;
    }

    fn update_waypoint(&mut self, time_step: f32) -> NavVec3 {
        let future = self.position + self.velocity * time_step;
        if self.waypoint.len() == 0 {
            get_normalized_distance_vector(future,self.position)
        } else {
            let unit_vector = get_normalized_distance_vector(future, self.waypoint[0]);
            let predicted = future + unit_vector * time_step;
            if get_distance_scalar(self.position, self.waypoint[0]) <= get_distance_scalar(self.position, predicted) {
                self.waypoint.remove(0);
            }
            unit_vector
        }
    }
}
fn get_normalized_distance_vector(vec1: NavVec3, vec2: NavVec3) -> NavVec3 {
    let d = vec2 - vec1;
    let w = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
    if w == 0.0 {
        return (0.0,0.0,0.0).into()
    } else {
        d / w
    }
}
fn get_distance_scalar(vec1: NavVec3, vec2: NavVec3) -> f32 {
    let d = vec2 - vec1;
    (d.x * d.x + d.y * d.y + d.z * d.z).sqrt()
}