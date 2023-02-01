use glam::Vec2;

pub struct BoundingBox2D {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoundingBox2D {
    pub fn get_triangle_bounding_box_2d(positions: &[Vec2; 3]) -> BoundingBox2D {
        let left = positions[0].x.min(positions[1].x).min(positions[2].x);
        let right = positions[0].x.max(positions[1].x).max(positions[2].x);
        let top = positions[0].y.min(positions[1].y).min(positions[2].y);
        let bottom = positions[0].y.max(positions[1].y).max(positions[2].y);

        BoundingBox2D {
            left,
            right,
            top,
            bottom,
        }
    }
}
