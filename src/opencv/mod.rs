use image;
use num_traits;

use std::ffi;

use rect;

mod sys;

use image::GenericImage;

pub struct Mat {
    mat: *mut sys::CvMat,
    _data: Option<Vec<u8>>,
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
            // ImageLuma8 and ImageLumaA8
            _ => unreachable!(),
        };
        unsafe {
            let mat = sys::cvCreateMatHeader(rows as _, cols as _, type_ as _);
            sys::cvSetData(mat as _, data.as_mut_ptr() as _, step as _);
            Self {
                mat,
                _data: Some(data),
            }
        }
    }

    fn as_arr(&self) -> *const sys::CvArr {
        self.mat as _
    }

    fn as_arr_mut(&mut self) -> *mut sys::CvArr {
        self.mat as _
    }
}


impl Drop for Mat {
    fn drop(&mut self) {
        unsafe { sys::cvReleaseMat(&mut self.mat) }
    }
}

pub fn show_image(name: &str, image: &Mat) -> Result<(), ffi::NulError> {
    let name = ffi::CString::new(name)?;
    unsafe { sys::cvShowImage(name.as_ptr(), image.as_arr()) }
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

pub fn rectangle<T, R>(img: &mut Mat,
                       rect: &R,
                       color: &[u8; 4],
                       thickness: Option<usize>)
                       -> Option<()>
    where T: num_traits::ToPrimitive,
          R: rect::Rect<T>
{
    let pt1 = sys::CvPoint {
        y: rect.y_min().to_i32()?,
        x: rect.x_min().to_i32()?,
    };
    let pt2 = sys::CvPoint {
        y: rect.y_max().to_i32()?,
        x: rect.x_max().to_i32()?,
    };
    let color = sys::CvScalar { val: [color[1] as _, color[1] as _, color[0] as _, color[3] as _] };
    let thickness = match thickness {
        Some(t) => t as _,
        None => -1,
    };
    unsafe { sys::cvRectangle(img.as_arr_mut(), pt1, pt2, color, thickness, 8, 0) }
    Some(())
}
