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

Usage: menoh-yolo [<image>]
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_image: Option<path::PathBuf>,
}

fn main() -> Result<(), Box<dyn(error::Error)>> {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", LABEL_NAMES.len(), "mkldnn", "")?;

    if let Some(path) = args.arg_image {
        let img = image::open(path)?;
        let bbox = model.predict(&img)?;

        let mut img = opencv::IplImage::from_image(img);
        for bb in bbox.iter() {
            opencv::rectangle(&mut img, bb, &[255, 0, 0, 0], Some(3));
        }
        while opencv::wait_key(None) != Some('q') {
            opencv::show_image("result", &img)?;
        }
    } else {
        let mut cap = opencv::Capture::open_camera(0).unwrap();

        while opencv::wait_key(Some(10)) != Some('q') {
            let img = cap.query_frame().unwrap().into_image();
            let bbox = model.predict(&img)?;

            let mut img = opencv::IplImage::from_image(img);
            for bb in bbox.iter() {
                opencv::rectangle(&mut img, bb, &[255, 0, 0, 0], Some(3));
            }
            opencv::show_image("result", &img)?;
        }
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
