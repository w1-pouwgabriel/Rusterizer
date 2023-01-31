use glam::{Vec3Swizzles, Vec2, Mat4, Vec4};
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
        model: &Mat4,
        view: &Mat4,
        projection: &Mat4,
        texture: Option<&Texture>,
        buffer: &mut FrameBuffer,
        depth_buffer: &mut [f32],
        viewport_size: Vec2
    ){
        let mvp = *projection * *view * *model;

        let clip0 = mvp * Vec4::from((self.v0.position, 1.0));
        let clip1 = mvp * Vec4::from((self.v1.position, 1.0));
        let clip2 = mvp * Vec4::from((self.v2.position, 1.0));

        let rec0 = 1.0 / clip0.w;
        let rec1 = 1.0 / clip1.w;
        let rec2 = 1.0 / clip2.w;
    
        let uv0 = self.v0.uv * rec0;
        let uv1 = self.v1.uv * rec1;
        let uv2 = self.v2.uv * rec2;
    
        let color0 = self.v0.color * rec0;
        let color1 = self.v1.color * rec1;
        let color2 = self.v2.color * rec2;

        // This would be the output of the vertex shader (clip space)
        // then we perform perspective division to transform in ndc
        // now x,y,z componend of ndc are between -1 and 1
        let ndc0 = clip0 * rec0;
        let ndc1 = clip1 * rec1;
        let ndc2 = clip2 * rec2;

        // screeen coordinates remapped to window
        let sc0 = glam::vec2(
            map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc1 = glam::vec2(
            map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc2 = glam::vec2(
            map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
        );

        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = index_to_coords(i, viewport_size.x as usize);
            // center of the pixel
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

            let area = edge_function(sc0, sc1, sc2);

            if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                let correction = 1.0 / correction;
                let depth = bary.x * self.v0.position.z + bary.y * self.v1.position.z + bary.z * self.v2.position.z;
                if depth < depth_buffer[i] {
                    depth_buffer[i] = depth;
                    let color = bary.x * color0 + bary.y * color1 + bary.z * color2;
                    let color = color * correction;
                    let mut color = to_argb32(
                        255,
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    );
                    if let Some(tex) = texture {
                        let tex_coords = bary.x * uv0 + bary.y * uv1 + bary.z * uv2;
                        let tex_coords = tex_coords * correction;
                        color = tex.argb_at_uv(tex_coords.x, tex_coords.y);
                    }

                    *pixel = color;
                }
            }
        }
    }
}