#[derive(Copy, Clone)]
pub trait Material {
    pub fn bounce(dir: Vec3D, nor: Vec3D, attenuation: &mut Vec3D, color: &mut Vec3D) -> Option<Vec3D>;
}

pub struct BlackHole;
impl Material for BlackHole {
    pub fn bounce(dir: Vec3D, nor: Vec3D, attenuation: &mut Vec3D, color: &mut Vec3D) -> Option<Vec3D> {
        return None();
    }
}