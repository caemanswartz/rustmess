extern crate glium;

use glium::{glutin, Surface};
use cgmath;

mod etc;
mod body;
mod model;
mod gfx;

const MS_PER_UPDATE: u32 = 16;

fn main() {
    #[allow(unused_imports)]
    use crate::{
        etc::*,
        body::Body,
        model::Model,
        gfx::GraphicLibrary
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let load_time = std::time::Instant::now();
    let mut library = GraphicLibrary::new();
    library.load_path(&display, "assets");
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
    let mut bodies = [
        Body::new([0.0, 0.0, 0.0], [1.0,0.0,0.0], orientation, Model::new(scale,"tetrahedron".to_string(),"d4texture".to_string())),
        Body::new([1.0, 1.0, 0.0], [1.0,1.0,0.0], orientation, Model::new(scale,"hexahedron".to_string(),"d6texture".to_string())),
        Body::new([0.0, 1.0, 0.0], [0.0,1.0,0.0], orientation, Model::new(scale,"octahedron".to_string(),"d8texture".to_string())),
        Body::new([0.0, 1.0, 1.0], [0.0,1.0,1.0], orientation, Model::new(scale,"trapezohedron".to_string(),"d10texture".to_string())),
        Body::new([0.0, -1.0, 0.0], [0.0,0.0,1.0], orientation, Model::new(scale,"dodecahedron".to_string(),"d12texture".to_string())),
        Body::new([-1.0, 0.0, 0.0], [0.0,0.0,0.0], orientation, Model::new(scale,"icosahedron".to_string(),"d20texture".to_string()))
    ];

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
                let position = body.get_position();
                if position[0] < -1.0 || position[0] > 1.0 
                   || position[1] < -1.0 || position[1] > 1.0
                   || position[2] < -1.0 || position[2] > 1.0 {
                    body.flip_velocity();
                }
                body.apply_time_step(MS_PER_UPDATE as f32 / 1000.0);
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