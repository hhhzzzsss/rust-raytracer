pub mod file_io;
pub mod color;
pub mod vec3d;
pub mod mat3d;

use crate::color::color::Color;
use crate::vec3d::Vec3D;
use crate::mat3d::Mat3D;

use std::f64::consts::PI;

fn test_image() {
    let mut color_vec : Vec<Vec<Color>> = Vec::new(); 

    for _i in 0..800 {
        let mut inner_vec: Vec<Color> = Vec::new();
        for _j in 0..800 {
            inner_vec.push(Color { r:100, g:100, b:100});
        }
        color_vec.push(inner_vec);
    }

    file_io::file_io::vec_to_jpeg(color_vec);
}

fn test_linalg() {
    let v1 = Vec3D::new(1., 2., 3.);
    println!("v1: {}", v1.to_string());
    let v2 = Vec3D::new(2., 2., 2.);
    println!("v2: {}", v2.to_string());
    println!("v1 + v2: {}", (v1+v2).to_string());
    println!("v1 * v2: {}", (v1*v2).to_string());
    println!("v1 * 2: {}", (v1*2.).to_string());
    println!("2 * v1: {}", (2.*v1).to_string());
    println!("v2 / 2: {}", (v2/2.).to_string());
    println!("2 / v2: {}", (2./v2).to_string());
    println!("Mat3D::pitch(PI/2.): {}", Mat3D::pitch(PI/2.).to_string());
    println!("Mat3D::yaw(PI): {}", Mat3D::yaw(PI).to_string());
    println!("Mat3D::roll(PI/4.): {}", Mat3D::roll(PI/4.).to_string());
    println!("Mat3D::pitch(PI/2.) * v1: {}", (Mat3D::pitch(PI/2.) * v1).to_string());
    println!("Mat3D::pitch(PI/2.) * Mat3D::pitch(PI/2.): {}", (Mat3D::pitch(PI/2.)*Mat3D::pitch(PI/2.)).to_string());
}

fn main() {
    test_linalg();
}
