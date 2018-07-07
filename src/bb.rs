use rect;

pub struct Bb {
    pub y_min: f32,
    pub x_min: f32,
    pub y_max: f32,
    pub x_max: f32,
    pub label: usize,
    pub score: f32,
}

impl rect::Rect<f32> for Bb {
    fn y_min(&self) -> f32 {
        self.y_min
    }
    fn x_min(&self) -> f32 {
        self.x_min
    }
    fn y_max(&self) -> f32 {
        self.y_max
    }
    fn x_max(&self) -> f32 {
        self.x_max
    }
}
