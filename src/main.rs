extern crate glium;

use glium::{glutin, Surface};
use navmesh::{NavMesh,NavVec3};
use rand::Rng;

mod etc;
mod body;
mod gfx;

const MS_PER_UPDATE: u32 = 16;

fn main() {
    #[allow(unused_imports)]
    use crate::{
        etc::*,
        body::Body,
        gfx::{
            Graphic,
            GraphicLibrary
        }
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let load_time = std::time::Instant::now();
    let library = GraphicLibrary::load(&display, "assets");
    println!("Loaded assets folder in {:?}", load_time.elapsed());

    let scale = [0.25, 0.25, 0.25];
    let orientation = [0.0, 0.0, 0.0, 1.0];
    /*
    let orientation_by: [f32; 4] = cgmath::Quaternion::from(
        cgmath::Euler{
            x: cgmath::Deg(1.2),
            y: cgmath::Deg(0.6),
            z: cgmath::Deg(0.0)
        }).into();
    */
    // construct navmesh verticies
    let vertices = vec![
        (1.0,1.0,0.0).into(),
        (-1.0,1.0,0.0).into(),
        (1.0,-1.0,0.0).into(),
        (-1.0,-1.0,0.0).into()
    ];
    // construct navmesh triangles
    let triangles = vec![
        (0,1,2).into(),
        (1,2,3).into()
    ];
    // construct navmesh object
    let navmesh = NavMesh::new(vertices,triangles).unwrap();
    let test1 = NavVec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let test2 = NavVec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0
    };
    let test3 = test1 - test2;
    println!("{:?}", test3);

    // construct bodies to move and draw
    let origin = [0.0, 0.0, 0.0];
    let mut bodies = [
        Body::new(
            1.0,
            [0.0, 0.0, 0.0],
            origin,
            orientation,
            Graphic::new(
                scale,
                "tetrahedron".to_string(),
                "d4texture".to_string())),
        Body::new(
            1.0,
            [1.0, 1.0, 0.0],
            origin,
            orientation, 
            Graphic::new(
                scale,
                "hexahedron".to_string(),
                "d6texture".to_string())),
        Body::new(
            1.0,
            [0.0, 1.0, 0.0],
            origin,
            orientation, 
            Graphic::new(
                scale,
                "octahedron".to_string(),
                "d8texture".to_string())),
        Body::new(
            1.0,
            [0.0, 1.0, 0.0],
            origin,
            orientation, 
            Graphic::new(
                scale,
                "trapezohedron".to_string(),
                "d10texture".to_string())),
        Body::new(
            1.0,
            [0.0, -1.0, 0.0],
            origin,
            orientation, 
            Graphic::new(
                scale,
                "dodecahedron".to_string(),
                "d12texture".to_string())),
        Body::new(
            1.0,
            [-1.0, 0.0, 0.0],
            origin,
            orientation, 
            Graphic::new(
                scale,
                "icosahedron".to_string(),
                "d20texture".to_string()))
    ];

    // give waypoints to bodies
    let mut rng = rand::thread_rng();
    for body in &mut bodies {
        let rn1 = rng.gen::<f32>() * 2.0 - 1.0;
        let rn2 = rng.gen::<f32>() * 2.0 - 1.0;
        println!("({},{},0.0)", rn1, rn2);
        body.set_waypoint(
            &navmesh,
            (
                rn1,
                rn2,
                0.0
            ).into()
        );
    }

    let program = build_program(&display, "assets/vertex_shader.glsl", "assets/fragment_shader.glfl");
    let mut last_time = std::time::Instant::now();
    let mut lag = 0u32;
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
// update clock
        let current_time = std::time::Instant::now();
        let elapsed_time = current_time.duration_since(last_time).as_millis() as u32;
        last_time = current_time;
        lag += elapsed_time;

// process events
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ =>  (),
        }
// update
        while lag >= MS_PER_UPDATE {
            for body in &mut bodies {
                body.update_time_step(MS_PER_UPDATE as f32 / 1000.0);
            };
            lag -= MS_PER_UPDATE;
        }
// render
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        
        
        let view = view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);
        let perspective = perspective_matrix(&target);

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        for body in &mut bodies {
            body.draw(&mut target, &library, view, perspective, light, &program, &params);
        };

        target.finish().unwrap();
    });
}