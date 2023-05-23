use super::Color;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn from_color(color: Color) -> Pixel {
        Pixel {
            r: (color.x.sqrt() * 255.999) as u8,
            g: (color.y.sqrt() * 255.999) as u8,
            b: (color.z.sqrt() * 255.999) as u8,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(Pixel { r: 0, g: 0, b: 0 })
        }

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        assert!(y < self.height);
        assert!(x < self.width);
        self.data[y * self.width + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        assert!(y < self.height);
        assert!(x < self.width);
        self.data[y * self.width + x] = pixel;
    }
}
