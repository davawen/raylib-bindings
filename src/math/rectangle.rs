pub use crate::ffi::Rectangle;
use crate::prelude::Vector2;

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle { x, y, width, height }
    }

    pub const fn from_vecs(pos: Vector2, size: Vector2) -> Self {
        Rectangle::new(pos.x, pos.y, size.x, size.y)
    }

    /// Creates a new rectangle from the given points.
    /// Flips signs where necessary if `bot_right` is above or to the left of `top_left`.
    pub fn from_points(mut top_left: Vector2, bot_right: Vector2) -> Self {
        let mut size = bot_right - top_left;
        if size.x < 0.0 {
            size.x *= -1.0;
            top_left.x = bot_right.x;
        }
        if size.y < 0.0 {
            size.y *= -1.0;
            top_left.y = bot_right.y;
        }
        Rectangle::from_vecs(top_left, size)
    }

    /// Checks wether this rectangle has 0 width and 0 height
    pub fn is_empty(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }
}
