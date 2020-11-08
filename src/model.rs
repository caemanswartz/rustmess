use glium::{
    IndexBuffer,
    Surface,
    uniform,
    VertexBuffer
};
use std::io::Cursor;
use obj::{
    load_obj, Obj,
    TexturedVertex
};
use cgmath::{
    Quaternion,
    Matrix4
};

use crate::etc::load_bytes;

#[derive(Debug)]
pub struct Model {
    vertices: VertexBuffer<TexturedVertex>,
    indices: IndexBuffer<u16>,
    diffuse_map: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d
}
impl Model {
    pub fn new(vertices: VertexBuffer<TexturedVertex>, indices: IndexBuffer<u16>,
               diffuse_map: glium:: texture::SrgbTexture2d, normal_map: glium::texture::Texture2d) -> Model {
        Model {
            vertices,
            indices,
            diffuse_map,
            normal_map
        }
    }
    pub fn from_files(display: &glium::Display, object_file_path: &str, diffuse_file_path: &str, normal_file_path: &str) -> Model {
        let (vertices, indices) = load_object_file(&display, object_file_path);
        Model::new(
            vertices,
            indices,
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
    pub fn draw(&self,target: &mut glium::Frame, translation: [f32;3], rotation: [f32;4], scaling: [f32;3],
                view: [[f32;4]; 4], perspective: [[f32;4]; 4], u_light: [f32; 3], program: &glium::Program, params: &glium::DrawParameters) {
        let t = Matrix4::from_translation(cgmath::Vector3::new(translation[0],translation[1],translation[2]));
        let r = Matrix4::from(Quaternion::from(rotation));
        let s = Matrix4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
        let m = t * r * s;
        let model: [[f32;4];4] = m.into();
        target.draw(&self.vertices,
            &self.indices,
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
fn load_object_file(display: &glium::Display, object_file_path: &str) -> (VertexBuffer<TexturedVertex>, IndexBuffer<u16>) {
    let buffer = load_bytes(object_file_path);
    let obj: Obj<TexturedVertex> = load_obj(&buffer[..]).unwrap();
    let vertices = obj.vertex_buffer(display).unwrap();
    let indices = obj.index_buffer(display).unwrap();
    (vertices, indices)
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