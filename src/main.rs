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
mod opencv;
mod partial_cmp;
mod rect;
mod yolo_v2;

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

    let img = image::open(args.arg_src)?;
    let bbox = model.predict(&img)?;

    let mut img = opencv::Mat::from_image(img);
    for bb in bbox.iter() {
        opencv::rectangle(&mut img, bb, Some(3));
        println!("{}: ({}, {}, {}, {}) {}",
                 LABEL_NAMES[bb.label],
                 bb.y_min,
                 bb.x_min,
                 bb.y_max,
                 bb.x_max,
                 bb.score);
    }
    while opencv::wait_key(None) != Some('q') {
        opencv::show_image("result", &img)?;
    }

    Ok(())
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
