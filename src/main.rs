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

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", 20, "mkldnn", "")?;

    let mut img = image::open(args.arg_src)?;
    let bbox = model.predict(&img)?;
    for bb in bbox.iter() {
        bb::draw(&mut img, bb, 2);
        println!("{} {}", bb.label, bb.score);
    }
    img.save(args.arg_dest)?;

    Ok(())
}
