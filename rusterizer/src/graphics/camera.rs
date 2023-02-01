use crate::transform::Transform;

use glam::Mat4;

pub struct Camera {
    pub frustum_near: f32,
    pub frustum_far: f32,
    pub fov: f32, // in radians
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            frustum_near: 0.1,
            frustum_far: 100.0,
            fov: std::f32::consts::PI / 4.0,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY,
            speed: 10.0,
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov,
            self.aspect_ratio,
            self.frustum_near,
            self.frustum_far,
        )
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }

    pub fn update_settings(
        &mut self,
        new_frustum_near: Option<f32>,
        new_frustum_far: Option<f32>,
        new_fov: Option<f32>, // in radian
        new_aspect_ratio: Option<f32>,
        new_transform: Option<Transform>,
        new_speed: Option<f32>,
    ) {
        self.frustum_near = new_frustum_near.unwrap_or_else(|| self.frustum_near);
        self.frustum_far = new_frustum_far.unwrap_or_else(|| self.frustum_far);
        self.fov = new_fov.unwrap_or_else(|| self.fov); // in radia = fov: , // in radian
        self.aspect_ratio = new_aspect_ratio.unwrap_or_else(|| self.aspect_ratio);
        self.transform = new_transform.unwrap_or_else(|| self.transform);
        self.speed = new_speed.unwrap_or_else(|| self.speed);
    }
}
