use qmath::prelude::*;
use qmath::vec2::QVec2;
use crate::algorithm::gjk;
use super::{ QPoint, QBbox, QPolygon, QShapeCommon, QShapeType };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QCircle {
    pos: QVec2,
    radius: Q64,
}

impl QCircle {
    pub fn new(pos: QVec2, radius: Q64) -> Self {
        assert!(radius > Q64::ZERO, "[QCircle::new] radius({radius:?}) should be larger than zero.");
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

impl QShapeCommon for QCircle {
    fn points(&self) -> Vec<QPoint> {
        let mut points = Vec::new();
        let num_points = 16;
        
        for i in 0..num_points {
            let angle: Q64 = q64!(i) / q64!(num_points) * Q64::TAU;
            let (sin, cos) = angle.sin_cos();
            let x = self.pos.x.saturating_add(self.radius.saturating_mul(cos));
            let y = self.pos.y.saturating_add(self.radius.saturating_mul(sin));
            points.push(QPoint::new(QVec2::new(x, y)));
        }

        points
    }

    fn get_bbox(&self) -> QBbox {
        QBbox::new_from_parts(self.pos.saturating_sub_num(self.radius), self.pos.saturating_add_num(self.radius))
    }

    fn get_centroid(&self) -> QPoint {
        QPoint::new(self.pos)
    }

    fn get_shape_type(&self) -> QShapeType {
        QShapeType::QCircle
    }

    fn is_point_inside(&self, point: &QPoint) -> bool {
        self.pos.distance_squared(point.pos()) <= self.radius.saturating_mul(self.radius)
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