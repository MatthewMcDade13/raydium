use std::ops::Neg;

use crate::{
    ray::{HitRecord, NormalFace, Ray},
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Vec3(1.0, 0.5, 1.0),
            fuzz: 0.0,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + Vec3::new_rand_unit_sphere().mul_scalar(self.fuzz),
        );
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

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self { ir: 0.0 }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Vec3::WHITE;
        let refraction_ratio = match hit.normal_face {
            NormalFace::FrontOuter => 1.0 / self.ir,
            NormalFace::BackInner => self.ir,
        };
        let unit_dir = ray.direction.normalize();
        let cos_theta = f64::min(Vec3::dot(&unit_dir.neg(), &hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            reflect(unit_dir, hit.normal)
        } else {
            refract(&unit_dir, &hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction);

        Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}

pub fn reflectance(cosine: f64, reflection_index: f64) -> f64 {
    // Schlick's approximation for reflectance.
    let r0 = {
        let r = (1.0 - reflection_index) / (1.0 + reflection_index);
        r * r
    };
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n.mul_scalar(2.0 * v.dot(&n))
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(Vec3::dot(&uv.neg(), &n), 1.0);
    let r_out_perp = (*uv + n.mul_scalar(cos_theta)).mul_scalar(etai_over_etat);
    let r_out_parallel = n.mul_scalar(-f64::sqrt(f64::abs(1.0 - r_out_perp.len_sq()))); //n.mul_scalar(-(1.0 - r_out_perp.len_sq()).abs().sqrt());
    r_out_perp + r_out_parallel
}
