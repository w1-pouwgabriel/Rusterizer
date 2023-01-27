extern crate minifb;
use glam::Vec2;

use triangle::Triangle;
use window::Window;


mod triangle;
mod window;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub fn to_argb32(a: u8, r: u8, g: u8, b: u8) -> u32{
    let mut color:u32 = a as u32;
    color = (color << 8) + r as u32;
    color = (color << 8) + g as u32;
    color = (color << 8) + b as u32;
    
    color
}

//clockwise
pub fn edge_function(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

//num items in a row
pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}

pub fn scan_line() -> u32{
    50u32
}

pub fn aabb_check(min: Vec2, max: Vec2, point: Vec2) -> bool
{
    if point.x > min.x && point.y > min.y{
        if point.y < max.y && point.y < max.y{
            return true;
        }
    }

    false
}

fn main() {
    let mut window = Window::new("Rusterizer - ESC to exit".to_string(), WIDTH, HEIGHT);

    let triangle = Triangle::new(glam::vec2(270.0, 150.0), glam::vec2(370.0, 200.0), glam::vec2(480.0, 150.0));

    // Limit to max ~60 fps update rate
    window.limit_fps(Some(60));

    while !window.should_close() {

        for i in 0..window.frame_buffer().get_data().len(){
            let coords = index_to_coords(i, WIDTH);
            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            if aabb_check(triangle.min, triangle.max, coords) {
                let m2 = edge_function(coords, triangle.v0, triangle.v1);

                let m0 = edge_function(coords, triangle.v1, triangle.v2);
                let m1 = edge_function(coords, triangle.v2, triangle.v0);
                
                // if m0 & m1 & m2 >= 0 we are inside the triangle
                if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
                    window.frame_buffer().set_pixel(coords.x as usize, coords.y as usize, to_argb32(
                        255,
                        255.0 as u8,
                        80.0 as u8,
                        80.0 as u8,
                    ));
                }else{
                    window.frame_buffer().set_pixel(coords.x as usize, coords.y as usize, to_argb32(
                        255,
                        0.0 as u8,
                        0.0 as u8,
                        0.0 as u8,
                    ));
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