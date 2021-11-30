use crate::vec3d::Vec3D;
use crate::mat3d::Mat3D;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::obj::Object;

pub fn trace(origin: Vec3D, dir: Vec3D, scene: &Scene) -> Vec3D {
  match scene.intersect(origin, dir) {
    Some(hitres) => Vec3D::new(1., 1., 1.),
    None => Vec3D::new(0., 0., 0.),
  }
}

pub fn render(camera: &Camera, scene: &Scene)  -> Vec<Vec<Vec3D>>{
  // todo: parallelism
  let rotation_matrix = Mat3D::rotation(camera.pitch, camera.yaw, camera.roll);
  let mut image: Vec<Vec<Vec3D>> = Vec::new();
  let field_ratio: f64 = camera.fov.tan() / 2.;
  for y in 0..camera.height {
    image.push(Vec::new());
    for x in 0..camera.width {
      let mut u: f64 = ((x as f64)*2. - (camera.width as f64)) as f64;
      let mut v: f64 = ((y as f64)*2. - (camera.height as f64)) as f64;
      u /= camera.height as f64;
      v /= camera.height as f64;
      u *= field_ratio;
      v *= field_ratio;
      let dir = (rotation_matrix * Vec3D::new(u, v, 1.)).normalize();
      image[y as usize].push(trace(camera.origin, dir, &scene));
    }
  }
  image
}
