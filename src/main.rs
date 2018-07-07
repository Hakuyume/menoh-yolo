extern crate image;
extern crate menoh;
#[macro_use]
extern crate ndarray;
extern crate num_traits;

use std::error;

mod bb;
mod model_ext;
mod partial_cmp;
mod rect;
mod yolo_v2;

use image::GenericImage;

const USAGE: &'static str = r#"
YOLOv2 on Menoh

Usage: menoh-yolo [src] [dest]
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_src: path::PathBuf,
    arg_dest: path::PathBuf,
}

fn main() -> Result<(), Box<dyn(error::Error)>> {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", 20, "mkldnn", "")?;

    let mut img = image::open(arg.arg_src)?;
    let bbox = model.predict(&img)?;
    for bb in bbox.iter() {
        draw_bb(&mut img, bb, 2);
        println!("{} {}", bb.label, bb.score);
    }
    img.save(arg.arg_dest)?;

    Ok(())
}

fn draw_bb(img: &mut image::DynamicImage, bb: &bb::Bb, t: u32) {
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
