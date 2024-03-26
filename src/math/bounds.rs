use bevy::math::{IRect, IVec2};

/// Defines the bounds of an object in 2D space as offsets from a center point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl IBounds {
    #[allow(dead_code)]
    const ZERO: Self = Self::zero();

    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.right + self.left
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.bottom + self.top
    }

    #[inline]
    pub fn size(&self) -> IVec2 {
        IVec2::new(self.width(), self.height())
    }

    /// Returns the [IRect] formed by these bounds when placed at the given position.
    #[inline]
    pub fn at_pos(&self, pos: IVec2) -> IRect {
        IRect::from_corners(
            pos - IVec2::new(self.left, self.top),
            pos + IVec2::new(self.right, self.bottom),
        )
    }
}
