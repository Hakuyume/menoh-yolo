use image;
use std::ffi;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn cv_imshow(winname: *const c_char, rows: c_int, cols: c_int, data: *const u8);
    fn cv_waitKey(delay: c_int) -> c_int;
}

pub fn imshow(winname: &str, mat: &image::DynamicImage) -> Result<(), ffi::NulError> {
    let winname = ffi::CString::new(winname)?;
    let mut mat = mat.to_rgb();
    let (rows, cols) = (mat.height() as _, mat.width() as _);
    for pixel in mat.pixels_mut() {
        pixel.data.reverse();
    }
    let data = mat.into_vec();
    unsafe { cv_imshow(winname.as_ptr(), rows, cols, data.as_ptr()) }
    Ok(())
}

pub fn wait_key(delay: Option<usize>) -> Option<char> {
    let code = unsafe { cv_waitKey(delay.unwrap_or(0) as _) };
    if code >= 0 {
        Some(code as u8 as _)
    } else {
        None
    }
}
