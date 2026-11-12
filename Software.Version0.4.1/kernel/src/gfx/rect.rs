//! Rectangulos.
use core::cmp::{max, min};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.right() && py >= self.y && py < self.bottom()
    }

    pub fn inflate(&self, d: i32) -> Rect {
        Rect::new(self.x - d, self.y - d, self.w + 2 * d, self.h + 2 * d)
    }

    pub fn union(&self, o: &Rect) -> Rect {
        let x0 = min(self.x, o.x);
        let y0 = min(self.y, o.y);
        let x1 = max(self.right(), o.right());
        let y1 = max(self.bottom(), o.bottom());
        Rect::new(x0, y0, x1 - x0, y1 - y0)
    }

    pub fn intersect(&self, o: &Rect) -> Option<Rect> {
        let x0 = max(self.x, o.x);
        let y0 = max(self.y, o.y);
        let x1 = min(self.right(), o.right());
        let y1 = min(self.bottom(), o.bottom());
        if x1 <= x0 || y1 <= y0 {
            None
        } else {
            Some(Rect::new(x0, y0, x1 - x0, y1 - y0))
        }
    }
}
