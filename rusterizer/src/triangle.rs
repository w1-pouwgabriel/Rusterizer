pub struct Triangle{
    pub v0: glam::Vec2,
    pub v1: glam::Vec2,
    pub v2: glam::Vec2,

    pub min: glam::Vec2, // Bounding box
    pub max: glam::Vec2
}

impl Triangle{
    pub fn new(new_v0: glam::Vec2, new_v1: glam::Vec2, new_v2: glam::Vec2) -> Triangle {
        Triangle {
            v0: new_v0,
            v1: new_v1,
            v2: new_v2,
            min: new_v0.min(new_v1.min(new_v2)),
            max: new_v0.max(new_v1.max(new_v2))
        }
    }
}