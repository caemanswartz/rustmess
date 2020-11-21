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

#[derive(Debug,Clone,Copy)]
pub enum WaypointState {
    IncrementWaypoint,
    WaitingForPath,
    ReachedGoal
}

#[derive(Debug,Clone)]
pub struct Body {
    mass: f32,
    waypoint: Vec<NavVec3>,
    waypoint_state: WaypointState,
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
            waypoint: Vec::new(),
            waypoint_state: WaypointState::ReachedGoal,
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
        self.waypoint = navmesh.find_path(
            self.position,
            waypoint,
            NavQuery::Accuracy,
            NavPathMode::MidPoints
        ).unwrap();
        self.waypoint_state = WaypointState::IncrementWaypoint;
    }
    pub fn clr_waypoint(&mut self) {
        self.waypoint = Vec::new();
        self.waypoint_state = WaypointState::ReachedGoal;
    }
    pub fn update_time_step(&mut self, navmesh: &NavMesh, time_step: f32) {
        let acceleration = self.update_waypoint(navmesh, time_step);
        self.velocity = self.velocity + acceleration * time_step;
        self.position = self.position + self.velocity * time_step;
    }

    fn update_waypoint(&mut self, navmesh: &NavMesh, time_step: f32) -> NavVec3 {
        let n = self.waypoint.len();
        match self.waypoint_state {
            IncrementWaypoint => {
                if n == 0 {
                    self.waypoint_state = WaypointState::ReachedGoal;
                    self.update_waypoint(navmesh, time_step)
                } else {
                    self.waypoint_state = WaypointState::WaitingForPath;
                    self.update_waypoint(navmesh, time_step)
                }
            },
            WaitingForPath => {
                let waypoint = self.waypoint[0];
                let future = self.position + self.velocity * time_step;
                let unit_vector = get_normalized_distance_vector(future, waypoint);
                let predicted = future + unit_vector * time_step;
                if get_distance_scalar(self.position, waypoint) <= get_distance_scalar(self.position, predicted) {
                
                    self.set_waypoint(navmesh, self.waypoint[n-1]);
                }
                self.waypoint_state = WaypointState::IncrementWaypoint;
                unit_vector
            }
            _ => {(0.0,0.0,0.0).into()}
        }
    }

}
/* simple apthfinding
    while not at goal
    pick a direction to move toward the goal
    if direction is clear
    move there
    else pick another direction
 */
fn get_normalized_distance_vector(vec1: NavVec3, vec2: NavVec3) -> NavVec3{
    let dx = vec1.x - vec2.x;
    let dy = vec1.y - vec2.y;
    let dz = vec1.z - vec2.z;
    let w = dx * dx + dy * dy + dz * dz;
    NavVec3 {
        x: dx / w,
        y: dy / w,
        z: dz / w
    }
}
fn get_distance_scalar(vec1: NavVec3, vec2: NavVec3) -> f32 {
    let dx = vec1.x - vec2.x;
    let dy = vec1.y - vec2.y;
    let dz = vec1.z - vec2.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}