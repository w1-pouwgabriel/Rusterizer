extern crate minifb;

mod texture;
mod triangle;
mod window;
mod utils;
mod vertex;

use triangle::Triangle;
use window::Window;
use vertex::Vertex;
use texture::Texture;
use utils::*;

use std::path::Path;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

#[cfg(test)]
mod tests {
    use crate::Vertex;
    use crate::utils::*;

    #[test]
    fn lerping() {
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

        let interpolated = lerp(v0, v1, 0.5);
        assert_eq!(interpolated.uv.y, 0.5);
    }
}

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);

    let mut z_buffer;

    let texture = Texture::load(Path::new("assets/Hackerman.jpg"));

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

    let triangle = Triangle::new(v0, v1, v2);
    let triangle2 = Triangle::new(v0, v2, v3);

    println!("interpolated vertex: {:?}", lerp(v0, v1, 0.5));

    // Limit to max ~60 fps update rate
    window.limit_fps(Some(60));

    while !window.should_close() {

        //Clear z_buffer
        z_buffer = vec![f32::INFINITY; window.frame_buffer().width() * window.frame_buffer().height()];

        triangle.raster_triangle(&texture, window.frame_buffer(), &mut z_buffer);
        triangle2.raster_triangle(&texture, window.frame_buffer(), &mut z_buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.display();
    }
}