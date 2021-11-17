pub mod file_io;
pub mod color;

fn main() {
    use crate::color::color::Color;
    println!("Hello, world!");

    file_io::file_io::test();

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
