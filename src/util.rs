use crate::vec3d::Vec3D;

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
