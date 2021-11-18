pub fn intersect() -> Option<f64> {
  todo!();
}

pub fn trace() -> Vec3D {
  todo!();
}

pub fn render(Camera camera, Scene scene) -> Vec<<Vec<Vec3D>> {
  // todo: parallelism
  Mat3D rotation_matrix = Mat3D::rotation(camera.pitch, camera.yaw, camera.roll);
  let image: Vec<Color> = Vec::new();
  for y in 0..camera.height {
    image.push(Vec::new());
    for x in 0..camera.width {
      let u: f64 = x*2 - camera.width;
      let v: f64 = y*2 - camera.height;
      u /= camera.height;
      v /= camera.width;
      let field_ratio: f64 = camera.fov.tan() / 2;
      u *= field_ratio;
      v *= field_ratio;
      Vec3D ray_direction = normalize(rotation_matrix * (Vec3D {u, v, 1}));
      image[x].push(trace(camera.origin, ray_direction, scene));
    }
    image
  }
}
