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

    pub fn color<T: Hittable>(&self, world: HitList<T>) -> Vec3 {
        if let Some(hit) = world.hit(self, 0.0, f64::INFINITY) {
            (hit.normal + Color::WHITE).mul_scalar(0.5)
        } else {
            let dir = self.direction.normalize();
            let t = 0.5 * (dir.y() + 1.0);
            Vec3::lerp(&Color::WHITE, &Vec3(0.5, 0.7, 1.0), t)
        }
    }
}

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
        }
    }

    pub fn from_ray(ray: &Ray, point: Vec3, normal: Vec3, t: f64) -> Self {
        let mut s = Self::new(point, normal, t);
        s.set_face_normal(ray, normal);
        s
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
pub struct HitList<T>(pub Vec<T>) where T: Hittable;
impl<T> HitList<T> where T: Hittable {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Hittable for HitList<T> where T: Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;

        self.0.iter().fold(None, |acc, curr| {
            if let Some(hit) = curr.hit(ray, t_min, closest) {
                closest = hit.t;
                Some(hit)
            } else {
                acc
            }
        })

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
