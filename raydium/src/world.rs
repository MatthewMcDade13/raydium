use std::ops::Neg;

use rand::{Rng, SeedableRng};

use crate::{math::radians, ray::Ray, render::defaults, vec::Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    origin: Vec3,
    pixel00: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    time: (f64, f64),
    info: CameraInfo,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraInfo {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vert_up: Vec3,
    pub vert_fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub time: (f64, f64),
    pub max_scatter_depth: u32,
    pub samples_per_pixel: u32,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            look_from: Vec3(0., 0., -1.),
            look_at: Vec3::zero(),
            vert_up: Vec3(0., 1., 0.),
            vert_fov: 40.,
            aspect_ratio: 1.,
            aperture: 0.,
            focus_dist: 10.,
            time: (0., 0.),
            max_scatter_depth: defaults::MAX_SCATTER_DEPTH,
            samples_per_pixel: defaults::NUM_SAMPLES,
        }
    }
}

impl Camera {
    pub const fn origin(&self) -> Vec3 {
        self.origin
    }
    pub const fn pixel00(&self) -> Vec3 {
        self.pixel00
    }
    pub const fn lens_radius(&self) -> f64 {
        self.lens_radius
    }
    pub const fn max_scatter_depth(&self) -> u32 {
        self.info.max_scatter_depth
    }
    pub const fn samples_per_pixel(&self) -> u32 {
        self.info.samples_per_pixel
    }

    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vert_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time: Option<(f64, f64)>,
        max_scatter_depth: u32,
        samples_per_pixel: u32,
    ) -> Self {
        Self::with_info(&CameraInfo {
            aspect_ratio,
            look_from,
            look_at,
            vert_up,
            vert_fov,
            aperture,
            time: time.unwrap_or_default(),
            focus_dist,
            max_scatter_depth,
            samples_per_pixel,
        })
    }
    pub fn with_info(info: &CameraInfo) -> Self {
        let CameraInfo {
            look_from,
            look_at,
            vert_up,
            vert_fov,
            aspect_ratio,
            aperture,
            focus_dist,
            time,
            ..
        } = info.clone();

        let theta = radians(vert_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&vert_up, &w).normalize();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = u.mul_scalar(viewport_w).mul_scalar(focus_dist);
        let vertical = v.mul_scalar(viewport_h).mul_scalar(focus_dist);
        let pixel00 = origin
            - horizontal.div_scalar(2.0)
            - vertical.div_scalar(2.0)
            - w.mul_scalar(focus_dist);

        let lens_radius = aperture / 2.0;
        Self {
            origin,
            pixel00,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time,
            info: info.clone(),
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::new_rand_in_unit_disk().mul_scalar(self.lens_radius);
        let offset = self.u.mul_scalar(rd.x()) + self.v.mul_scalar(rd.y());
        let direction = self.pixel00 + self.horizontal.mul_scalar(u) + self.vertical.mul_scalar(v)
            - self.origin
            - offset;
        Ray::new(self.origin + offset, direction)
    }
}
