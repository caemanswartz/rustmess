#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

mod graphics;

fn main() {
    #[allow(unused_imports)]
    use graphics::*;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let tetra = Model::from_files(&display, "assets/tetrahedron.obj", "assets/d4_diffuse_texture.jpg", "assets/d4_normal_map.png");
    //let icosa = Model::from_files(&display, "assets/icosahedron.obj", "assets/d20_diffuse_texture.jpg", "assets/d20_normal_map.png");

    let program = build_program(&display, "assets/vertex_shader.glsl", "assets/fragment_shader.glfl");
    let mut t: f32 = 0.0;
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        t += 0.005;
        if t > 1.0 {
            t = 0.0
        }

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
        let position0 = [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let scale = [
            [0.25, 0.0, 0.0, 0.0],
            [0.0, 0.25, 0.0, 0.0],
            [0.0, 0.0, 0.25, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let rotation = [
            [t.cos(), -t.sin(), 0.0, 0.0],
            [t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        // position * rotation * scale

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

        tetra.draw(&mut target, position0, rotation, scale, view, perspective, light, &program, &params);
        //icosa.draw(&mut target, position1, rotation, scale, view, perspective, light, &program, &params);

        target.finish().unwrap();
    });
}