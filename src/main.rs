mod image;
mod vec;
use image::{Image, RGB, ImageType};


fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    for i in 0..size {
        for j in 0..size {
            let rgb = RGB {
                r: (i*255/size) as u8, g: ((size-j)*255/size) as u8, b: 125
            };
            image.set_pixel(i as usize, j as usize, rgb);
        }
    }

    image.write_file("out.ppm", ImageType::Binary)
}
