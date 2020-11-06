#[macro_use]
extern crate glium;

use glium::{glutin, Surface, uniform};

mod graphics;

fn main() {
    #[allow(unused_imports)]
    use graphics::*;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

// onetime data constuction: TODO pull out from main

    let vertices = build_vertices(&display);

    let diffuse_map = load_diffuse_map(&display, image::ImageFormat::Jpeg, "src/tuto-14-diffuse.jpg");
    let normal_map = load_normal_map(&display, image::ImageFormat::Png, "src/tuto-14-normal.png");

    let program = build_program(&display, "src/vertex_shader.glsl", "src/fragment_shader.glfl");

// start of event loop: KEEP

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

// (re)drawing object params: TODO pull and replace with a single function call

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

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
  
        draw(&mut target, &vertices, &program, &uniform!{ model: model, view: view, perspective: perspective,
             u_light: light, diffuse_tex: &diffuse_map, normal_tex: &normal_map}, &params);
// draw to screen: KEEP
        target.finish().unwrap();
    });
}