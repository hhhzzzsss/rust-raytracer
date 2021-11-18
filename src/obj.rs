pub trait Object {
  pub fn intersect(Vec3D origin, Vec3D dir) -> Option<HitResult>;
}