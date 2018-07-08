extern crate docopt;
extern crate image;
extern crate menoh;
#[macro_use]
extern crate ndarray;
extern crate num_traits;
#[macro_use]
extern crate serde_derive;

use std::error;
use std::path;

mod bb;
mod model_ext;
mod partial_cmp;
mod rect;
mod yolo_v2;

use image::GenericImage;
use rect::Rect;

const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo <src> <dest>
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

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", LABEL_NAMES.len(), "mkldnn", "")?;

    let mut img = image::open(args.arg_src)?;
    let bbox = model.predict(&img)?;
    for bb in bbox.iter() {
        draw_rect(&mut img, bb);
        println!("{}: ({}, {}, {}, {}) {}",
                 LABEL_NAMES[bb.label],
                 bb.y_min,
                 bb.x_min,
                 bb.y_max,
                 bb.x_max,
                 bb.score);
    }
    img.save(args.arg_dest)?;

    Ok(())
}

pub fn draw_rect<R>(img: &mut image::DynamicImage, rect: &R)
    where R: Rect<f32>
{
    let t = 2;
    let color = image::Rgba { data: [255, 0, 0, 0] };

    let y_min = rect.y_min() as u32;
    let x_min = rect.x_min() as u32;
    let y_max = rect.y_max() as u32;
    let x_max = rect.x_max() as u32;

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
}

const LABEL_NAMES: &'static [&'static str] = &["aeroplane",
                                               "bicycle",
                                               "bird",
                                               "boat",
                                               "bottle",
                                               "bus",
                                               "car",
                                               "cat",
                                               "chair",
                                               "cow",
                                               "diningtable",
                                               "dog",
                                               "horse",
                                               "motorbike",
                                               "person",
                                               "pottedplant",
                                               "sheep",
                                               "sofa",
                                               "train",
                                               "tvmonitor"];
