use crate::vec3d::Vec3D;
use crate::util::mix;
use std::f64::consts::PI;

pub trait Sky {
    fn get_color(&self, dir: Vec3D) -> Vec3D;
}

#[derive(Copy, Clone)]
pub struct SolidSky {
    color: Vec3D
}

impl SolidSky {
    pub fn new(color: Vec3D) -> Self {
        return Self { color }
    }
}

impl Sky for SolidSky {
    fn get_color(&self, dir: Vec3D) -> Vec3D {
        return self.color;
    }
}

#[derive(Copy, Clone)]
pub struct DomeGradientSky {
    horizon_color: Vec3D,
    zenith_color: Vec3D
}

impl DomeGradientSky {
    const ZENITH_VEC: Vec3D = Vec3D{x: 0., y: 1., z: 0.};
    pub fn new(horizon_color: Vec3D, zenith_color: Vec3D) -> Self {
        return Self { horizon_color, zenith_color }
    }

    pub fn from_average(horizon_color: Vec3D, average_color: Vec3D) -> Self {
        let zenith_color = (-2.*PI*horizon_color + average_color) / ((PI-2.)*PI);
        return Self { horizon_color, zenith_color };
    }
}

impl Sky for DomeGradientSky {
    fn get_color(&self, dir: Vec3D) -> Vec3D {
        let a = Vec3D::dot(dir, Self::ZENITH_VEC).max(0.);
        return mix(self.horizon_color, self.zenith_color, a);
    }
}