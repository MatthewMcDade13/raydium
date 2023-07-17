use sdl2::{render::Texture, rect::Rect};

use crate::{ray::{Ray, Hittable, HitRecord}, vec::Vec3, render::Drawable, math::IOResult};
use std::f32;


#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        let p = f32::consts::PI;
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

pub struct SdlTexture<'a> {
    texture: Texture<'a>,
    pub rect: Rect,
}

impl<'a> SdlTexture<'a> {
    pub fn new(texture: Texture<'a>, rect: Rect) -> Self {
        Self { texture, rect }
    }

    pub fn copy(&mut self, bytes: &[u8], pitch: usize) -> IOResult<()> {
        if let Err(e) = self.texture.update(None, bytes, pitch) {
            Err(format!("SdlTexture::copy -- Failed to update texture: {:?}", e).into())
        } else {
            Ok(())
        }
    }
}

impl<'a> Drawable for SdlTexture<'a> {
    fn draw(&self, renderer: &crate::render::Renderer) {
        renderer.draw_texture(&self.texture, &self.rect);
    }
}