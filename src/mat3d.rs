pub struct Mat3D {
  x11: f64,
  x12: f64,
  x13: f64,
  x21: f64,
  x22: f64,
  x23: f64,
  x31: f64,
  x32: f64,
  x33: f64
}

impl Mul<Vec3D> for Mat3D {
  type Output = Vec3D;

  fn mul(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x11*rhs.x + x12*rhs.y + x13*rhs.z,
      x21*rhs.x + x22*rhs.y + x23*rhs.z,
      x31*rhs.x + x32*rhs.y + x33*rhs.z
    }
  }
}

impl Mul<Mat3D> for Mat3D {
  type Output = Mat3D;

  fn mul(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11*rhs.x11 + x12*rhs.x21 + x13*rhs.x31,
      x11*rhs.x12 + x12*rhs.x22 + x13*rhs.x32,
      x11*rhs.x13 + x12*rhs.x23 + x13*rhs.x33,
      
      x21*rhs.x11 + x22*rhs.x21 + x23*rhs.x31,
      x21*rhs.x12 + x22*rhs.x22 + x23*rhs.x32,
      x21*rhs.x13 + x22*rhs.x23 + x23*rhs.x33,
      
      x31*rhs.x11 + x32*rhs.x21 + x33*rhs.x31,
      x31*rhs.x12 + x32*rhs.x22 + x33*rhs.x32,
      x31*rhs.x13 + x32*rhs.x23 + x33*rhs.x33
    }
  }
}