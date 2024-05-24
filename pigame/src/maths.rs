pub use glam;
use glam::Vec2;

/// 2D rectangle
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Width
    pub w: f32,
    /// Height
    pub h: f32,
}

impl Rect {
    /// Create a new rectangle
    #[must_use]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Intersect two rectangles. Returns `None` if they do not intersect. Otherwise, returns the
    /// rectangle that is the intersection of the two.
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

    /// Return the centre of the rectangle.
    #[must_use]
    pub fn centre(self) -> Vec2 {
        Vec2::new(self.x + self.w / 2., self.y + self.h / 2.)
    }
}
