use glium::{
    IndexBuffer,
    Surface,
    uniform,
    VertexBuffer
};
use std::{
    collections::HashMap,
    fs,
    io::Cursor,
    string::String
};
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
pub struct GraphicTexture {
    diffuse_tex: glium::texture::SrgbTexture2d,
    normals_map: glium::texture::Texture2d
}
impl GraphicTexture {
    pub fn from_path(display: &glium::Display, diffuse_file_path: &str, normal_file_path: &str) -> GraphicTexture {
        GraphicTexture {
            diffuse_tex: GraphicTexture::load_diffuse_tex(display, diffuse_file_path),
            normals_map: GraphicTexture::load_normals_map(display, normal_file_path)
        }
    }
    fn get_image_format(file_path: &str) -> image::ImageFormat {
        let path = std::path::Path::new(file_path);
        let extension = path.extension().and_then(std::ffi::OsStr::to_str);
        match extension.unwrap() {
            "png" => image::ImageFormat::Png,
            "jpg" => image::ImageFormat::Jpeg,
            "bmp" => image::ImageFormat::Bmp,
            "gif" => image::ImageFormat::Gif,
            _ => panic!("Could not recognize the image format of {}", path.display())
        }
    }
    fn load_diffuse_tex(display: &glium::Display, file_path: &str) -> glium::texture::SrgbTexture2d {
        let format = GraphicTexture::get_image_format(file_path);
        let buffer = load_bytes(file_path);
        let image = image::load(Cursor::new(buffer),
                                format).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        glium::texture::SrgbTexture2d::new(display, raw_image).unwrap()
    }
    fn load_normals_map(display: &glium::Display, file_path: &str) -> glium::texture::Texture2d {
        let format = GraphicTexture::get_image_format(file_path);
        let buffer = load_bytes(file_path);
        let image = image::load(Cursor::new(buffer),
                                format).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        glium::texture::Texture2d::new(display, raw_image).unwrap()
    }
}

#[derive(Debug)]
pub struct GraphicObject {
    vertices: VertexBuffer<TexturedVertex>,
    indices: IndexBuffer<u16>
}
impl GraphicObject {
    pub fn from_path(display: &glium::Display, object_file_path: &str) -> GraphicObject {
        let (vertices, indices) = GraphicObject::load_object_file(&display, object_file_path);
        GraphicObject {
            vertices,
            indices
        }
    }

    fn load_object_file(display: &glium::Display, object_file_path: &str) -> (VertexBuffer<TexturedVertex>, IndexBuffer<u16>) {
        let buffer = load_bytes(object_file_path);
        let obj: Obj<TexturedVertex> = load_obj(&buffer[..]).unwrap();
        let vertices = obj.vertex_buffer(display).unwrap();
        let indices = obj.index_buffer(display).unwrap();
        (vertices, indices)
    }
}

#[derive(Debug)]
pub struct GraphicLibrary {
    obj_dict: HashMap<String, GraphicObject>,
    tex_dict: HashMap<String, GraphicTexture>
}

impl GraphicLibrary {
    pub fn new() -> GraphicLibrary {
        GraphicLibrary {
            obj_dict: HashMap::new(),
            tex_dict: HashMap::new()
        }
    }
    pub fn load(display: &glium::Display, file_path: &str) -> GraphicLibrary {
        let mut library = GraphicLibrary::new();
        library.load_path(display, file_path);
        library
    }
    pub fn load_json(&mut self, display: &glium::Display, json_file_path: &str) {
        let buffer: serde_json::Value = serde_json::from_slice(&load_bytes(json_file_path)).unwrap();
        self.obj_dict.insert(
            buffer["object_key"].to_string().trim_matches('"').to_string(),
            GraphicObject::from_path(display, buffer["object_file_path"].to_string().trim_matches('"'))
        );
        self.tex_dict.insert(
            buffer["texture_key"].to_string().trim_matches('"').to_string(),
            GraphicTexture::from_path(display, buffer["diffuse_file_path"].to_string().trim_matches('"'),
                                    buffer["normal_file_path"].to_string().trim_matches('"'))
        );
    }
    pub fn load_path(&mut self, display:&glium::Display, file_path: &str) {
        let paths = fs::read_dir(file_path).unwrap();
        for path in paths {
            let file = path.unwrap().path();
            let extension = file.extension().and_then(std::ffi::OsStr::to_str).unwrap();
            match extension {
                "json" => self.load_json(display, file.to_str().unwrap()),
                _=> continue
            }
        }

    }
    pub fn get_obj(&self, obj_key: &str) -> &GraphicObject {
        match self.obj_dict.get(obj_key) {
            Some(object) => object,
            None => panic!("Could not find object key '{}' in album", obj_key)
        }
    }
    pub fn get_tex(&self, tex_key: &str) -> &GraphicTexture {
        match self.tex_dict.get(tex_key) {
            Some(texture) => texture,
            None => panic!("Could not find texture key '{}' in album", tex_key)
        }
    }

    pub fn draw(&self, target: &mut glium::Frame, object_key: &str, texture_key: &str, translation: [f32;3], rotation: [f32;4], scaling: [f32;3],
            view: [[f32;4]; 4], perspective: [[f32;4]; 4], u_light: [f32; 3], program: &glium::Program, params: &glium::DrawParameters) {
    let t = Matrix4::from_translation(cgmath::Vector3::new(translation[0],translation[1],translation[2]));
    let r = Matrix4::from(Quaternion::from(rotation));
    let s = Matrix4::from_nonuniform_scale(-scaling[0], scaling[1], scaling[2]);
    let m = t * r * s;
    let model: [[f32;4];4] = m.into();
    let object = self.get_obj(object_key);
    let texture = self.get_tex(texture_key);
    target.draw(&object.vertices,
        &object.indices,
        &program,
        &uniform!{
            model: model,
            view: view,
            perspective: perspective,
            u_light: u_light,
            diffuse_tex: &texture.diffuse_tex,
            normals_tex: &texture.normals_map
        },
        params).unwrap();
    }
}

#[derive(Debug,Clone)]
pub struct Graphic {
    scale: [f32; 3],
    object_key: String,
    texture_key: String
}
#[allow(dead_code)]
impl Graphic {
    pub fn new(
        scale: [f32; 3],
        object_key: String,
        texture_key: String
    ) -> Graphic {
        Graphic {
            scale,
            object_key,
            texture_key
        }
    }
    pub fn draw(&self, target: &mut glium::Frame, library: &GraphicLibrary, position: [f32;3], orientation: [f32;4], view: [[f32;4]; 4], perspective: [[f32;4]; 4],
                u_light: [f32; 3],program: &glium::Program, params: &glium::DrawParameters) {
        library.draw(target, &self.object_key, &self.texture_key, position, orientation, self.scale,
                   view, perspective, u_light, program, params);
    }
}