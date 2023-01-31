extern crate minifb;

#[path = "resources/texture.rs"] pub mod texture;
#[path = "math/transform.rs"] mod transform;
mod window;
mod utils;
#[path = "graphics/triangle.rs"] mod triangle;
#[path = "graphics/vertex.rs"] mod vertex;
#[path = "graphics/mesh.rs"] mod mesh;
#[path = "graphics/camera.rs"] mod camera;

use transform::Transform;
use window::Window;
use vertex::Vertex;
use texture::Texture;
use mesh::Mesh;
use camera::Camera;

use std::path::{Path, self};

const WIDTH: usize = 720;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);

    let mut z_buffer;                                               //Maybe add this variable to some sort of graphics pipeline???
    let texture = Texture::load(Path::new("assets/uv_mapper.jpg"));  //TODO: Add some resource manager

    let v0 = Vertex {
        position: glam::vec3(-2.0, -2.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v1 = Vertex {
        position: glam::vec3(-2.0, 2.0, 0.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v2 = Vertex {
        position: glam::vec3(2.0, 2.0, 0.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        position: glam::vec3(2.0, -2.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
    let vertices = vec![v0, v1, v2, v3];

    let mesh = Mesh::from_vertices(&triangles, &vertices);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_far: 100.0,
        ..Default::default()
    };

    // Limit to max ~60 fps update rate
    window.limit_fps(Some(60));

    while !window.should_close() {

        let (width, height) = window.frame_buffer().size();
        //Clear buffer
        z_buffer = vec![f32::INFINITY; width * height];
        let window_size = glam::vec2(width as f32, height as f32);

        mesh.raster_mesh(        
            &Transform::IDENTITY.local(),
            &camera.view(),
            &camera.projection(),
            Some(&texture),
            window.frame_buffer(),
            &mut z_buffer,
            window_size);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.display();
    }
}