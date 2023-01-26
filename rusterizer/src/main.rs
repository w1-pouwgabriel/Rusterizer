extern crate minifb;
use framebuffer::FrameBuffer;
use minifb::{Key, Window, WindowOptions};
use glam::Vec2;

mod framebuffer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32{
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


fn main() {
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Rusterizer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let triangle = [
        glam::vec2(0.0, 0.0),
        glam::vec2((WIDTH / 2) as f32, 360.0),
        glam::vec2(WIDTH as f32, 0.0),
    ];

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..frame_buffer.get_data().len(){
            let coords = index_to_coords(i, WIDTH);
            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            let m2 = edge_function(coords, triangle[0], triangle[1]);

            let m0 = edge_function(coords, triangle[1], triangle[2]);
            let m1 = edge_function(coords, triangle[2], triangle[0]);
            
            // if m0 & m1 & m2 >= 0 we are inside the triangle
            if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
                frame_buffer.set_pixel(coords.x as usize, coords.y as usize, to_argb8(
                    255,
                    255.0 as u8,
                    80.0 as u8,
                    80.0 as u8,
                ));
            }else{
                frame_buffer.set_pixel(coords.x as usize, coords.y as usize, to_argb8(
                    255,
                    0.0 as u8,
                    0.0 as u8,
                    0.0 as u8,
                ));
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&frame_buffer.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}