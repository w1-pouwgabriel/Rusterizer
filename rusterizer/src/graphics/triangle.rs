use glam::{Vec2, Mat4, Vec4, Vec3Swizzles, Vec4Swizzles};
use crate::{vertex, utils::*, window::{FrameBuffer}, texture::Texture, boundingbox2D::BoundingBox2D};

pub struct Triangle{
    pub v0: vertex::Vertex,
    pub v1: vertex::Vertex,
    pub v2: vertex::Vertex,
}

impl Triangle{
    pub fn new(new_v0: vertex::Vertex, new_v1: vertex::Vertex, new_v2: vertex::Vertex) -> Triangle {
        Triangle {
            v0: new_v0,
            v1: new_v1,
            v2: new_v2,
        }
    }

    pub fn raster_triangle(
        &self, 
        mvp: &Mat4,
        texture: Option<&Texture>,
        buffer: &mut FrameBuffer,
        depth_buffer: &mut [f32],
        viewport_size: Vec2
    ){
        //Clip space
        let clip0 = *mvp * Vec4::from((self.v0.position, 1.0));
        let clip1 = *mvp * Vec4::from((self.v1.position, 1.0));
        let clip2 = *mvp * Vec4::from((self.v2.position, 1.0));

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

        if let Some(bounding_box) = self.triangle_screen_bounding_box(&[sc0, sc1, sc2], viewport_size){

            for y in (bounding_box.top as usize)..=bounding_box.bottom as usize {
                for x in (bounding_box.left as usize)..=bounding_box.right as usize {

                    buffer.set_pixel(x, y, to_argb32(
                        255,
                        (255.0) as u8,
                        (255.0) as u8,
                        (255.0) as u8,
                    ));

                    let coords = glam::vec2(x as f32, y as f32) + 0.5;
                    let pixel_id = coords_to_index(x, y, viewport_size.x as usize);
                    let area = edge_function(sc0, sc1, sc2);

                    if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                        let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                        let depth = correction;
                        let correction = 1.0 / correction; // 1/(1/z) = z
                        //let depth = bary.x * self.v0.position.z + bary.y * self.v1.position.z + bary.z * self.v2.position.z;
                        if depth < depth_buffer[pixel_id] {
                            depth_buffer[pixel_id] = depth;
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
                            
                            buffer.set_pixel(x, y, color);
                        }
                    }
                }
            }
        }
    }
    
    pub fn triangle_screen_bounding_box(
        &self,
        positions_screen_space: &[Vec2; 3],
        viewport_size: Vec2,
    ) -> Option<BoundingBox2D> {
        let bb = BoundingBox2D::get_triangle_bounding_box_2d(positions_screen_space);

        if bb.left >= viewport_size.x || bb.right < 0.0 || bb.bottom >= viewport_size.y || bb.top < 0.0
        {
            None
        } else {
            let left = bb.left.max(0.0);
            let right = bb.right.min(viewport_size.x - 1.0);
            let bottom = bb.bottom.max(0.0);
            let top = bb.top.min(viewport_size.y - 1.0);

            Some(BoundingBox2D {
                left,
                right,
                top,
                bottom,
            })
        }
    }
}