use std::io::Cursor;
use glium::Surface;
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

// vertex devinition
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);

// model structure for drawing
#[derive(Debug)]
pub struct Model {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub diffuse_map: glium::texture::SrgbTexture2d,
    pub normal_map: glium::texture::Texture2d,
    pub program: glium::Program
}
impl Model {
    pub fn draw(&self,target: &mut glium::Frame, model: [[f32;4]; 4], view: [[f32;4]; 4], perspective: [[f32;4]; 4], u_light: [f32; 3], params: &glium::DrawParameters) {
        target.draw(&self.vertices, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &self.program, &uniform!{ model: model, view: view, perspective: perspective,
            u_light: u_light, diffuse_tex: &self.diffuse_map, normal_tex: &self.normal_map}, params).unwrap();
    }
}

pub fn build_vertices(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
    glium::vertex::VertexBuffer::new(display, &[
        Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
    ]).unwrap()
}

// load files as vector of bytes
pub fn load_bytes(file_path: &str) -> Vec<u8> {
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
pub fn load_diffuse_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::SrgbTexture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, raw_image).unwrap()
}
//convert bytes to normal texture
pub fn load_normal_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::Texture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, raw_image).unwrap()
}

// constructs opengl program
pub fn build_program(display: &glium::Display, vertex_shader_file_path: &str, fragment_shader_file_path: &str) -> glium::Program {
    let vertex_shader_bytes = load_bytes(vertex_shader_file_path);
    let vertex_shader_src = String::from_utf8_lossy(&vertex_shader_bytes);
    let fragment_shader_bytes = load_bytes(fragment_shader_file_path);
    let fragment_shader_src = String::from_utf8_lossy(&fragment_shader_bytes);
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}

//constructs perspective matrix
pub fn perspective_matrix(target: &glium::Frame) -> [[f32; 4]; 4] {
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
pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
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