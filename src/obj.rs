pub trait Object {
  pub fn intersect(origin: Vec3D, dir: Vec3D) -> Option<HitResult>;
}

pub struct Sphere {
  center: Vec3D,
  radius: f64,
  material: Material
}

impl Sphere {
  pub fn new(center: Vec3D, radius: f64, material: Material) -> Self {
    Self {
      center,
      radius,
      material
    }
  }
}

impl Object for Sphere {
  pub fn intersect(&self, origin: Vec3D, dir: Vec3D) -> HitResult {
    let oc = origin - self.center;
    let a = dir.length_squared();
    let half_b = Vec3D::dot(oc, dir);
    let c = oc.length_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 { return None(); }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if (root <= 0.) {
      root = (-half_b + sqrtd) / a;
      if (root <= 0.) return None();
    }

    HitResult {
      pos: origin + root*dir;
      nor: (res.p - self.center) / self.radius;
      dist: root;
      material: self.material; 
    }
  }
}