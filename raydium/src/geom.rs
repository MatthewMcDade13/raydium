


use std::{f32, rc::Rc, sync::Arc};

use crate::{ray::{HitRecord, Ray, Hittable}, vec::Vec3, material::Material};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>, center: Vec3, radius: f64) -> Self {
        let p = f32::consts::PI;
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center).div_scalar(self.radius);
        let mut hitrec = HitRecord::new(point, outward_normal, t, self.material.clone());
        hitrec.set_face_normal(ray, outward_normal);
        Some(hitrec)
    }
}

