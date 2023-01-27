use glam::{Vec2};

pub fn to_argb32(a: u8, r: u8, g: u8, b: u8) -> u32{
    let mut color:u32 = a as u32;
    color = (color << 8) + r as u32;
    color = (color << 8) + g as u32;
    color = (color << 8) + b as u32;
    
    color
}

pub fn from_vec_to_argb32(color_vec: glam::Vec3) -> u32{
    let mut color:u32 = 255 as u32;
    color = (color << 8) + (color_vec.x * 255.99) as u32;
    color = (color << 8) + (color_vec.y * 255.99) as u32;
    color = (color << 8) + (color_vec.z * 255.99) as u32;
    
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

pub fn aabb_check(min: Vec2, max: Vec2, point: Vec2) -> bool
{
    if point.x > min.x && point.y > min.y{
        if point.y < max.y && point.y < max.y{
            return true;
        }
    }

    false
}