use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::{Rng, SeedableRng};

type Vec3_X = f64;
type Vec3_Y = f64;
type Vec3_Z = f64;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3(pub Vec3_X, pub Vec3_Y, pub Vec3_Y);

impl Vec3 {
    pub fn new_rand() -> Self {
        Self::new_rand_range(0.0, 1.0)
    }

    pub fn new_rand_range(min: f64, max: f64) -> Self {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let range = min..max;

        Self(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
        )
    }

    pub fn new_rand_unit_vector() -> Self {
        Self::new_rand_unit_sphere().normalize()
    }

    pub fn new_rand_unit_sphere() -> Self {
        loop {
            let p = Self::new_rand_range(-1.0, 1.0);
            if p.len_sq() >= 1.0 {
                continue;
            } else {
                return p.normalize();
            }
        }
    }

    pub fn new_rand_in_unit_disk() -> Self {
        let mut rng = rand::rngs::StdRng::from_entropy();
        loop {
            let p = Vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.len_sq() < 1.0 {
                break p;
            } else {
                continue;
            }
        }
    }

    pub const fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn lerp(&self, other: &Vec3, t: f64) -> Self {
        self.mul_scalar(1.0 - t) + other.mul_scalar(t)
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub const fn x(&self) -> f64 {
        self.0
    }
    pub const fn y(&self) -> f64 {
        self.1
    }
    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn add_scalar(&self, s: f64) -> Self {
        Self(self.x() + s, self.y() + s, self.z() + s)
    }

    pub fn mul_scalar(&self, s: f64) -> Self {
        Self(self.x() * s, self.y() * s, self.z() * s)
    }

    pub fn div_scalar(&self, s: f64) -> Self {
        Self(self.x() / s, self.y() / s, self.z() / s)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn normalize(&self) -> Self {
        self.div_scalar(self.len())
    }

    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.x().abs() < EPSILON && self.y().abs() < EPSILON && self.z().abs() < EPSILON
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

pub type Color = Vec3;
impl Color {
    pub const WHITE: Color = Vec3(1.0, 1.0, 1.0);
    pub const BLACK: Color = Vec3(0.0, 0.0, 0.0);
    pub const RED: Color = Vec3(1.0, 0.0, 0.0);
    pub const GREEN: Color = Vec3(0.0, 1.0, 0.0);
    pub const BLUE: Color = Vec3(0.0, 0.0, 1.0);
}
