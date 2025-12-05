use qmath::prelude::*;
use qmath::vec2::QVec2;

#[derive(Debug, Clone, Copy)]
pub struct QCircle {
    pos: QVec2,
    radius: Q64,
}

impl QCircle {
    pub fn new(pos: QVec2, radius: Q64) -> Self {
        Self {
            pos,
            radius,
        }
    }

    pub fn pos(&self) -> QVec2 {
        self.pos
    }

    pub fn radius(&self) -> Q64 {
        self.radius
    }
}