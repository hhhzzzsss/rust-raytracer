use crate::vec3d::Vec3D;

pub struct Camera {
  pub origin: Vec3D,
  pub pitch: f64,
  pub yaw: f64,
  pub roll: f64,
  pub fov: f64,
  pub width: usize,
  pub height: usize,
  pub samples: u32
}

impl Camera {
  pub fn new(origin: Vec3D, pitch: f64, yaw: f64, roll: f64) -> Camera {
    Camera {
      origin: origin,
      pitch: pitch,
      yaw: yaw,
      roll: roll,
      fov: 1.,
      width: 800,
      height: 600,
      samples: 256
    }
  }
}