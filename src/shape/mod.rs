pub mod point;
pub mod line;
pub mod bbox;
pub mod circle;
pub mod polygon;

pub use point::QPoint;
pub use line::QLine;
pub use bbox::QBbox;
pub use circle::QCircle;
pub use polygon::QPolygon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QShapeType {
    QPoint,
    QLine,
    QBbox,
    QCircle,
    QPolygon,
}

pub trait QShapeCommon {
    fn points(&self) -> Vec<QPoint>;

    fn get_bbox(&self) -> QBbox;

    fn get_centroid(&self) -> QPoint;

    fn get_shape_type(&self) -> QShapeType;

    fn is_point_inside(&self, point: &QPoint) -> bool;

    fn is_collide(&self, other: &impl QShapeCommon) -> bool;

    fn get_polygon(&self) -> QPolygon {
        QPolygon::new(self.points().to_vec())
    }

    fn ear_clipping_triangulation(&self) -> Vec<usize> {
        self.get_polygon().ear_clipping_triangulation()
    }
}