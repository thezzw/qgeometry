use qmath::prelude::*;
use qmath::vec2::QVec2;
use qmath::dir::QDir;
use crate::algorithm::{epa, gjk};
use super::{ QPoint, QBbox, QPolygon, QShapeCommon, QShapeType };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QLine {
    start: QPoint,
    end: QPoint,
}

impl QLine {
    pub fn new(start: QPoint, end: QPoint) -> Self {
        assert!(start != end, "[QLine::new] start({start:?}) should not be equal to end({end:?}).");
        Self {
            start,
            end,
        }
    }

    pub fn new_from_parts(start: QVec2, end: QVec2) -> Self {
        Self::new(QPoint::new(start), QPoint::new(end))
    }

    pub fn new_from_zero(end: QPoint) -> Self {
        Self {
            start: QPoint::new(QVec2::ZERO),
            end,
        }
    }

    pub fn start(&self) -> QPoint {
        self.start
    }

    pub fn end(&self) -> QPoint {
        self.end
    }

    pub fn start_mut(&mut self) -> &mut QPoint {
        &mut self.start
    }

    pub fn end_mut(&mut self) -> &mut QPoint {
        &mut self.end
    }

    pub fn set_start(&mut self, start: QPoint) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: QPoint) {
        self.end = end;
    }

    pub fn vector(&self) -> QVec2 {
        self.end.pos().saturating_sub(self.start.pos())
    }

    /// Is the angle between pa and pb greater than 90 degrees.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let pa = QPoint::new(qvec2!(-1.0, -1.0));
    /// let pb = QPoint::new(qvec2!(1.0, 1.0));
    /// let pc = QPoint::new(qvec2!(-1.0, 1.0));
    /// let la = QLine::new_from_zero(pa);
    /// let lb = QLine::new_from_zero(pb);
    /// let lc = QLine::new_from_zero(pc);
    /// assert!(la.is_obtuse(&lb));
    /// assert!(!la.is_obtuse(&lc));
    /// ```
    pub fn is_obtuse(&self, other: &QLine) -> bool {
        self.vector().dot(other.vector()) < 0
    }

    pub fn get_perpendicular_dir(&self) -> QDir {
        let start_pos = self.start.pos();
        let end_pos = self.end.pos();
        QDir::new_from_vec(QVec2::new(end_pos.y.saturating_sub(start_pos.y), -(end_pos.x.saturating_sub(start_pos.x))))
    }

    /// Get perpendicular direction towards the origin.
    pub fn get_perpendicular_dir_to_origin(&self) -> QDir {
        let rst = self.get_perpendicular_dir();
        if rst.to_vec().dot(-self.start.pos()) >= 0 { rst } else { -rst }
    }

    /// Get the perpendicular vector from a point.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let pa = QPoint::new(qvec2!(-1.0, -1.0));
    /// let pb = QPoint::new(qvec2!(1.0, 1.0));
    /// let pc = QPoint::new(qvec2!(-1.0, 1.0));
    /// let la = QLine::new_from_zero(pa);
    /// let lb = QLine::new_from_zero(pb);
    /// let lc = QLine::new_from_zero(pc);
    /// assert!(la.get_perpendicular_vector_from_point(&pc) == QVec2::new(Q64::ONE, Q64::NEG_ONE));
    /// assert!(la.get_perpendicular_vector_from_point(&pb) == QVec2::ZERO);
    /// assert!(la.get_perpendicular_vector_from_point(&pa) == QVec2::ZERO);
    /// assert!(lb.get_perpendicular_vector_from_point(&pc) == QVec2::new(Q64::ONE, Q64::NEG_ONE));
    /// assert!(lb.get_perpendicular_vector_from_point(&pa) == QVec2::ZERO);
    /// assert!(lb.get_perpendicular_vector_from_point(&pb) == QVec2::ZERO);
    /// assert!(lc.get_perpendicular_vector_from_point(&pc) == QVec2::ZERO);
    /// assert!(lc.get_perpendicular_vector_from_point(&pa) == QVec2::ONE);
    /// assert!(lc.get_perpendicular_vector_from_point(&pb) == QVec2::NEG_ONE);
    /// ```
    pub fn get_perpendicular_vector_from_point(&self, point: &QPoint) -> QVec2 {
        let vector_ab = self.end.pos().saturating_sub(self.start.pos());
        let vector_ap = point.pos().saturating_sub(self.start.pos());

        let t: Q64 = vector_ap.dot(vector_ab).saturating_div(vector_ab.dot(vector_ab));
        self.start.pos().saturating_sub(point.pos()).saturating_add(vector_ab.saturating_mul_num(t))
    }

    /// Get the perpendicular distance from a point.
    pub fn get_perpendicular_distance_from_point(&self, point: &QPoint) -> Q64 {
        self.get_perpendicular_vector_from_point(point).length()
    }

    /// Get the distance from a point.
    /// 
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let px = QPoint::new(QVec2::X);
    /// let pa = QPoint::new(QVec2::ZERO);
    /// let pb = QPoint::new(QVec2::Y);
    /// let lab = QLine::new(pa, pb);
    /// assert!(lab.get_distance_from_point(&px) == Q64::ONE);
    /// assert!(lab.get_distance_from_point(&pa) == Q64::ZERO);
    /// assert!(lab.get_distance_from_point(&pb) == Q64::ZERO);
    /// ```
    pub fn get_distance_from_point(&self, point: &QPoint) -> Q64 {
        if self.start.pos() == self.end.pos() {
            return self.start.distance(point);
        }

        let segment_vector = self.vector();
        let segment_length_squared = segment_vector.length_squared();
        if segment_length_squared == 0 {
            return self.start.distance(point);
        }

        let point_vector = point.pos().saturating_sub(self.start.pos());
        let projection = point_vector.dot(segment_vector);
        let t = projection / segment_length_squared;

        if t < Q64::ZERO {
            self.start.distance(point)
        } else if t > Q64::ONE {
            self.end.distance(point)
        } else {
            self.get_perpendicular_distance_from_point(point)
        }
    }

    /// Is the point on the line.
    /// Margin error is Q64::ZERO.
    /// 
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let pa = QPoint::new(QVec2::NEG_ONE);
    /// let pb = QPoint::new(QVec2::ONE);
    /// let lab = QLine::new(pa, pb);
    /// assert!(lab.is_point_on_line(&pb));
    /// 
    /// let pc = QPoint::new(qvec2!(-2.0, -2.0));
    /// assert!(!lab.is_point_on_line(&pc));
    /// 
    /// let pd = QPoint::new(QVec2::ONE / q64!(3));
    /// assert!(lab.is_point_on_line(&pd));
    /// 
    /// let pe = QPoint::new(qvec2!(0.5, 0.6));
    /// let lb = QLine::new_from_zero(pb);
    /// assert!(!lb.is_point_on_line(&pe));
    /// 
    /// let pf = QPoint::new(QVec2::ONE + QVec2::EPS);
    /// assert!(!lab.is_point_on_line(&pf));
    /// 
    /// let pg = QPoint::new(QVec2::MAX);
    /// let ph = QPoint::new(QVec2::MIN);
    /// let lgh = QLine::new(pg, ph);
    /// assert!(lgh.is_point_on_line(&pg));
    /// assert!(lgh.is_point_on_line(&ph));
    /// assert!(!lab.is_point_on_line(&pg));
    /// assert!(!lab.is_point_on_line(&ph));
    /// ```
    pub fn is_point_on_line(&self, point: &QPoint) -> bool {
        let ab = self.end.pos().saturating_sub(self.start.pos());
        let ap = point.pos().saturating_sub(self.start.pos());

        let cross = ap.cross(ab);
        if cross != 0 { return false; }

        let dot_product = ap.dot(ab);
        let squared_length = ab.length_squared();
        dot_product >= 0 && dot_product <= squared_length
    }

    pub fn get_x_at_y(&self, y: Q64) -> Q64 {
        let vi = self.start.pos();
        let vj = self.end.pos();
        if vj.y == vi.y { return vi.x; }
        vj.x.saturating_sub(vi.x).saturating_mul(
            y.saturating_sub(vi.y).saturating_div(vj.y.saturating_sub(vi.y))
        ).saturating_add(vi.x)
    }

    pub fn get_y_at_x(&self, x: Q64) -> Q64 {
        let vi = self.start.pos();
        let vj = self.end.pos();
        if vj.x == vi.x { return vi.y; }
        vj.y.saturating_sub(vi.y).saturating_mul(
            x.saturating_sub(vi.x).saturating_div(vj.x.saturating_sub(vi.x))
        ).saturating_add(vi.y)
    }
}

impl QShapeCommon for QLine {
    fn points(&self) -> Vec<QPoint> {
        vec![self.start, self.end]
    }

    fn get_bbox(&self) -> QBbox {
        let mut left_bottom = self.start.pos().min(self.end.pos());
        let mut right_top = self.start.pos().max(self.end.pos());
        if left_bottom.x == right_top.x {
            left_bottom.x = left_bottom.x.saturating_sub(Q64::EPS);
            right_top.x = right_top.x.saturating_add(Q64::EPS);
        }
        if left_bottom.y == right_top.y {
            left_bottom.y = left_bottom.y.saturating_sub(Q64::EPS);
            right_top.y = right_top.y.saturating_add(Q64::EPS);
        }
        QBbox::new_from_parts(left_bottom, right_top)
    }

    fn get_centroid(&self) -> QPoint {
        QPoint::new(self.start.pos().midpoint(self.end.pos()))
    }

    fn get_shape_type(&self) -> QShapeType {
        QShapeType::QLine
    }

    fn is_point_inside(&self, point: &QPoint) -> bool {
        self.is_point_on_line(point)
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

    fn try_get_seperation_vector(&self, other: &impl QShapeCommon) -> Option<QVec2> {
        let other_shape_type = other.get_shape_type();
        match other_shape_type {
            _ => {
                let my_polygon = QPolygon::new(self.points());
                let other_polygon = QPolygon::new(other.points());
                epa(&my_polygon, &other_polygon)
            }
        }
    }
}