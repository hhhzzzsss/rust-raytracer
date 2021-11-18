use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use crate::vec3d::Vec3D;

#[derive(Copy, Clone)]
pub struct Mat3D {
  pub x11: f64,
  pub x12: f64,
  pub x13: f64,
  pub x21: f64,
  pub x22: f64,
  pub x23: f64,
  pub x31: f64,
  pub x32: f64,
  pub x33: f64
}

impl Mat3D {
  pub fn new(
    x11: f64, x12: f64, x13:f64,
    x21: f64, x22: f64, x23:f64,
    x31: f64, x32: f64, x33:f64) -> Mat3D {
    Mat3D {
      x11, x12, x13,
      x21, x22, x23,
      x31, x32, x33
    }
  }

  pub fn pitch(angle: f64) -> Mat3D {
    Mat3D::new(
      1.,  0.,          0.,
      0.,  angle.cos(), angle.sin(),
      0., -angle.sin(), angle.cos()
    )
  }

  pub fn yaw(angle: f64) -> Mat3D {
    Mat3D::new(
      angle.cos(), 0., -angle.sin(),
      0.,          1.,  0.,
      angle.sin(), 0.,  angle.cos()
    )
  }

  pub fn roll(angle: f64) -> Mat3D {
    Mat3D::new(
      angle.cos(), -angle.sin(), 0., 
      angle.sin(),  angle.cos(), 0.,
      0.,           0.,          1.
    )
  }

  pub fn rotation(pitch: f64, yaw: f64, roll: f64) -> Mat3D {
    Mat3D::yaw(yaw) * Mat3D::pitch(pitch) * Mat3D::roll(roll)
  }

  pub fn to_string(&self) -> String {
    return format!("Mat3D {{\n\t{:6.2} {:6.2} {:6.2}\n\t{:6.2} {:6.2} {:6.2}\n\t{:6.2} {:6.2} {:6.2} }}",
      self.x11, self.x12, self.x13,
      self.x21, self.x22, self.x23,
      self.x31, self.x32, self.x33
    );
  }
}

impl Mul<Vec3D> for Mat3D {
  type Output = Vec3D;

  fn mul(self, rhs: Vec3D) -> Vec3D {
    Vec3D {
      x: self.x11*rhs.x + self.x12*rhs.y + self.x13*rhs.z,
      y: self.x21*rhs.x + self.x22*rhs.y + self.x23*rhs.z,
      z: self.x31*rhs.x + self.x32*rhs.y + self.x33*rhs.z
    }
  }
}

impl Mul<Mat3D> for Mat3D {
  type Output = Mat3D;

  fn mul(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11: self.x11*rhs.x11 + self.x12*rhs.x21 + self.x13*rhs.x31,
      x12: self.x11*rhs.x12 + self.x12*rhs.x22 + self.x13*rhs.x32,
      x13: self.x11*rhs.x13 + self.x12*rhs.x23 + self.x13*rhs.x33,
      
      x21: self.x21*rhs.x11 + self.x22*rhs.x21 + self.x23*rhs.x31,
      x22: self.x21*rhs.x12 + self.x22*rhs.x22 + self.x23*rhs.x32,
      x23: self.x21*rhs.x13 + self.x22*rhs.x23 + self.x23*rhs.x33,
      
      x31: self.x31*rhs.x11 + self.x32*rhs.x21 + self.x33*rhs.x31,
      x32: self.x31*rhs.x12 + self.x32*rhs.x22 + self.x33*rhs.x32,
      x33: self.x31*rhs.x13 + self.x32*rhs.x23 + self.x33*rhs.x33
    }
  }
}

impl Mul<f64> for Mat3D {
  type Output = Mat3D;

  fn mul(self, rhs: f64) -> Mat3D {
    Mat3D {
      x11: self.x11*rhs,
      x12: self.x12*rhs,
      x13: self.x13*rhs,
      x21: self.x21*rhs,
      x22: self.x22*rhs,
      x23: self.x23*rhs,
      x31: self.x31*rhs,
      x32: self.x32*rhs,
      x33: self.x33*rhs
    }
  }
}

impl Mul<Mat3D> for f64 {
  type Output = Mat3D;

  fn mul(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11: self*rhs.x11,
      x12: self*rhs.x12,
      x13: self*rhs.x13,
      x21: self*rhs.x21,
      x22: self*rhs.x22,
      x23: self*rhs.x23,
      x31: self*rhs.x31,
      x32: self*rhs.x32,
      x33: self*rhs.x33,
    }
  }
}

impl Div<f64> for Mat3D {
  type Output = Mat3D;

  fn div(self, rhs: f64) -> Mat3D {
    Mat3D {
      x11: self.x11/rhs,
      x12: self.x12/rhs,
      x13: self.x13/rhs,
      x21: self.x21/rhs,
      x22: self.x22/rhs,
      x23: self.x23/rhs,
      x31: self.x31/rhs,
      x32: self.x32/rhs,
      x33: self.x33/rhs
    }
  }
}

impl Div<Mat3D> for f64 {
  type Output = Mat3D;

  fn div(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11: self/rhs.x11,
      x12: self/rhs.x12,
      x13: self/rhs.x13,
      x21: self/rhs.x21,
      x22: self/rhs.x22,
      x23: self/rhs.x23,
      x31: self/rhs.x31,
      x32: self/rhs.x32,
      x33: self/rhs.x33,
    }
  }
}

impl Add<Mat3D> for Mat3D {
  type Output = Mat3D;

  fn add(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11: self.x11+rhs.x11,
      x12: self.x12+rhs.x12,
      x13: self.x13+rhs.x13,
      x21: self.x21+rhs.x21,
      x22: self.x22+rhs.x22,
      x23: self.x23+rhs.x23,
      x31: self.x31+rhs.x31,
      x32: self.x32+rhs.x32,
      x33: self.x33+rhs.x33
    }
  }
}

impl Sub<Mat3D> for Mat3D {
  type Output = Mat3D;

  fn sub(self, rhs: Mat3D) -> Mat3D {
    Mat3D {
      x11: self.x11-rhs.x11,
      x12: self.x12-rhs.x12,
      x13: self.x13-rhs.x13,
      x21: self.x21-rhs.x21,
      x22: self.x22-rhs.x22,
      x23: self.x23-rhs.x23,
      x31: self.x31-rhs.x31,
      x32: self.x32-rhs.x32,
      x33: self.x33-rhs.x33
    }
  }
}