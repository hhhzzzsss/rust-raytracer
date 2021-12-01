use std::ops::Add;
use std::ops::Mul;
use crate::vec3d::Vec3D;
use fastrand::Rng;
use std::f64::consts::PI;

pub trait Clamp {
  fn clamp(self, a: f64, b: f64) -> Self;
}

impl Clamp for f64 {
  fn clamp(self, a: f64, b: f64) -> Self {
    if self < a {
      return a;
    }
    if self > b {
      return b;
    }
    return self;
  }
}

impl Clamp for Vec3D {
  fn clamp(self, a: f64, b: f64) -> Self {
    Vec3D::new(
      self.x.clamp(a, b),
      self.y.clamp(a, b),
      self.z.clamp(a, b)
    )
  }
}

pub fn mix<T: Mul<f64, Output=T> + Add<Output=T>>(x: T, y: T, a: f64) -> T {
  return x * (1.-a) + y * a;
}

pub fn rand_unit_vector(rng: &Rng) -> Vec3D {
  let z = rng.f64()*2. - 1.;
  let theta = rng.f64()*2.*PI;
  let s = (1. - z*z).sqrt();
  return Vec3D::new(s*theta.cos(), s*theta.sin(), z);
}

pub fn reflect(incident: Vec3D, normal: Vec3D) -> Vec3D {
  return incident - 2.*Vec3D::dot(incident,normal) * normal;
}

pub fn fresnel(incident: Vec3D, normal: Vec3D, ior: f64) -> f64 {
  let mut n1 = 1.;
  let mut n2 = 1.;
  if Vec3D::dot(incident, normal) < 0. {
    n2 = ior;
  }
  else {
    n1 = ior;
  }

  let cosi = Vec3D::dot(incident, normal).clamp(-1., 1.).abs();
  let sint = n1 / n2 * (1. - cosi*cosi).max(0.).sqrt();
  
  if sint >= 1. {
    return 1.;
  }
  else {
    let cost = (1. - sint*sint).max(0.).sqrt();
    let rs = ((n2 * cosi) - (n1 * cost)) / ((n2 * cosi) + (n1 * cost));
    let rp = ((n1 * cosi) - (n2 * cost)) / ((n1 * cosi) + (n2 * cost));
    return (rs * rs + rp * rp) / 2.;
  }
}