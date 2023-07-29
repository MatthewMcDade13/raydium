use crate::{
    ray::{HitRecord, Ray},
    vec::Vec3,
};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
}
unsafe impl Sync for Vec3 {}
unsafe impl Sync for Lambertian {}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub const fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let scatter_dir = {
            let mut sd = hit.normal + Vec3::new_rand_unit_vector();
            if sd.is_near_zero() {
                sd = hit.normal
            }
            sd
        };
        Some(ScatterResult {
            scattered: Ray::new(hit.point, scatter_dir),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub const fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    #[inline]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n.mul_scalar(2.0 * v.dot(&n))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = Self::reflect(ray.direction.normalize(), hit.normal);
        let scattered = Ray::new(hit.point, reflected);
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
