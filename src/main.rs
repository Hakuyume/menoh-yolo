extern crate docopt;
extern crate image;
extern crate menoh;
#[macro_use]
extern crate ndarray;
extern crate num_traits;
#[macro_use]
extern crate serde_derive;

use std::error;

mod bb;
#[cfg(not(feature = "opencv"))]
mod draw;
mod model_ext;
#[cfg(feature = "opencv")]
mod opencv;
mod partial_cmp;
mod rect;
mod yolo_v2;

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

#[cfg(feature = "opencv")]
mod main_with_opencv;
#[cfg(feature = "opencv")]
fn main() -> Result<(), Box<dyn(error::Error)>> {
    main_with_opencv::main_()
}

#[cfg(not(feature = "opencv"))]
mod main_without_opencv;
#[cfg(not(feature = "opencv"))]
fn main() -> Result<(), Box<dyn(error::Error)>> {
    main_without_opencv::main_()
}
