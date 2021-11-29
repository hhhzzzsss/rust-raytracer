use crate::vec3d::Vec3D;
use crate::hit_result::HitResult;

pub trait Material {
    fn bounce(&self, dir: Vec3D, hit_result: &HitResult, attenuation: &mut Vec3D, color: &mut Vec3D) -> Option<Vec3D>;
}

#[derive(Copy, Clone)]
pub struct BlackHole;
impl Material for BlackHole {
    fn bounce(&self, dir: Vec3D, hit_result: &HitResult, attenuation: &mut Vec3D, color: &mut Vec3D) -> Option<Vec3D> {
        return None;
    }
}