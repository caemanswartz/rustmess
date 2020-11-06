#[macro_use]
extern crate glium;

use std::io::Cursor;
use glium::{glutin, Surface, uniform};
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

// vertex devinition
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

fn main() {
    #[allow(unused_imports)]

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

// helper functions

fn draw<T: glium::uniforms::Uniforms>(target: &mut glium::Frame, vertices: &glium::VertexBuffer<Vertex>, program: &glium::Program, uniform: &T, params: &glium::DrawParameters) {
    target.draw(vertices, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), program, uniform, params).unwrap();
}

fn build_vertices(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    glium::vertex::VertexBuffer::new(display, &[
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ]).unwrap()
}

// load files as vector of bytes
fn load_bytes(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        _ => buffer,
    }
}
// convert bytes to diffuse texture
fn load_diffuse_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::SrgbTexture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, raw_image).unwrap()
}
//convert bytes to normal texture
fn load_normal_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::Texture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, raw_image).unwrap()
}

// constructs opengl program
fn build_program(display: &glium::Display, vertex_shader_file_path: &str, fragment_shader_file_path: &str) -> glium::Program {
    let vertex_shader_bytes = load_bytes(vertex_shader_file_path);
    let vertex_shader_src = String::from_utf8_lossy(&vertex_shader_bytes);
    let fragment_shader_bytes = load_bytes(fragment_shader_file_path);
    let fragment_shader_src = String::from_utf8_lossy(&fragment_shader_bytes);
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}

//constructs perspective matrix
fn perspective_matrix(target: &glium::Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

//constructs view matrix
fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}