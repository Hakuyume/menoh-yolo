use image;
use num_traits;

use std::ffi;
use std::mem;
use std::ptr;

use rect;

mod sys;

use image::GenericImage;

pub struct IplImage {
    header: *mut sys::IplImage,
    data: Vec<u8>,
}

impl IplImage {
    pub fn from_image(image: image::DynamicImage) -> Self {
        let (height, width) = (image.height() as _, image.width() as _);
        match image {
            image::DynamicImage::ImageRgb8(mut image) => {
                for pixel in image.pixels_mut() {
                    pixel.data.reverse();
                }
                Self::new(height, width, 3, image.into_vec())
            }
            image::DynamicImage::ImageRgba8(mut image) => {
                for pixel in image.pixels_mut() {
                    pixel.data[..3].reverse();
                }
                Self::new(height, width, 4, image.into_vec())
            }
            // ImageLuma8 and ImageLumaA8
            _ => unreachable!(),
        }
    }

    pub fn into_image(mut self) -> image::DynamicImage {
        let header = unsafe { &(*self.header) };
        let (height, width) = (header.height as _, header.width as _);
        let mut data = Vec::new();
        mem::swap(&mut self.data, &mut data);
        match header.nChannels as _ {
            3 => {
                let mut image = image::RgbImage::from_raw(height, width, data).unwrap();
                for pixel in image.pixels_mut() {
                    pixel.data.reverse();
                }
                image::DynamicImage::ImageRgb8(image)
            }
            4 => {
                let mut image = image::RgbaImage::from_raw(height, width, data).unwrap();
                for pixel in image.pixels_mut() {
                    pixel.data[..3].reverse();
                }
                image::DynamicImage::ImageRgba8(image)
            }
            _ => unreachable!(),
        }
    }

    fn new(height: usize, width: usize, channels: usize, mut data: Vec<u8>) -> Self {
        assert_eq!(data.len(), height * width * channels);
        let size = sys::CvSize {
            height: height as _,
            width: width as _,
        };
        let header = unsafe { sys::cvCreateImageHeader(size, 8, channels as _) };
        assert!(!header.is_null());
        unsafe { sys::cvSetData(header as _, data.as_mut_ptr() as _, (width * channels) as _) };
        Self { header, data }
    }

    unsafe fn empty(height: usize, width: usize, channels: usize) -> Self {
        let mut data = Vec::with_capacity(height * width * channels);
        data.set_len(height * width * channels);
        Self::new(height, width, channels, data)
    }

    fn as_arr(&self) -> *const sys::CvArr {
        self.header as _
    }

    fn as_arr_mut(&mut self) -> *mut sys::CvArr {
        self.header as _
    }
}

impl Drop for IplImage {
    fn drop(&mut self) {
        unsafe { sys::cvReleaseImageHeader(&mut self.header) }
    }
}

pub fn show_image(name: &str, image: &IplImage) -> Result<(), ffi::NulError> {
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

pub fn rectangle<T, R>(img: &mut IplImage,
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

pub struct Capture {
    capture: *mut sys::CvCapture,
}

impl Capture {
    pub fn open_camera(index: usize) -> Option<Self> {
        let capture = unsafe { sys::cvCreateCameraCapture(index as _) };
        if capture.is_null() {
            None
        } else {
            Some(Self { capture })
        }
    }

    pub fn query_frame(&mut self) -> Option<IplImage> {
        unsafe {
            let frame = sys::cvQueryFrame(self.capture).as_ref()?;
            let mut mat =
                IplImage::empty(frame.height as _, frame.width as _, frame.nChannels as _);
            sys::cvCopy(frame as *const _ as _, mat.as_arr_mut(), ptr::null_mut());
            Some(mat)
        }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        unsafe { sys::cvReleaseCapture(&mut self.capture) }
    }
}
