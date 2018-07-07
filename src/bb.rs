use image;

use rect;

use image::GenericImage;

pub struct Bb {
    pub y_min: f32,
    pub x_min: f32,
    pub y_max: f32,
    pub x_max: f32,
    pub label: usize,
    pub score: f32,
}

impl rect::Rect<f32> for Bb {
    fn y_min(&self) -> f32 {
        self.y_min
    }
    fn x_min(&self) -> f32 {
        self.x_min
    }
    fn y_max(&self) -> f32 {
        self.y_max
    }
    fn x_max(&self) -> f32 {
        self.x_max
    }
}

pub fn draw(img: &mut image::DynamicImage, bb: &Bb, t: u32) {
    let y_min = bb.y_min as u32;
    let x_min = bb.x_min as u32;
    let y_max = bb.y_max as u32;
    let x_max = bb.x_max as u32;

    for y in y_min..y_max {
        for x in &[x_min, x_max] {
            for x in x - t..x + t {
                if y < img.height() && x < img.width() {
                    img.put_pixel(x, y, image::Rgba { data: [255, 0, 0, 0] });
                }
            }
        }
    }
    for x in x_min..x_max {
        for y in &[y_min, y_max] {
            for y in y - t..y + t {
                if y < img.height() && x < img.width() {
                    img.put_pixel(x, y, image::Rgba { data: [255, 0, 0, 0] });
                }
            }
        }
    }
}
