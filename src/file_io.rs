pub mod file_io {
    use image::{ImageBuffer, Rgb};
    use crate::color::color::Color;

    pub fn test() {
        println!("Test function called!");
    }

    //Assumes that position in inner vector determines x-axis position
    //and position in outer vector determines y-axis position
    pub fn vec_to_jpeg(colors: Vec<Vec<Color>>) {
        let imgx: u32 = colors.get(0).unwrap().len() as u32;
        let imgy: u32 = colors.len() as u32;

        let mut img = ImageBuffer::new(imgx, imgy);

        for y in 0..imgy {
            for x in 0..imgx {
                let color = colors.get(y as usize).unwrap().get(x as usize).unwrap();
                img.put_pixel(x, y, Rgb([color.r, color.g, color.b]));
            }
        }
        img.save("image.jpg").unwrap();
    }
}