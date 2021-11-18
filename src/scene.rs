use cargo::hit_result::HitResult;

struct Scene {
  objects: Vec<Object>
}

impl Scene {
  fn new() -> Self {
    Self {
      objects: Vec::new();
    }
  }

  fn add(object: Object) {
    objects.push(object);
  }
}

impl Object for Scene {
  fn intersect(origin: Vec3D, dir: Vec3D) -> Option<HitResult> {
    let shortest = None();
    for object in objects {
      let hitres = object.intersect(origin, dir);
      if (shortest.is_none()) {
        shortest = hitres;
      } else if (hitres.is_some() && hitres.dist < shortest.dist>) {
        shortest = hitres;
      }
    }
    shortest
  }
}