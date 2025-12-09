use qmath::prelude::*;
use qmath::vec2::QVec2;
use crate::algorithm::gjk;
use super::{ QPoint, QPolygon, QShapeCommon, QShapeType };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QBbox {
    left_bottom: QPoint,
    right_top: QPoint,
}

impl QBbox {
    pub fn new(left_bottom: QPoint, right_top: QPoint) -> Self {
        assert!(
            left_bottom.x() < right_top.x() && left_bottom.y() < right_top.y(),
            "[QBbox::new] right_top({right_top:?}) should be larger than left_bottom({left_bottom:?})."
        );
        Self {
            left_bottom,
            right_top,
        }
    }

    pub fn new_from_parts(left_bottom: QVec2, right_top: QVec2) -> Self {
        Self::new(QPoint::new(left_bottom), QPoint::new(right_top))
    }

    pub fn left_bottom(&self) -> QPoint {
        self.left_bottom
    }

    pub fn right_top(&self) -> QPoint {
        self.right_top
    }

    pub fn width(&self) -> Q64 {
        self.right_top.x().saturating_sub(self.left_bottom.x())
    }

    pub fn height(&self) -> Q64 {
        self.right_top.y().saturating_sub(self.left_bottom.y())
    }
}

impl QShapeCommon for QBbox {
    fn points(&self) -> Vec<QPoint> {
        vec![
            self.left_bottom,
            QPoint::new_from_parts(self.left_bottom.x(), self.right_top.y()),
            self.right_top,
            QPoint::new_from_parts(self.right_top.x(), self.left_bottom.y())
        ]
    }

    fn get_bbox(&self) -> QBbox {
        *self
    }

    fn get_centroid(&self) -> QPoint {
        QPoint::new(self.left_bottom.pos().midpoint(self.right_top.pos()))
    }

    fn get_shape_type(&self) -> QShapeType {
        QShapeType::QBbox
    }

    fn is_point_inside(&self, point: &QPoint) -> bool {
        if point.x() < self.left_bottom.x() { return false; }
        if point.x() > self.right_top.x() { return false; }
        if point.y() < self.left_bottom.y() { return false; }
        if point.y() > self.right_top.y() { return false; }
        true
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