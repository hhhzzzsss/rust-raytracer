use crate::vec3d::Vec3D;
use crate::mat3d::Mat3D;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::obj::Object;
use fastrand::Rng;

pub const MIN_DIST: f64 = 0.0002;
pub const MAX_DIST: f64 = 1000000.0;

pub fn trace(origin: Vec3D, initial_dir: Vec3D, scene: &Scene, max_bounces: u32, rng: &Rng) -> Vec3D {
  let mut pos = origin; 
  let mut dir = initial_dir;
  let mut color = Vec3D::new(0., 0., 0.);
  let mut attenuation = Vec3D::new(1., 1., 1.);
  for _i in 0..max_bounces {
    let done: bool = match scene.intersect(pos + MIN_DIST*dir, dir) {
      Some(hitres) => { // hits object
        if hitres.dist > MAX_DIST { // treat as no hit if exceeds MAX_DIST
          color = color + scene.sky.get_color(dir) * attenuation;
          true
        } else {
          pos = hitres.pos;
          match hitres.material.bounce(dir, &hitres, &mut attenuation, &mut color, rng) {
            Some(newdir) => { // ray bounce
              dir = newdir;
              false
            },
            None => { // no ray bounce
              true
            }
          }
        }
      },
      None => { // no hit
        color = color + scene.sky.get_color(dir) * attenuation;
        true
      },
    };
    
    if done {
      break;
    }
  }

  return color;
}

pub fn render(camera: &Camera, scene: &Scene)  -> Vec<Vec<Vec3D>> {
  // todo: parallelism
  let rng = Rng::new();
  let rotation_matrix = Mat3D::rotation(camera.pitch, camera.yaw, camera.roll);
  let mut image: Vec<Vec<Vec3D>> = Vec::new();
  let field_ratio: f64 = camera.fov.tan() / 2.;
  let pixel_size = 2. * field_ratio / (camera.height as f64);
  for y in 0..camera.height {
    image.push(Vec::new());
    for x in 0..camera.width {
      let mut u: f64 = ((x as f64)*2. - (camera.width as f64)) as f64;
      let mut v: f64 = ((camera.height as f64) - (y as f64)*2.) as f64;
      u /= camera.height as f64;
      v /= camera.height as f64;
      u *= field_ratio;
      v *= field_ratio;
      u += rng.f64() * pixel_size;
      v += rng.f64() * pixel_size;
      let dir = (rotation_matrix * Vec3D::new(u, v, 1.)).normalize();
      let mut color = Vec3D::new(0., 0., 0.);
      for _i in 0..camera.samples {
        color = color + trace(camera.origin, dir, &scene, 8, &rng);
      }
      color = color / (camera.samples as f64);
      color = color.powf(0.45); // gamma correction
      image[y as usize].push(color);
    }
  }
  image
}
