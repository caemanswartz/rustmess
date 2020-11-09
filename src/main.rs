extern crate glium;

use glium::{glutin, Surface};

mod etc;
mod album;
mod actor;

fn main() {
    #[allow(unused_imports)]
    use crate::{
        etc::*,
        actor::Actor,
        album::Album
    };

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let load_time = std::time::Instant::now();
    let mut album = Album::new();
    album.load_path(&display, "assets");
    println!("Loaded assets folder in {:?}", load_time.elapsed());

    let scale = [0.25, 0.25, 0.25];
    let orientation = [0.0, 0.0, 0.0, 1.0];
    let mut actors = [
        Actor::new([1.0, 0.0, 0.0], orientation, scale,"tetrahedron".to_string(),"d4texture".to_string()),
        Actor::new([1.0, 1.0, 0.0], orientation, scale,"hexahedron".to_string(),"d6texture".to_string()),
        Actor::new([0.0, 1.0, 0.0], orientation, scale,"octahedron".to_string(),"d8texture".to_string()),
        Actor::new([0.0, 1.0, 1.0], orientation, scale,"trapezohedron".to_string(),"d10texture".to_string()),
        Actor::new([0.0, -1.0, 0.0], orientation, scale,"dodecahedron".to_string(),"d12texture".to_string()),
        Actor::new([-1.0, 0.0, 0.0], orientation, scale,"icosahedron".to_string(),"d20texture".to_string())
    ];

    let program = build_program(&display, "assets/vertex_shader.glsl", "assets/fragment_shader.glfl");
    let t = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {

        let p = std::time::Instant::now().duration_since(t).as_millis() as f32;
        let r: f32 = (p / 100.0) % 360.0;

        let rotation: [f32; 4] = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Deg(90.0 + r),
            y: cgmath::Deg(45.0),
            z: cgmath::Deg(15.0 + r)
        }).into();

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
            _ =>  (),
        }

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

        for actor in &mut actors {
            actor.orientation_to(rotation);
            actor.draw(&mut target, &album, view, perspective, light, &program, &params);
        };

        target.finish().unwrap();
    });
}