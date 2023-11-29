use std::io::Write;

#[derive(Default, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<RGB>,
}

pub enum ImageType {
    Str,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut v = Vec::new();
        v.resize(width*height, RGB::default());
        Image {
            width, height,
            pixels: v
        }
    }
    pub fn write_file(&self, path: &str, image_type: ImageType) {
        match image_type {
            ImageType::Str => self.write_string_file(path),
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: RGB) {
        self.pixels[x + y*self.width] = color;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> &RGB {
        &self.pixels[x + y*self.width]
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }

    fn write_string_file(&self, path: &str) {
        let mut content = String::new();

        content.push_str("P3\n");
        content.push_str(format!("{} {}\n", self.width, self.height).as_str());
        content.push_str("255\n");

        content.push_str(self.pixels.iter().map(|p| format!("{} {} {} ", p.r, p.g, p.b)).collect::<String>().as_str());

        let mut file = std::fs::File::create(path).expect(format!("File {} not found", path).as_str());
        file.write_all(content.as_bytes()).expect("Unable to write file");
    }
}


fn main() {
    let mut image = Image::new(100, 100);

    for i in 0..100 {
        for j in 0..100 {
            let rgb = RGB {
                r: (i*255/100) as u8, g: ((100-j)*255/100) as u8, b: 125
            };
            image.set_pixel(i as usize, j as usize, rgb);
        }
    }

    image.write_file("out.ppm", ImageType::Str)
}
