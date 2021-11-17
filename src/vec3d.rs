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