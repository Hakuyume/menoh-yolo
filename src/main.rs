extern crate docopt;
extern crate image;
extern crate imageproc;
extern crate menoh;
#[macro_use]
extern crate ndarray;
extern crate num_traits;
extern crate rusttype;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

use std::error;
use std::path;

mod bb;
mod drawing;
mod model_ext;
#[cfg(feature = "opencv")]
mod opencv;
mod partial_cmp;
mod rect;
mod yolo_v2;

const LABEL_NAMES: &'static [&'static str] = &[
    "aeroplane",
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
    "tvmonitor",
];

#[cfg(not(feature = "opencv"))]
pub fn main() -> Result<(), Box<dyn error::Error>> {
    use rect::Rect;

    const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo [options] <src> <dest>

Options:
  --model PATH  onnx model path [default: YOLOv2.onnx]
"#;

    #[derive(Debug, Deserialize)]
    struct Args {
        arg_src: path::PathBuf,
        arg_dest: path::PathBuf,
        flag_model: path::PathBuf,
    }

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx(args.flag_model, LABEL_NAMES.len(), "mkldnn", "")?;
    let font = drawing::get_font()?;

    let mut img = image::open(args.arg_src)?;
    let bbox = model.predict(&img)?;
    drawing::draw_bbox_mut(&mut img, bbox.iter(), LABEL_NAMES, &font);
    for bb in bbox.iter() {
        println!(
            "{}, ({}, {}, {}, {}) {}",
            LABEL_NAMES[bb.label],
            bb.top(),
            bb.left(),
            bb.bottom(),
            bb.right(),
            bb.score
        );
    }
    img.save(args.arg_dest)?;

    Ok(())
}

#[cfg(feature = "opencv")]
pub fn main() -> Result<(), Box<dyn error::Error>> {
    use std::time;

    const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo [options]

Options:
  --model PATH  onnx model path [default: YOLOv2.onnx]
  --camera ID  camera ID [default: 0]
"#;

    #[derive(Debug, Deserialize)]
    struct Args {
        flag_model: path::PathBuf,
        flag_camera: usize,
    }

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx(args.flag_model, LABEL_NAMES.len(), "mkldnn", "")?;
    let font = drawing::get_font()?;

    let mut cap = opencv::Capture::open_camera(args.flag_camera).unwrap();
    let start = time::Instant::now();
    let mut n_frame = 0;

    while opencv::wait_key(Some(10)) != Some('q') {
        let mut img = cap.query_frame().unwrap().into_image();
        let bbox = model.predict(&img)?;
        drawing::draw_bbox_mut(&mut img, bbox.iter(), LABEL_NAMES, &font);

        n_frame += 1;
        imageproc::drawing::draw_text_mut(
            &mut img,
            image::Rgba { data: [0, 0, 0, 0] },
            0,
            0,
            rusttype::Scale::uniform(32.),
            &font,
            &format!(
                "{:.2} FPS",
                n_frame as f64 / start.elapsed().as_secs() as f64
            ),
        );
        opencv::show_image("result", &opencv::IplImage::from_image(img))?;
    }

    Ok(())
}
