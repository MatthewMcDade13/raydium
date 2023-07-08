use std::ops::{Add, Sub, Div, Mul};


type Vec3_X = f64;
type Vec3_Y = f64;
type Vec3_Z = f64;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3(pub Vec3_X, pub Vec3_Y, pub Vec3_Y);


impl Vec3 {

    pub const fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
    }

    pub const fn x(&self) -> f64 { self.0 }
    pub const fn y(&self) -> f64 { self.1 }
    pub const fn z(&self) -> f64 { self.2 }

    pub fn mul_scalar(&self, s: f64) -> Self {
        Self(self.x() * s, self.y() * s, self.z() * s)
    }

    pub fn div_scalar(&self, s: f64) -> Self {
        Self(self.x() / s, self.y() / s, self.z() / s)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x()*other.x() 
            + self.y()*other.y() 
            + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit_vector(&self) -> Self {
        self.div_scalar(self.len())
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

pub type Point3 = Vec3;
pub type Color = Vec3;
