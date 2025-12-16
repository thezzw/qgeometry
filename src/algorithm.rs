use qmath::prelude::*;
use qmath::vec2::QVec2;
use qmath::dir::QDir;
use crate::prelude::*;

/// Get the simplex point of minkowski difference at giving direction.
pub fn _get_simplex_point_in_direction(shape_a: &QPolygon, shape_b: &QPolygon, dir: QDir) -> QPoint {
    let point_a = shape_a.get_farest_point_in_direction(dir);
    let point_b = shape_b.get_farest_point_in_direction(-dir);
    QPoint::new(point_a.pos().saturating_sub(point_b.pos()))
}

/// GJK algorithm.
/// 
/// This function checks if two convex polygons intersect using the Gilbert-Johnson-Keerthi algorithm.
/// It works by trying to build a simplex (in 2D, a triangle) around the origin using points from 
/// the Minkowski difference of the two shapes.
/// 
/// # Arguments
/// 
/// * `shape_a` - First polygon
/// * `shape_b` - Second polygon
/// 
/// # Returns
/// 
/// True if the polygons intersect, false otherwise
pub fn gjk(shape_a: &QPolygon, shape_b: &QPolygon) -> bool {
    let minkowski_difference = get_minkowski_difference(shape_a, shape_b);
    minkowski_difference.is_point_inside(&QPoint::ZERO)
}

/// EPA algorithm.
/// 
/// This function computes the penetration depth and direction between two convex polygons
/// that are known to be intersecting (using GJK).
/// 
/// # Arguments
/// 
/// * `shape_a` - First polygon
/// * `shape_b` - Second polygon
/// 
/// # Returns
/// 
/// Separation vector for shape_b (direction and magnitude of penetration)
pub fn epa(shape_a: &QPolygon, shape_b: &QPolygon) -> Option<QVec2> {
    let minkowski_difference = get_minkowski_difference(shape_a, shape_b);
    if minkowski_difference.is_point_inside(&QPoint::ZERO) {
        let nearest_lines_index = minkowski_difference.get_nearest_lines_index_to_point(&QPoint::ZERO);
        assert!(nearest_lines_index.len() >= 2, "[algorithm::epa] Nearest lines index must have at least 2 elements, shape_a: {:?}, shape_b: {:?}, minksowski_difference: {:?}", shape_a, shape_b, minkowski_difference);
        let line = QLine::new(minkowski_difference.points()[nearest_lines_index[0]], minkowski_difference.points()[nearest_lines_index[1]]);
        return Some(line.get_perpendicular_vector_from_point(&QPoint::ZERO));
    }
    None
}

/// Calculate the Minkowski difference of two convex polygons.
/// 
/// The Minkowski difference of two shapes A and B is defined as the set of all points a - b
/// where a is in A and b is in B. For convex polygons, this can be computed efficiently by
/// taking the Minkowski sum of A and -B.
/// 
/// # Arguments
/// 
/// * `shape_a` - First polygon
/// * `shape_b` - Second polygon
/// 
/// # Returns
/// 
/// A new polygon representing the Minkowski difference
pub fn get_minkowski_difference(shape_a: &QPolygon, shape_b: &QPolygon) -> QPolygon {
    let mut all_diff_points = vec![];
    shape_a.points().iter().for_each(|pa|
        shape_b.points().iter().for_each(|pb|
            all_diff_points.push(pa.pos().saturating_sub(pb.pos()))
        )
    );
    QPolygon::new_from_parts(andrew_graham_scan(&all_diff_points))
}

/// Andrew's monotone chain convex hull algorithm.
/// 
/// # Returns
/// 
/// The convex hull of the points.
pub fn andrew_graham_scan(points: &Vec<QVec2>) -> Vec<QVec2> {
    use std::collections::HashSet;
    let mut unique_points: Vec<QVec2> = points.into_iter().collect::<HashSet<_>>().into_iter().map(|p| *p).collect();

    let n = unique_points.len();
    if n <= 2 {
        return unique_points;
    }

    // Sort points lexicographically (first by x, then by y)
    unique_points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });

    /// Computes the 2D cross product of OA and OB vectors, i.e. z-component of their 3D cross product.
    /// Returns a positive value, if OAB makes a counter-clockwise turn,
    /// negative for clockwise turn, and zero if the points are collinear.
    fn cross(o: &QVec2, a: &QVec2, b: &QVec2) -> Q64 {
        (a.saturating_sub(*o)).cross(b.saturating_sub(*o))
    }

    // Build lower hull
    let mut lower = Vec::with_capacity(n);
    for p in &unique_points {
        while lower.len() >= 2 && cross(&lower[lower.len()-2], &lower[lower.len()-1], p) <= Q64::ZERO {
            lower.pop();
        }
        lower.push(*p);
    }

    // Build upper hull
    let mut upper = Vec::with_capacity(n);
    for p in unique_points.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len()-2], &upper[upper.len()-1], p) <= Q64::ZERO {
            upper.pop();
        }
        upper.push(*p);
    }

    // Remove last point of each half because it's repeated
    lower.pop();
    upper.pop();

    // Concatenate lower and upper hull
    lower.extend(upper);
    lower
}
