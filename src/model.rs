use glium::{
    Surface,
    uniform
};
use std::io::Cursor;
use obj::{
    load_obj, Obj,
    TexturedVertex
};
use cgmath::Matrix4;

use crate::etc::load_bytes;

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
    ///         translation transformation as [[f32;4]; 4]
    ///         rotation transformation as [[f32;4]; 4]
    ///         scale transformation as [[f32;4]; 4]
    ///         view transformation as [[f32;4]; 4]
    ///         perspective transformation as [[f32;4]; 4]
    ///         light color as [f32; 3]
    ///         opengl program as glium::Program
    pub fn draw(&self,target: &mut glium::Frame, translation: [f32;3], rotation: [[f32;4];4], scaling: [f32;3],
                view: [[f32;4]; 4], perspective: [[f32;4]; 4], u_light: [f32; 3], program: &glium::Program, params: &glium::DrawParameters) {
        let t = Matrix4::from_translation(cgmath::Vector3::new(translation[0],translation[1],translation[2]));
        let r = Matrix4::new(
            rotation[0][0], rotation[0][1], rotation[0][2], rotation[0][3],
            rotation[1][0], rotation[1][1], rotation[1][2], rotation[1][3],
            rotation[2][0], rotation[2][1], rotation[2][2], rotation[2][3],
            rotation[3][0], rotation[2][1], rotation[1][2], rotation[3][3],
        );
        let s = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
        let m = t * r * s;
        let model: [[f32;4];4] = m.into();
        target.draw(&self.vertices,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &program,
            &uniform!{
                model: model,
                view: view,
                perspective: perspective,
                u_light: u_light,
                diffuse_tex: &self.diffuse_map,
                normal_tex: &self.normal_map
            },
            params).unwrap();
    }
}
/// Constructs vector of vertices inlcuding normals and uv
/// takes   disply as glium::Display
///         file path as str
/// returns vertices as glium::VertexBuffer<obj::TexturedVertex>
fn load_object_file(display: &glium::Display, object_file_path: &str) -> glium::VertexBuffer<TexturedVertex> {
    let buffer = load_bytes(object_file_path);
    let obj: Obj<TexturedVertex> = load_obj(&buffer[..]).unwrap();
    glium::VertexBuffer::new(display, &obj.vertices).unwrap()
}

/// Constructs diffues texture map from file
/// takes   display as glium::Display
///         image format as image::ImageFormat
///         file path as str
/// returns opengl texture as glium::texture::SrgbTexture2d
fn load_diffuse_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::SrgbTexture2d {
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
fn load_normal_map(display: &glium::Display, format: image::ImageFormat, file_path: &str) -> glium::texture::Texture2d {
    let buffer = load_bytes(file_path);
    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, raw_image).unwrap()
}