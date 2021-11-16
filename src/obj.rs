trait Object {
  pub fn new(Vec3D center) -> Self;
  pub fn intersect(Vec3D dir) -> Option<Color>;
}