use image;
use imageproc;
use rusttype;
use std::fmt;
use std::ops;

use bb;

use rect::Rect;

pub fn get_font() -> Result<rusttype::Font<'static>, rusttype::Error> {
    rusttype::Font::from_bytes(include_bytes!("../opensans/ttfs/OpenSans-Regular.ttf") as &[u8])
}

pub fn draw_bbox_mut<'a, I, B, L, S>(image: &mut I, bbox: B, label_names: &L, font: &rusttype::Font)
where
    I: image::GenericImage<Pixel = image::Rgba<u8>>,
    B: Iterator<Item = &'a bb::Bb>,
    L: ?Sized + ops::Index<usize, Output = S>,
    S: fmt::Display,
{
    let scale = 32.;

    for bb in bbox {
        let text = format!("{}: {:.2}", label_names[bb.label], bb.score);
        let text_width =
            font.layout(
                &text,
                rusttype::Scale::uniform(scale),
                rusttype::Point { x: 0., y: 0. },
            ).filter_map(|l| l.pixel_bounding_box())
            .last()
            .unwrap()
            .max
            .x + 2;
        imageproc::drawing::draw_filled_rect_mut(
            image,
            imageproc::rect::Rect::at(bb.left() as _, (bb.top() - scale) as _)
                .of_size(text_width as _, scale as _),
            image::Rgba {
                data: [255, 255, 255, 0],
            },
        );
        imageproc::drawing::draw_text_mut(
            image,
            image::Rgba { data: [0, 0, 0, 0] },
            bb.left() as _,
            (bb.top() - scale) as _,
            rusttype::Scale::uniform(scale),
            &font,
            &text,
        );
        draw_hollow_rect_mut(
            image,
            imageproc::rect::Rect::at(bb.left() as _, bb.top() as _)
                .of_size(bb.width() as _, bb.height() as _),
            image::Rgba {
                data: [255, 0, 0, 0],
            },
            5,
        );
    }
}

fn draw_line_segment_mut<I>(
    image: &mut I,
    start: (f32, f32),
    end: (f32, f32),
    color: I::Pixel,
    thickness: f32,
) where
    I: image::GenericImage,
    I::Pixel: 'static,
{
    let line_iterator = imageproc::drawing::BresenhamLineIter::new(start, end);
    for point in line_iterator {
        imageproc::drawing::draw_filled_circle_mut(
            image,
            (point.0 as _, point.1 as _),
            (thickness / 2.) as _,
            color,
        );
    }
}

fn draw_hollow_rect_mut<I>(
    image: &mut I,
    rect: imageproc::rect::Rect,
    color: I::Pixel,
    thickness: u32,
) where
    I: image::GenericImage,
    I::Pixel: 'static,
{
    let left = rect.left() as _;
    let right = rect.right() as _;
    let top = rect.top() as _;
    let bottom = rect.bottom() as _;
    let thickness = thickness as _;

    draw_line_segment_mut(image, (left, top), (right, top), color, thickness);
    draw_line_segment_mut(image, (left, bottom), (right, bottom), color, thickness);
    draw_line_segment_mut(image, (left, top), (left, bottom), color, thickness);
    draw_line_segment_mut(image, (right, top), (right, bottom), color, thickness);
}
