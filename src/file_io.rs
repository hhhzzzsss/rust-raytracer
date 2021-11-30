pub mod file_io {
    use image::{ImageBuffer, Rgb};
    use crate::vec3d::Vec3D;
    use crate::util::Clamp;

    pub fn test() {
        println!("Test function called!");
    }

    fn Vec3D_to_Rgb(v: Vec3D) -> Rgb<u8> {
        let rescaled = (v*256.).clamp(0., 255.);
        Rgb([
            rescaled.x as u8,
            rescaled.y as u8,
            rescaled.z as u8
        ])
    }

    //Assumes that position in inner vector determines x-axis position
    //and position in outer vector determines y-axis position
    pub fn vec_to_jpeg(colors: Vec<Vec<Vec3D>>) {
        let imgx: u32 = colors.get(0).unwrap().len() as u32;
        let imgy: u32 = colors.len() as u32;

        let mut img = ImageBuffer::new(imgx, imgy);

        for y in 0..imgy {
            for x in 0..imgx {
                let color = *colors.get(y as usize).unwrap().get(x as usize).unwrap();
                img.put_pixel(x, y, Vec3D_to_Rgb(color));
            }
        }
        img.save("image.jpg").unwrap();
    }
}