use docopt;
use image;
use std::error;
use std::path;

use LABEL_NAMES;
use draw;
use yolo_v2;

const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo <src> <dest>
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_src: path::PathBuf,
    arg_dest: path::PathBuf,
}

pub fn main_() -> Result<(), Box<dyn(error::Error)>> {
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
