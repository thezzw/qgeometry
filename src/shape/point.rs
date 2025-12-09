use qmath::prelude::*;
use qmath::vec2::QVec2;
use crate::algorithm::gjk;
use super::{ QBbox, QPolygon, QShapeCommon, QShapeType };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QPoint {
    pos: QVec2,
}

impl QPoint {
    pub const ZERO: Self = Self::new(QVec2::ZERO);

    pub const fn new(pos: QVec2) -> Self {
        Self {
            pos
        }
    }

    pub const fn new_from_parts(x: Q64, y: Q64) -> Self {
        Self::new(QVec2::new(x, y))
    }

    pub fn pos(&self) -> QVec2 {
        self.pos
    }

    pub fn x(&self) -> Q64 {
        self.pos.x
    }

    pub fn y(&self) -> Q64 {
        self.pos.y
    }

    pub fn distance(&self, other: &QPoint) -> Q64 {
        self.pos.distance(other.pos)
    }
}

impl QShapeCommon for QPoint {
    fn points(&self) -> Vec<QPoint> {
        vec![*self]
    }

    fn get_bbox(&self) -> QBbox {
        QBbox::new_from_parts(self.pos - QVec2::EPS, self.pos + QVec2::EPS)
    }

    fn get_centroid(&self) -> QPoint {
        *self
    }

    fn get_shape_type(&self) -> QShapeType {
        QShapeType::QPoint
    }

    fn is_point_inside(&self, point: &QPoint) -> bool {
        self == point
    }

    fn is_collide(&self, other: &impl QShapeCommon) -> bool {
        let other_shape_type = other.get_shape_type();
        match other_shape_type {
            _ => {
                let my_polygon = QPolygon::new(self.points());
                let other_polygon = QPolygon::new(other.points());
                gjk(&my_polygon, &other_polygon)
            }
        }
    }
}