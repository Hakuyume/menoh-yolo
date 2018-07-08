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
#[cfg(not(feature = "opencv"))]
mod draw;
mod model_ext;
#[cfg(feature = "opencv")]
mod opencv;
mod partial_cmp;
mod rect;
mod yolo_v2;

#[cfg(feature = "opencv")]
fn main() -> Result<(), Box<dyn(error::Error)>> {
    const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo [<image>]
"#;

    #[derive(Debug, Deserialize)]
    struct Args {
        arg_image: Option<path::PathBuf>,
    }

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", LABEL_NAMES.len(), "mkldnn", "")?;
    let mut predict = |img| -> Result<_, menoh::Error> {
        let bbox = model.predict(&img)?;
        let mut img = opencv::IplImage::from_image(img);
        for bb in bbox.iter() {
            opencv::rectangle(&mut img, bb, &[255, 0, 0, 0], Some(3));
        }
        Ok(img)
    };

    if let Some(path) = args.arg_image {
        let img = predict(image::open(path)?)?;
        while opencv::wait_key(None) != Some('q') {
            opencv::show_image("result", &img)?;
        }
    } else {
        let mut cap = opencv::Capture::open_camera(0).unwrap();
        while opencv::wait_key(Some(10)) != Some('q') {
            let img = predict(cap.query_frame().unwrap().into_image())?;
            opencv::show_image("result", &img)?;
        }
    }

    Ok(())
}

#[cfg(not(feature = "opencv"))]
fn main() -> Result<(), Box<dyn(error::Error)>> {
    const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo <src> <dest>
"#;

    #[derive(Debug, Deserialize)]
    struct Args {
        arg_src: path::PathBuf,
        arg_dest: path::PathBuf,
    }

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", LABEL_NAMES.len(), "mkldnn", "")?;

    let mut img = image::open(args.arg_src)?;
    let bbox = model.predict(&img)?;
    for bb in bbox.iter() {
        draw::draw_rect(&mut img, bb);
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
