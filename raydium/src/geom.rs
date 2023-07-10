use crate::{ray::Ray, vec::Vec3};

#[derive(Debug, Clone)]
pub enum NormalFace {
    FrontOuter, BackInner
}


#[derive(Debug, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub normal_face: NormalFace
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64) -> Self {
        Self {
            point, normal, t,
            normal_face: NormalFace::FrontOuter
        }/*  */
    }

    pub fn new_out_norm(point: Vec3, normal: Vec3, t: f64) -> Self {
        let mut s = Self::new(point, normal, t);
        s.set_face_normal(ray::Ray::new(point, normal), outward_normal(&s));
        Self {
            point, normal, t,
            normal_face: NormalFace::BackInner
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let dprod = Vec3::dot(&ray.direction, &outward_normal);
        self.normal_face = if dprod < 0.0 { 
            NormalFace::FrontOuter 
        } else {
            NormalFace::BackInner
        };
        self.normal = match self.normal_face {
            NormalFace::FrontOuter => outward_normal,
            NormalFace::BackInner => -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { 
            point: Vec3::default(), 
            normal: Vec3::default(), 
            t: f64::default(), 
            normal_face: NormalFace::FrontOuter 
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}


impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center).div_scalar(self.radius);
        let mut hitrec = HitRecord::new(point, outward_normal, t);
        hitrec.set_face_normal(ray, outward_normal);
        Some(hitrec)        
    }
}