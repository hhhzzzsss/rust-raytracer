#[derive(Copy, Clone)]
pub struct HitResult {
  pos: Vec3D,
	nor: Vec3D,
  dist: f64,
  front_face: bool,
  material: &Material,
}

impl HitResult {
  pub fn new(pos: Vec3D, nor: Vec3D, dist: f64, front_face: bool, material: &Material) -> Self {
    Self {
      pos,
      nor,
      dist,
      front_face,
      material
    }
  }
}