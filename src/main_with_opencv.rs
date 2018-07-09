use docopt;
use image;
use menoh;
use std::error;
use std::path;

use LABEL_NAMES;
use opencv;
use rect;
use yolo_v2;

const USAGE: &'static str = r#"
YOLO on Menoh

Usage: menoh-yolo [<image>]
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_image: Option<path::PathBuf>,
}

pub fn main_() -> Result<(), Box<dyn(error::Error)>> {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mut model = yolo_v2::YOLOv2::from_onnx("YOLOv2.onnx", LABEL_NAMES.len(), "mkldnn", "")?;
    let font = opencv::Font::new(1., 2);

    let mut predict = |img| -> Result<_, menoh::Error> {
        let bbox = model.predict(&img)?;
        let mut img = opencv::IplImage::from_image(img);
        for bb in bbox.iter() {
            opencv::rectangle(&mut img, bb, &[255, 0, 0, 0], Some(3));
            let text = format!("{}: {:.2}", LABEL_NAMES[bb.label], bb.score);
            let ((h, w), _) = opencv::get_text_size(&text, &font).unwrap();
            opencv::rectangle(&mut img,
                              &TLBR(bb.y_min - h as f32 - 5.,
                                    bb.x_min - 5.,
                                    bb.y_min + 5.,
                                    bb.x_min + w as f32 + 5.),
                              &[255, 255, 255, 0],
                              None);
            opencv::put_text(&mut img, &text, (bb.y_min, bb.x_min), &font, &[0, 0, 0, 0]);
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

struct TLBR<T>(T, T, T, T);

impl<T> rect::Rect<T> for TLBR<T>
    where T: Copy
{
    fn y_min(&self) -> T {
        self.0
    }
    fn x_min(&self) -> T {
        self.1
    }
    fn y_max(&self) -> T {
        self.2
    }
    fn x_max(&self) -> T {
        self.3
    }
}
