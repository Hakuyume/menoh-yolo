use image;
use num_traits;

use rect;

use image::GenericImage;

pub fn draw_rect<T, R>(img: &mut image::DynamicImage, rect: &R) -> Option<()>
    where T: num_traits::ToPrimitive,
          R: rect::Rect<T>
{
    let t = 2;
    let color = image::Rgba { data: [255, 0, 0, 0] };

    let y_min = rect.y_min().to_u32()?;
    let x_min = rect.x_min().to_u32()?;
    let y_max = rect.y_max().to_u32()?;
    let x_max = rect.x_max().to_u32()?;

    for y in y_min..y_max {
        for x in &[x_min, x_max] {
            for x in x - t..x + t {
                if y < img.height() && x < img.width() {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
    for x in x_min..x_max {
        for y in &[y_min, y_max] {
            for y in y - t..y + t {
                if y < img.height() && x < img.width() {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }

    Some(())
}
