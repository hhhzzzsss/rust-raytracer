pub struct Camera {
  origin: Vec3D,
  pitch: f64,
  yaw: f64,
  roll: f64,
  fov: f64,
  width: u32,
  height: u32
}

impl Camera {
  pub fn new(origin: Vec3D, pitch: f64, yaw: f64, roll: f64) -> Camera {
    Camera {
      origin: origin,
      pitch: pitch,
      yaw: yaw,
      roll: roll,
      fov: 1,
      width: 800,
      height: 600
    }
  }
}