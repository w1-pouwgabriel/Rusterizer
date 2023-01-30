extern crate minifb;

#[path = "resources/texture.rs"] pub mod texture;
#[path = "math/transform.rs"] mod transform;
mod window;
mod utils;
#[path = "graphics/triangle.rs"] mod triangle;
#[path = "graphics/vertex.rs"] mod vertex;
#[path = "graphics/mesh.rs"] mod mesh;

use window::Window;
use vertex::Vertex;
use texture::Texture;
use mesh::Mesh;

use std::path::Path;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);
    let mut window_size = glam::vec2(window.frame_buffer().width() as f32, window.frame_buffer().height() as f32);

    let mut z_buffer;                                               //Maybe add this variable to some sort of graphics pipeline???
    let texture = Texture::load(Path::new("assets/Hackerman.jpg"));  //TODO: Add some resource manager

    let v0 = Vertex {
        position: glam::vec3(100.0, 100.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v1 = Vertex {
        position: glam::vec3(100.0, 400.0, 0.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v2 = Vertex {
        position: glam::vec3(400.0, 400.0, 0.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(1.0, 1.0),
    };
    let v3 = Vertex {
        position: glam::vec3(400.0, 100.0, 0.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };

    let mut triangles = vec![glam::uvec3(0, 1, 2), glam::uvec3(0, 2, 3)];
    let mut vertices = vec![v0, v1, v2, v3];

    let mut mesh = Mesh::new();
    mesh.add_section_from_vertices(&mut triangles, &mut vertices);

    // Limit to max ~60 fps update rate
    window.limit_fps(Some(60));

    while !window.should_close() {

        //Clear z_buffer
        z_buffer = vec![f32::INFINITY; window.frame_buffer().width() * window.frame_buffer().height()];
        window_size = glam::vec2(window.frame_buffer().width() as f32, window.frame_buffer().height() as f32);

        mesh.raster_mesh(&texture, window.frame_buffer(), &mut z_buffer, window_size);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.display();
    }
}