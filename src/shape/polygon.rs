use qmath::prelude::*;
use qmath::vec2::QVec2;
use qmath::dir::QDir;
use crate::algorithm::gjk;
use super::{ QPoint, QLine, QBbox, QShapeCommon, QShapeType };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QPolygon {
    points: Vec<QPoint>,
}

impl QPolygon {
    pub fn new(points: Vec<QPoint>) -> Self {
        Self {
            points,
        }
    }

    pub fn new_from_parts(points: Vec<QVec2>) -> Self {
        Self {
            points: points.into_iter().map(|pos| QPoint::new(pos)).collect(),
        }
    }

    /// Get the cloest line to the origin.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let p0 = QPoint::new(QVec2::ZERO);
    /// let shape = vec![qvec2!(0.0, 0.0), qvec2!(1.0, 0.0), qvec2!(1.0, 1.0)];
    /// let polygon = QPolygon::new_from_parts(shape);
    /// assert_eq!(polygon.get_nearest_lines_index_to_point(&p0), vec![2, 0, 0, 1]);
    /// 
    /// let shape_x = vec![qvec2!(-6, 4), qvec2!(2, -4), qvec2!(-6, -4)];
    /// let polygon_x = QPolygon::new_from_parts(shape_x);
    /// assert_eq!(polygon_x.get_nearest_lines_index_to_point(&p0), vec![0, 1]);
    /// ```
    pub fn get_nearest_lines_index_to_point(&self, point: &QPoint) -> Vec<usize> {
        let points_count = self.points.len();
        if points_count < 2 {
            return vec![];
        }
        if points_count == 2 {
            return vec![0, 1];
        }

        let mut rst = vec![];
        let mut min_dist: Q64 = Q64::MAX;
        let mut j = points_count - 1;

        for i in 0..points_count {
            let vj = self.points[j].pos();
            let vi = self.points[i].pos();
            let line = QLine::new(QPoint::new(vj), QPoint::new(vi));
            
            let line_distance = line.get_distance_from_point(point);
            if line_distance < min_dist {
                min_dist = line_distance;
                rst.clear();
                rst.push(j); rst.push(i);
            } else if line_distance == min_dist {
                rst.push(j); rst.push(i);
            }

            j = i;
        }
        rst
    }

    /// Get the first farest point of the shape in giving direction.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qmath::dir::QDir;
    /// use qgeometry::prelude::*;
    /// 
    /// let shape = vec![
    ///     QPoint::new(qvec2!(0.0, 0.0)),
    ///     QPoint::new(qvec2!(1.0, 0.0)),
    ///     QPoint::new(qvec2!(1.0, 1.0))
    /// ];
    /// let polygon = QPolygon::new(shape);
    /// let dir = QDir::new_from_vec(qvec2!(1.0, 1.0));
    /// let rst = polygon.get_farest_point_in_direction(dir);
    /// assert!(rst.pos() == qvec2!(1.0, 1.0));
    /// ```
    pub fn get_farest_point_in_direction(&self, dir: QDir) -> QPoint {
        *self.points
            .iter()
            .max_by(|a, b| {
                let dot_a: Q64 = a.pos().dot(dir.to_vec());
                let dot_b: Q64 = b.pos().dot(dir.to_vec());
                dot_a.partial_cmp(&dot_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("[get_farest_point_in_direction] Shape must not be empty.")
    }
}

impl QShapeCommon for QPolygon {
    fn points(&self) -> Vec<QPoint> {
        self.points.clone()
    }

    fn get_shape_type(&self) -> QShapeType {
        QShapeType::QPolygon
    }

    /// Get centroid of the shape.
    /// 
    /// Computes centroid using baseline offset method to prevent overflow when dealing with large coordinate values.
    /// 
    /// __Invalid when sum_diff overflow.__
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let shape_a = vec![
    ///     qvec2!(0.0, 0.0),
    ///     qvec2!(1.0, 0.0),
    ///     qvec2!(1.0, 1.0),
    ///     qvec2!(0.0, 1.0)
    /// ];
    /// let polygon = QPolygon::new_from_parts(shape_a);
    /// let rst = polygon.get_centroid();
    /// assert!(rst.pos() == qvec2!(0.5, 0.5));
    /// 
    /// let shape_b = vec![
    ///     QPoint::new(QVec2::MAX),
    ///     QPoint::new(QVec2::new(Q64::MAX, Q64::MIN)),
    ///     QPoint::new(QVec2::MIN),
    ///     QPoint::new(QVec2::new(Q64::MIN, Q64::MAX))
    /// ];
    /// let polygon = QPolygon::new(shape_b);
    /// let rst = polygon.get_centroid();
    /// assert!(rst.pos() != QVec2::ZERO);
    /// ```
    fn get_centroid(&self) -> QPoint {
        let n = self.points.len();
        if n == 0 { return QPoint::new(QVec2::ZERO); }

        let base_point = self.points[0].pos();
        let baseline_x = base_point.x;
        let baseline_y = base_point.y;
        let mut sum_diff_x = Q64::ZERO;
        let mut sum_diff_y = Q64::ZERO;
        for point in &self.points {
            sum_diff_x = sum_diff_x.saturating_add(point.x().saturating_sub(baseline_x));
            sum_diff_y = sum_diff_y.saturating_add(point.y().saturating_sub(baseline_y));
        }

        let sum_diff_avg_x = sum_diff_x.saturating_div(q64!(n));
        let sum_diff_avg_y = sum_diff_y.saturating_div(q64!(n));
        let centroid_x = baseline_x.saturating_add(sum_diff_avg_x);
        let centroid_y = baseline_y.saturating_add(sum_diff_avg_y);
        return QPoint::new_from_parts(centroid_x, centroid_y);
    }

    /// Return true if the point is inside the shape.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let shape = vec![
    ///     QPoint::new(qvec2!(0.0, 0.0)),
    ///     QPoint::new(qvec2!(1.0, 0.0)),
    ///     QPoint::new(qvec2!(1.0, 1.0)),
    ///     QPoint::new(qvec2!(0.0, 1.0))
    /// ];
    /// let polygon = QPolygon::new(shape);
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::splat(Q64::HALF))));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::splat(Q64::HALF) + Q64::EPS)));
    /// 
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::ZERO + Q64::EPS)));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::ONE - Q64::EPS)));
    /// 
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::HALF, Q64::ONE))));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::HALF, Q64::ZERO))));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::ZERO, Q64::HALF))));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::ONE, Q64::HALF))));
    /// 
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::ZERO)));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::ONE)));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::ZERO, Q64::ONE))));
    /// assert!(polygon.is_point_inside(&QPoint::new(QVec2::new(Q64::ONE, Q64::ZERO))));
    /// 
    /// assert!(!polygon.is_point_inside(&QPoint::new(QVec2::ONE + Q64::EPS)));
    /// assert!(!polygon.is_point_inside(&QPoint::new(QVec2::ZERO - Q64::EPS)));
    /// assert!(!polygon.is_point_inside(&QPoint::new(QVec2::MAX)));
    /// assert!(!polygon.is_point_inside(&QPoint::new(QVec2::MIN)));
    fn is_point_inside(&self, point: &QPoint) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut rst = false;
        let mut j = self.points.len() - 1;

        for i in 0..self.points.len() {
            let vi = self.points[i].pos();
            let vj = self.points[j].pos();

            let line = QLine::new(QPoint::new(vi), QPoint::new(vj));
            if line.is_point_on_line(point) {
                return true;
            }

            let py: Q64 = point.y();
            if (vi.y > py) != (vj.y > py) {
                let intersect_x: Q64 = line.get_x_at_y(py);
                let px: Q64 = point.x();
                if px < intersect_x {
                    rst = !rst;
                }
            }
            j = i;
        }
        rst
    }

    /// Ear clipping triangulation.
    /// 
    /// Return the triangles' indices, these triangles' vertices are in CW order
    /// to aviod backface culling when camera's y is positive.
    /// # Examples
    /// ```
    /// use qmath::prelude::*;
    /// use qmath::vec2::QVec2;
    /// use qgeometry::prelude::*;
    /// 
    /// let shape_a = vec![
    ///     qvec2!(0.0, 0.0),
    ///     qvec2!(1.0, 0.0),
    ///     qvec2!(1.0, 1.0),
    ///     qvec2!(0.0, 1.0)
    /// ];
    /// let polygon = QPolygon::new_from_parts(shape_a);
    /// let triangles = polygon.ear_clipping_triangulation();
    /// assert!(triangles.len() == 6);
    /// ```
    fn ear_clipping_triangulation(&self) -> Vec<usize> {
        let shape = &self.points;
        fn is_valid_ear(shape: &Vec<QPoint>, a: &QPoint, b: &QPoint, c: &QPoint) -> bool {
            let cross_product: Q64 = (b.pos() - a.pos()).cross(c.pos() - b.pos());
            // Ear triangle's vertices need to be in CCW order.
            if cross_product == Q64::ZERO { return false; }
            for point in shape.iter() {
                if point != a && point != b && point != c && QPolygon::new(vec![*a, *b, *c]).is_point_inside(point) { return false; }
            }
            true
        }

        let mut points = shape.to_vec();
        let mut triangles_indices = Vec::new();
        let get_index = |v| shape.iter().position(|&p| p == v).unwrap();

        while points.len() > 3 {
            for i in 0..points.len() {
                let j = (i + 1) % points.len();
                let k = (i + 2) % points.len();

                let a = points[i];
                let b = points[j];
                let c = points[k];

                if is_valid_ear(shape, &a, &b, &c) {
                    triangles_indices.push(get_index(c));
                    triangles_indices.push(get_index(b));
                    triangles_indices.push(get_index(a));
                    points.remove(j);
                    break;
                }
            }
        }

        assert!(points.len() == 3);
        triangles_indices.push(get_index(points[2]));
        triangles_indices.push(get_index(points[1]));
        triangles_indices.push(get_index(points[0]));

        triangles_indices
    }

    fn get_bbox(&self) -> QBbox {
        unimplemented!()
    }

    fn is_collide(&self, other: &impl QShapeCommon) -> bool {
        let other_shape_type = other.get_shape_type();
        match other_shape_type {
            _ => {
                let other_polygon = QPolygon::new(other.points());
                gjk(self, &other_polygon)
            }
        }
    }
}