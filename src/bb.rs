use std::cmp;

use rect::Rect;

pub struct Bb {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub label: usize,
    pub score: f32,
}

impl Rect<f32> for Bb {
    fn top(&self) -> f32 {
        self.top
    }
    fn left(&self) -> f32 {
        self.left
    }
    fn bottom(&self) -> f32 {
        self.bottom
    }
    fn right(&self) -> f32 {
        self.right
    }
}

pub fn non_maximum_suppression(bbox: &mut Vec<Bb>, thresh: f32) {
    bbox.sort_unstable_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(cmp::Ordering::Equal)
    });
    retain_with_prev(bbox, |b, s| {
        s.iter().all(|s| b.label != s.label || b.iou(s) < thresh)
    });
}

fn retain_with_prev<T, F>(v: &mut Vec<T>, f: F)
where
    F: Fn(&T, &[T]) -> bool,
{
    let len = (0..v.len()).fold(0, |l, r| {
        v.swap(l, r);
        l + if f(&v[l], &v[0..l]) { 1 } else { 0 }
    });
    v.truncate(len);
}
