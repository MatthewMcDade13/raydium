use crate::{vec::Vec3, ray::{Ray, HitList, Hittable}};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub viewport_h: f64,
    pub viewport_w: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub bottom_left: Vec3
}

impl Camera {

    pub fn new(aspect_ratio: f64, h: f64, focal_len: f64) -> Self {
        let viewport_w = aspect_ratio * h;
        let origin = Vec3::zero();
        let horizontal = Vec3(viewport_w, 0.0, 0.0);
        let vertical = Vec3(0.0, h, 0.0);
        let bottom_left = {
            origin
            - horizontal.div_scalar(2.0)
            - vertical.div_scalar(2.0)
            - Vec3(0.0, 0.0, focal_len)
        };
        Self {
            viewport_h: h,
            viewport_w,
            focal_length: focal_len,
            origin: Vec3(0.0, 0.0, 0.0),
            horizontal,
            vertical,
            bottom_left,            
        }
    }

    pub fn cast_ray_at(&self, u: f64, v: f64) -> Ray {
        let direction = self.bottom_left 
            + self.horizontal.mul_scalar(u)
            + self.vertical.mul_scalar(v)
            - self.origin;

        Ray::new(self.origin, direction)        
    }

}
