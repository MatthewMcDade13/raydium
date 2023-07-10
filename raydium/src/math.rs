pub type IOResult<T> = std::io::Result<T>;

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

pub fn clamp<T: std::cmp::Ord>(val: T, minv: T, maxv: T) -> T {
    use std::cmp;
    cmp::min(cmp::max(val, minv), maxv)
}
