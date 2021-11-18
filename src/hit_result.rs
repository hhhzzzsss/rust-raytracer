#[derive(Copy, Clone)]
pub struct HitResult {
  dir: Vec3D,
	nor: Vec3D,
  dist: f64,
  front_face: bool,
  material: &Material,
}

impl HitResult {
  pub fn new(dir: Vec3D, nor: Vec3D, dist: f64, front_face: bool, material: &Material) -> Self {
    Self {
      dir,
      nor,
      dist,
      front_face,
      material
    }
  }
}