use glam::{Vec3Swizzles, Vec2};
use crate::{vertex, utils::*, window::{FrameBuffer}, texture::Texture};

pub struct Triangle{
    pub v0: vertex::Vertex,
    pub v1: vertex::Vertex,
    pub v2: vertex::Vertex,

    pub min: glam::Vec3, // Bounding box
    pub max: glam::Vec3
}

impl Triangle{
    pub fn new(new_v0: vertex::Vertex, new_v1: vertex::Vertex, new_v2: vertex::Vertex) -> Triangle {
        Triangle {
            v0: new_v0,
            v1: new_v1,
            v2: new_v2,
            min: new_v0.position.min(new_v1.position.min(new_v2.position)),
            max: new_v0.position.max(new_v1.position.max(new_v2.position))
        }
    }

    pub fn raster_triangle(
        &self, 
        texture: &Texture,
        buffer: &mut FrameBuffer, 
        depth_buffer: &mut Vec<f32>,
        viewport_size: Vec2
    ){
        let width = buffer.width();
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = index_to_coords(i, width);
            // shadowing a variable
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

            if aabb_check(self.min.xy(), self.max.xy(), coords) {
                let area = edge_function(self.v0.position.xy(), self.v1.position.xy(), self.v2.position.xy());
        
                if let Some(bary) =
                barycentric_coordinates(coords, self.v0.position.xy(), self.v1.position.xy(), self.v2.position.xy(), area)
                {
                    let depth = bary.x * self.v0.position.z + bary.y * self.v1.position.z + bary.z * self.v2.position.z;
                    if depth < depth_buffer[i] {
                        depth_buffer[i] = depth;

                        let tex_coords = bary.x * self.v0.uv + bary.y * self.v1.uv + bary.z * self.v2.uv;
                        let color = texture.argb_at_uv(tex_coords.x, tex_coords.y);

                        *pixel = color;
                    };
                }
            }
        }
    }
}