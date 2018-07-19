use rect;

pub struct Bb {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub label: usize,
    pub score: f32,
}

impl rect::Rect<f32> for Bb {
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
