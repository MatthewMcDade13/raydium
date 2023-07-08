use crate::vec::Vec3;


pub struct Camera {
    pub viewport_h: f64,
    pub viewport_w: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}
