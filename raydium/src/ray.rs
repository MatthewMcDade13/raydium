use crate::{vec::Vec3, world::Camera};



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
        let unit_dir = self.direction.unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        
        Vec3(1.0, 1.0, 1.0).mul_scalar(1.0 - t)
            + Vec3(0.5, 0.7, 1.0).mul_scalar(t)
    }
}


