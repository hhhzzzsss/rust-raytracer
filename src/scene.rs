use crate::vec3d::Vec3D;
use crate::obj::Object;
use crate::hit_result::HitResult;

pub struct Scene<'a> {
  pub objects: Vec<Box<dyn Object<'a> + 'a>>
}

impl<'a> Scene<'a> {
  pub fn new() -> Self {
    Self {
      objects: Vec::new()
    }
  }

  pub fn add<T: Object<'a> + 'a>(&mut self, object: T) {
    self.objects.push(Box::new(object));
  }
}

impl<'a> Object<'a> for Scene<'a> {
  fn intersect(&self, origin: Vec3D, dir: Vec3D) -> Option<HitResult<'a>> {
    let mut shortest_opt = None;

    for object in &self.objects {
      let hitres_opt = object.intersect(origin, dir);
      shortest_opt = match &shortest_opt {
        None => hitres_opt,
        Some(shortest) => {
          match &hitres_opt {
            None => shortest_opt,
            Some(hitres) => {
              if hitres.dist < shortest.dist { hitres_opt }
              else { shortest_opt }
            }
          }
        }
      }
    }

    shortest_opt
  }
}