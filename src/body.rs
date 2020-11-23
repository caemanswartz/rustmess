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
        match self.update_waypoint(nav_mesh, time_step) {
            Some(vector) => {
                self.velocity = self.velocity + vector * time_step;
                self.position = self.position + self.velocity * time_step;
            },
            None => {}
        }
    }

    fn update_waypoint(&mut self, _nav_mesh: &NavMesh, time_step: f32) -> Option<NavVec3> {
        let max_velocity = 1.0;
        let n = self.waypoint.len();
        // check for waypoints
        if n == 0 {
            None
        // plan movement based on velocity and waypoint
        } else {
            let waypoint = self.waypoint[0];
            // get velocity length
            let velocity_length = get_distance_scalar((0.0,0.0,0.0).into(),self.velocity);
            // get normalized velocity vector
            let velocity_correction = get_normalized_distance_vector((0.0,0.0,0.0).into(), self.velocity);
            // get vector to waypoint
            let velocity_desired = get_normalized_distance_vector(self.position,waypoint);
            // construct velocity adjustment from correction and desired to keep velocity scalar 1.0 or less
            //      desired vector multiplied by percentage of max velocity not used for correction
            //      correction vector multiplied by percetage of max velocity currently used
            let planned_vector = velocity_desired * (max_velocity - velocity_length).max(0.0)
                - velocity_correction * velocity_length.min(1.0);
            // predict fucutre location
            let future_position = self.position + self.velocity * time_step;
            // get position after planned move
            let predicted_position = future_position + planned_vector * time_step;
            // calculate predicted postion's distance to current waypoint
            let predicted_distance_to_waypoint = get_distance_scalar(predicted_position, waypoint);
            // check if moved further than current distance to waypoint
            if predicted_distance_to_waypoint <= get_distance_scalar(self.position, waypoint) {
                // check if multiple waypoints or velocity is very small
                if n > 1 || velocity_length < time_step {
                    self.waypoint.remove(0);
                    // stop movement if no more waypoints
                    if n == 0 {
                        self.velocity = (0.0,0.0,0.0).into();
                        return None
                    }
                }
            }
            Some(planned_vector)
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