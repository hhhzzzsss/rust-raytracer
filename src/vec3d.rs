use std::ops::Add;
use std::ops::Mul;

pub struct Vec3D {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Vec3D {
  pub fn new(x: f64, y: f64, z:f64) -> Vec3D {
    let new_vec3d = Vec3D {
      x,
      y,
      z
    };
    new_vec3d
  }

  pub fn normalize(vec: Vec3D) -> Vec3D {
    todo!();
  }
}

impl Add<Vec3D> for Vec3D {
  type Output = Vec3D;

  fn add(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z
    }
  }
}

impl Mul<Vec3D> for Vec3D {
  type Output = Vec3D;

  fn mul(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z
    }
  }
}