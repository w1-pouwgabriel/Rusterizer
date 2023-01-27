extern crate minifb;

use glam::{Vec2, Vec3};

use triangle::Triangle;
use window::Window;
use utils::*;
use vertex::Vertex;

mod triangle;
mod window;
mod utils;
mod vertex;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let m0 = edge_function(point, v1, v2);
    let m1 = edge_function(point, v2, v0);
    let m2 = edge_function(point, v0, v1);
    // instead of 3 divisions we can do 1/area *
    let a = 1.0 / area;
    
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);

    let v0 = Vertex {
        position: glam::vec2(150.0, 150.0),
        color: glam::vec3(1.0, 0.0, 0.0),
    };
    let v1 = Vertex {
        position: glam::vec2(370.0, 300.0),
        color: glam::vec3(0.0, 1.0, 0.0),
    };
    let v2 = Vertex {
        position: glam::vec2(480.0, 150.0),
        color: glam::vec3(0.0, 0.0, 1.0),
    };
    let triangle = Triangle::new(v0.position, v1.position, v2.position);

    println!("test");

    // Limit to max ~60 fps update rate
    window.limit_fps(Some(60));

    while !window.should_close() {

        for i in 0..window.frame_buffer().get_data().len(){
            let coords = index_to_coords(i, WIDTH);
            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            if aabb_check(triangle.min, triangle.max, coords) {
                // Triangle area
                let area = edge_function(triangle.v0, triangle.v1, triangle.v2);
                // subtriangles area / triangle area
                if let Some(bary) = barycentric_coordinates(coords, triangle.v0, triangle.v1, triangle.v2, area){

                    let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;

                    window.frame_buffer().set_pixel(coords.x as usize, coords.y as usize, from_vec_to_argb32(color));
                }
            } 
            else{
                window.frame_buffer().set_pixel(coords.x as usize, coords.y as usize, to_argb32(
                    255,
                    0.0 as u8,
                    0.0 as u8,
                    0.0 as u8,
                ));
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.display();
    }
}