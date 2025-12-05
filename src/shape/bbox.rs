use qmath::prelude::*;
use qmath::vec2::QVec2;

#[derive(Debug, Clone, Copy)]
pub struct QBbox {
    pos: QVec2,
    width: Q64,
    height: Q64,
}

impl QBbox {
    pub fn new(pos: QVec2, width: Q64, height: Q64) -> Self {
        Self {
            pos,
            width,
            height,
        }
    }

    pub fn pos(&self) -> QVec2 {
        self.pos
    }

    pub fn width(&self) -> Q64 {
        self.width
    }

    pub fn height(&self) -> Q64 {
        self.height
    }
}