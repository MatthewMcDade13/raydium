use rand::{rngs::ThreadRng, Rng};

pub type IOResult<T> = Result<T, Box<dyn std::error::Error>>;

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932384626433832795;
pub const HALF_PI: f64 = 1.5707963267948966192313216916398;
pub const DEG_TO_RAD: f64 = 0.017453292519943295769236907684886;
pub const RAD_TO_DEG: f64 = 57.295779513082320876798154814105;
pub const EULER: f64 = 2.718281828459045235360287471352;


#[inline]
pub fn radians(deg: f64) -> f64 { deg * DEG_TO_RAD }

#[inline]
pub fn degrees(rad: f64) -> f64 { rad * RAD_TO_DEG }

#[inline]
pub const fn index_as_2d<T>(v: &[T], width: usize, x: usize, y: usize) -> &T {
    &v[x * width + y]
}

pub fn clamp(val: f64, minv: f64, maxv: f64) -> f64 {
    f64::min(f64::max(val, minv), maxv)
}


#[inline]
pub fn ftou8(val: f64) -> u8 {
    (val * 255.999).round() as u8
}

pub struct RectSize {
    pub width: u32,
    pub height: u32,
}