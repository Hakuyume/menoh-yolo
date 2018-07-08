use image;
use std::ffi;

mod sys;

pub struct Mat {
    mat: *mut sys::CvMat,
    data: Vec<u8>,
}

impl Mat {
    pub fn new(image: &image::DynamicImage) -> Self {
        let mut image = image.to_rgb();
        let (rows, cols) = (image.height() as _, image.width() as _);
        for pixel in image.pixels_mut() {
            pixel.data.reverse();
        }
        let mut data = image.into_vec();
        unsafe {
            let mat = sys::cvCreateMatHeader(rows, cols, sys::CV_MAT_TYPE_8UC3 as _);
            sys::cvSetData(mat, data.as_mut_ptr() as _, (cols * 3) as _);
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
