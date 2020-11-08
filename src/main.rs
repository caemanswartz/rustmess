extern crate glium;

use glium::{glutin, Surface};

mod etc;
mod model;

fn main() {
    #[allow(unused_imports)]
    use crate::{
        etc::*,
        model::Model
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let tetra = Model::from_files(&display, "assets/cube.obj", "assets/CubeTexture.jpg", "assets/Cube.001.png");
    let icosa = Model::from_files(&display, "assets/icosahedron.obj", "assets/d20_diffuse_texture.jpg", "assets/d20_normal_map.png");

    let program = build_program(&display, "assets/vertex_shader.glsl", "assets/fragment_shader.glfl");
    let t = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);


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
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        
        let p = std::time::Instant::now().duration_since(t).as_millis() as f32;
        let r: f32 = (p / 100.0) % 360.0;
        
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

        let rotation1: [f32; 4] = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Deg(90.0 + r),
            y: cgmath::Deg(45.0),
            z: cgmath::Deg(15.0 + r)
        }).into();
        let rotation2: [f32; 4] = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Deg(90.0 + r),
            y: cgmath::Deg(45.0 + r),
            z: cgmath::Deg(15.0)
        }).into();

        tetra.draw(&mut target, [1.0, 0.0, 0.0], rotation1, [0.33,0.33,0.33], view, perspective, light, &program, &params);
        icosa.draw(&mut target, [-1.0, 0.0, 0.0], rotation2, [0.25,0.25,0.25], view, perspective, light, &program, &params);

        target.finish().unwrap();
    });
}