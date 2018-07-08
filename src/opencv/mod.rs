use image;
use std::ffi;

mod sys;

use image::GenericImage;

pub struct Mat {
    mat: *mut sys::CvMat,
    data: Vec<u8>,
}

impl Mat {
    pub fn from_image(image: image::DynamicImage) -> Self {
        let (rows, cols) = (image.height(), image.width());
        let (type_, mut data, step) = match image {
            image::DynamicImage::ImageRgb8(mut image) => {
                for pixel in image.pixels_mut() {
                    pixel.data.reverse();
                }
                (sys::CV_MAT_TYPE_8UC3, image.into_vec(), cols * 3)
            }
            image::DynamicImage::ImageRgba8(mut image) => {
                for pixel in image.pixels_mut() {
                    pixel.data[..3].reverse();
                }
                (sys::CV_MAT_TYPE_8UC4, image.into_vec(), cols * 4)
            }
            _ => unreachable!(),
        };
        unsafe {
            let mat = sys::cvCreateMatHeader(rows as _, cols as _, type_ as _);
            sys::cvSetData(mat, data.as_mut_ptr() as _, step as _);
            Self { mat, data }
        }
    }
}


impl Drop for Mat {
    fn drop(&mut self) {
        unsafe { sys::cvReleaseMat(&mut self.mat) }
    }
}

pub fn show_image(name: &str, image: &Mat) -> Result<(), ffi::NulError> {
    let name = ffi::CString::new(name)?;
    unsafe { sys::cvShowImage(name.as_ptr(), image.mat) }
    Ok(())
}

pub fn wait_key(delay: Option<usize>) -> Option<char> {
    let code = unsafe { sys::cvWaitKey(delay.unwrap_or(0) as _) };
    if code >= 0 {
        Some(code as u8 as _)
    } else {
        None
    }
}
