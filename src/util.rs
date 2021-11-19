use crate::vec3d::Vec3D;

pub fn clamp(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        return a;
    }
    if x > b {
        return b;
    }
    return x;
}

pub fn clamp(v: Vec3D, a: f64; b: f64) -> Vec3D {
    Vec3D::new(
        clamp(v.x, a, b),
        clamp(v.y, a, b),
        clamp(v.z, a, b)
    );
}