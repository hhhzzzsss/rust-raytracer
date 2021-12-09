use crate::vec3d::Vec3D;
use crate::hit_result::HitResult;
use crate::util;
use fastrand::Rng;

pub trait Material : Sync {
    fn bounce(&self, dir: Vec3D, hit_result: &HitResult, attenuation: &mut Vec3D, color: &mut Vec3D, rng: &Rng) -> Option<Vec3D>;
}

pub struct BlackHole;
impl Material for BlackHole {
    fn bounce(&self, _dir: Vec3D, _hit_result: &HitResult, _attenuation: &mut Vec3D, _color: &mut Vec3D, rng: &Rng) -> Option<Vec3D> {
        return None;
    }
}

pub struct Dielectric {
    color: Vec3D,
    ior: f64
}

impl Dielectric {
    pub fn new(color: Vec3D, ior: f64) -> Self {
        Self {
            color,
            ior
        }
    }
}

impl Material for Dielectric {
    fn bounce(&self, dir: Vec3D, hit_result: &HitResult, attenuation: &mut Vec3D, color: &mut Vec3D, rng: &Rng) -> Option<Vec3D> {
        let reflectivity = util::fresnel(dir, hit_result.nor, self.ior);
        if rng.f64() < reflectivity {
            return Some(util::reflect(dir, hit_result.nor));
        }
        else {
            let nor = if Vec3D::dot(hit_result.nor, dir) < 0. {hit_result.nor} else {-hit_result.nor};
            let mut new_dir = nor + util::rand_unit_vector(rng);
            if new_dir.near_zero() { new_dir = nor; }
            new_dir = new_dir.normalize();
            *attenuation = (*attenuation) * self.color;
            return Some(new_dir);
        }
    }
}