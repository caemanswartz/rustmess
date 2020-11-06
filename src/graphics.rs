use std::io::Cursor;
use obj::{
    load_obj, Obj,
    TexturedVertex
};
use glium::Surface;
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

// model structure for drawing
#[derive(Debug)]
pub struct Model {
    vertices: glium::VertexBuffer<TexturedVertex>,
    diffuse_map: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
}
impl Model {
    pub fn new(vertices: glium::VertexBuffer<TexturedVertex>, diffuse_map: glium:: texture::SrgbTexture2d, normal_map: glium::texture::Texture2d) -> Model {
        Model {
            vertices,
            diffuse_map,
            normal_map
        }
    }
    pub fn from_files(display: &glium::Display, object_file_path: &str, diffuse_file_path: &str, normal_file_path: &str) -> Model {
        Model::new(
            load_object_file(&display, object_file_path),
            load_diffuse_map(&display, image::ImageFormat::Jpeg, diffuse_file_path),
            load_normal_map(&display, image::ImageFormat::Png, normal_file_path)
        )

    }
    /// Draws model to frame
    /// takes   frame as glium::Frame
    ///         model transformation as [[f32;4]; 4]
    ///         view transformation as [[f32;4]; 4]
    ///         perspective transformation as [[f32;4]; 4]
    ///         light color as [f32; 3]
    ///         opengl program as glium::Program
    pub fn draw(&self,target: &mut glium::Frame, model: [[f32;4]; 4], view: [[f32;4]; 4], perspective: [[f32;4]; 4], u_light: [f32; 3], program: &glium::Program, params: &glium::DrawParameters) {
        target.draw(&self.vertices, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program, &uniform!{ model: model, view: view, perspective: perspective,
            u_light: u_light, diffuse_tex: &self.diffuse_map, normal_tex: &self.normal_map}, params).unwrap();
    }
}

/// Constructs vertices from object file
/// takes   display as glium::Display
///         file path as str
/// return object vertices as glium::VertexBuffer<obj::TexturedVertex>
pub fn load_object_file(display: &glium::Display, object_file_path: &str) -> glium::VertexBuffer<TexturedVertex> {
    let buffer = load_bytes(object_file_path);
    let obj: Obj<TexturedVertex> = load_obj(&buffer[..]).unwrap();
    glium::VertexBuffer::new(display, &obj.vertices).unwrap()
}
/// Reads in bytes from a file
/// takes   file path as str
/// returns file contents as vec<u8>
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
/// Constructs diffues texture map from file
/// takes   display as glium::Display
///         image format as image::ImageFormat
///         file path as str
/// returns opengl texture as glium::texture::SrgbTexture2d
pub fn load_diffuse_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::SrgbTexture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(display, raw_image).unwrap()
}
/// Constructs normal texture map from file
/// takes   display as glium::Display
///         image format as image::ImageFormat
///         file path as str
/// returns opengl texture as glium::texture::Texture2d
pub fn load_normal_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::Texture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, raw_image).unwrap()
}

/// Constructs opengl program from shader files
/// takes   display as glium::Display
///         vertex file path as a str
///         fragment file path as a str
/// returns opengl program as glium::Program
pub fn build_program(display: &glium::Display, vertex_shader_file_path: &str, fragment_shader_file_path: &str) -> glium::Program {
    let vertex_shader_bytes = load_bytes(vertex_shader_file_path);
    let vertex_shader_src = String::from_utf8_lossy(&vertex_shader_bytes);
    let fragment_shader_bytes = load_bytes(fragment_shader_file_path);
    let fragment_shader_src = String::from_utf8_lossy(&fragment_shader_bytes);
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}

/// Constructs perspective transfromation matrix
/// takes   drawing surface as glium::Frame
/// returns perspective transformation as [[f32;4];4]
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

/// Constructs camera view transformation matrix
/// takes   position vector as [f32; 3],
///         direction vector as [f32; 3],
///         up vector as [f32; 3]
/// returns veiew transformation as [[f32; 4]; 4]
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