pub use glam;
use glam::Vec2;
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    #[must_use]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    #[must_use]
    pub fn intersect(self, other: Self) -> Option<Self> {
        let x1 = self.x;
        let y1 = self.y;
        let w1 = self.w;
        let h1 = self.h;
        let x2 = other.x;
        let y2 = other.y;
        let w2 = other.w;
        let h2 = other.h;
        let x = x1.max(x2);
        let y = y1.max(y2);
        let w = (x1 + w1).min(x2 + w2) - x;
        let h = (y1 + h1).min(y2 + h2) - y;
        if w < 0. || h < 0. {
            None
        } else {
            Some(Self::new(x, y, w, h))
        }
    }

    #[must_use]
    pub fn center(self) -> Vec2 {
        Vec2::new(self.x + self.w / 2., self.y + self.h / 2.)
    }
}
