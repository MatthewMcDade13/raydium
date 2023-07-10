use crate::{vec::{Vec3, Color}, world::Camera};



pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction.mul_scalar(t)
    }

    pub fn color(&self) -> Vec3 {
        let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &self);
        if t > 0.0 {
            let dir = self.at(t) - Vec3(0.0, 0.0, -1.0);
            let N = dir.normalize();
            Vec3(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0).mul_scalar(0.5)
        } else {
            let unit_dir = self.direction.normalize();
            let tt = 0.5 * (unit_dir.y() + 1.0);
            Vec3::lerp(&Color::WHITE, &Vec3(0.5, 0.7, 1.0), tt)
        }
        
    }
}


fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.len_sq();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.len_sq() - radius * radius;
    let discriminant = half_b*half_b - a*c; 
   
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
