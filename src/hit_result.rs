use crate::vec3d::Vec3D;
use crate::material::Material;

pub struct HitResult<'a> {
  pub pos: Vec3D,
	pub nor: Vec3D,
  pub dist: f64,
  pub material: &'a dyn Material
}

impl<'a> HitResult<'a> {
  pub fn new(pos: Vec3D, nor: Vec3D, dist: f64, material: &'a dyn Material) -> Self {
    Self {
      pos,
      nor,
      dist,
      material
    }
  }
}