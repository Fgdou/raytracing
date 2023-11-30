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
    Binary,
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
            ImageType::Binary => self.write_binary_file(path),
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
    fn write_binary_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).expect(format!("File {} not found", path).as_str());
        let _ = file.write("P6\n".as_bytes());
        let _ = file.write(format!("{} {}\n", self.width, self.height).as_bytes());
        let _ = file.write("255\n".as_bytes());

        let mut v: Vec<u8> = Vec::new();
        for p in &self.pixels {
            v.push(p.r);
            v.push(p.g);
            v.push(p.b);
        }
        let _ = file.write(&v);
    }
}