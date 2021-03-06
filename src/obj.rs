use crate::vec3d::Vec3D;
use crate::material::Material;
use crate::hit_result::HitResult;

pub trait Object<'a> {
  fn intersect(&self, origin: Vec3D, dir: Vec3D) -> Option<HitResult<'a>>;
}

pub struct Sphere<'a> {
  center: Vec3D,
  radius: f64,
  material: &'a dyn Material
}

impl<'a> Sphere<'a> {
  pub fn new(center: Vec3D, radius: f64, material: &'a dyn Material) -> Self {
    Self {
      center,
      radius,
      material
    }
  }
}

impl<'a> Object<'a> for Sphere<'a> {
  fn intersect(&self, origin: Vec3D, dir: Vec3D) -> Option<HitResult<'a>> {
    let oc = origin - self.center;
    let a = dir.length_squared();
    let half_b = Vec3D::dot(oc, dir);
    let c = oc.length_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 { return None; }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root <= 0. {
      root = (-half_b + sqrtd) / a;
      if root <= 0. { return None; }
    }

    let pos = origin + root*dir;
    let nor = (pos - self.center) / self.radius;
    let dist = root;
    let material = self.material;
    Some(HitResult {
      pos,
      nor,
      dist,
      material
    })
  }
}

pub struct HorizontalPlane<'a> {
  y: f64,
  material: &'a dyn Material
}

impl<'a> HorizontalPlane<'a> {
  pub const NORMAL: Vec3D = Vec3D{x:0., y:1., z:0.};
  pub fn new(y: f64, material: &'a dyn Material) -> Self {
    Self {
      y,
      material
    }
  }
}

impl<'a> Object<'a> for HorizontalPlane<'a> {
  fn intersect(&self, origin: Vec3D, dir: Vec3D) -> Option<HitResult<'a>> {
    if dir.y == 0. { return None; }

    let dy = origin.y - self.y;
    let dist = - dy / (Vec3D::dot(dir, Self::NORMAL));

    if dist < 0. { return None; }

    let pos = origin + dist*dir;
    let nor = Self::NORMAL;
    let material = self.material;

    Some(HitResult {
      pos,
      nor,
      dist,
      material
    })
  }
}