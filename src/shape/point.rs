use qmath::prelude::*;
use qmath::vec2::QVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QPoint {
    pos: QVec2,
}

impl QPoint {
    pub fn new(pos: QVec2) -> Self {
        Self {
            pos
        }
    }

    pub fn new_from_parts(x: Q64, y: Q64) -> Self {
        Self {
            pos: QVec2::new(x, y)
        }
    }

    pub fn pos(&self) -> QVec2 {
        self.pos
    }

    pub fn distance(&self, other: &QPoint) -> Q64 {
        self.pos.distance(other.pos)
    }
}