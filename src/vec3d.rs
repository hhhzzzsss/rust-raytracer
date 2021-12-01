use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

#[derive(Copy, Clone)]
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

  pub fn dot(v1: Vec3D, v2: Vec3D) -> f64 {
    return v1.x*v2.x + v1.y*v2.y + v1.z*v2.z;
  }

  pub fn length_squared(&self) -> f64 {
    return self.x*self.x + self.y*self.y + self.z*self.z;
  }

  pub fn length(&self) -> f64 {
    return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
  }

  pub fn near_zero(&self) -> bool {
    return self.x.abs() < 0.0001 && self.y.abs() < 0.0001 && self.z.abs() < 0.0001;
  }

  pub fn normalize(&self) -> Vec3D {
    let length = self.length();
    Vec3D {
      x: self.x / length,
      y: self.y / length,
      z: self.z / length
    }
  }

  pub fn to_string(&self) -> String {
    return format!("Vec3D {{ {:.3}, {:.3}, {:.3} }}", self.x, self.y, self.z);
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

impl Sub<Vec3D> for Vec3D {
  type Output = Vec3D;

  fn sub(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
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

impl Div<Vec3D> for Vec3D {
  type Output = Vec3D;

  fn div(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z
    }
  }
}

impl Neg for Vec3D {
  type Output = Vec3D;

  fn neg(self) -> Vec3D {
    Vec3D {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}

impl Mul<f64> for Vec3D {
  type Output = Vec3D;

  fn mul(self, rhs: f64) -> Vec3D {
    Vec3D {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs
    }
  }
}

impl Mul<Vec3D> for f64 {
  type Output = Vec3D;

  fn mul(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self * rhs.x,
      y: self * rhs.y,
      z: self * rhs.z
    }
  }
}

impl Div<f64> for Vec3D {
  type Output = Vec3D;

  fn div(self, rhs: f64) -> Vec3D {
    Vec3D {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs
    }
  }
}

impl Div<Vec3D> for f64 {
  type Output = Vec3D;

  fn div(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self / rhs.x,
      y: self / rhs.y,
      z: self / rhs.z
    }
  }
}