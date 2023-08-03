use crate::{math::radians, ray::Ray, vec::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub viewport_h: f64,
    pub viewport_w: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub bottom_left: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vert_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = radians(vert_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&vert_up, &w);
        let v = Vec3::cross(&w, &u);

        let focal_length = 1.0;

        let origin = look_from;
        let horizontal = u.mul_scalar(viewport_w);
        let vertical = v.mul_scalar(viewport_h);
        let bottom_left = origin - (horizontal.div_scalar(2.0)) - (vertical.div_scalar(2.0)) - w;
        Self {
            viewport_h,
            viewport_w,
            focal_length,
            origin,
            horizontal,
            vertical,
            bottom_left,
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.bottom_left + self.horizontal.mul_scalar(u) + self.vertical.mul_scalar(v)
                - self.origin;

        Ray::new(self.origin, direction)
    }
}
