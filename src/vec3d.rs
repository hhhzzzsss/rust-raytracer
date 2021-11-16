pub struct Vec3D {
  x: u32,
  y: u32,
  z: u32
}

impl Vec3D {
  pub fn new(x: u32, y: u32, z:u32) -> Vec3D {
      let new_vec3d = Vec3D {
          x,
          y,
          z
      };
      new_vec3d
  }

  pub fn normalize(Vec3D vec) -> Vec3D {
    todo!();
  }
}