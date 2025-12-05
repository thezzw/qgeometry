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

use qmath::vec2::QVec2;
pub trait ShapeCommon {
    fn points(&self) -> &Vec<QPoint>;
    fn get_centroid(&self) -> QPoint;
    fn get_farest_point_in_direction(&self, dir: QVec2) -> QPoint;
    fn is_point_inside(&self, point: &QPoint) -> bool;
    fn ear_clipping_triangulation(&self) -> Vec<usize>;
}