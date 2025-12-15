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
    QPolygon::new_from_parts(graham_scan(&all_diff_points))
}

/// Graham scan algorithm.
/// 
/// # Returns
/// 
/// The convex hull of the points.
pub fn graham_scan(points: &Vec<QVec2>) -> Vec<QVec2> {
    use std::collections::HashSet;
    let mut unique_points: Vec<QVec2> = points.into_iter().collect::<HashSet<_>>().into_iter().map(|p| *p).collect();

    if unique_points.len() < 3 {
        return unique_points;
    }

    // Find the point with the lowest y-coordinate (and leftmost if there are ties)\
    use std::cmp::Ordering;
    unique_points.sort_unstable_by(|a, b| {
        let y_rst = a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal);
        if y_rst != Ordering::Equal {
            y_rst
        } else {
            a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal)
        }
    });
    let pivot = unique_points[0];

    // Sort the points by polar angle with respect to the pivot
    unique_points.sort_by(|a: &QVec2, b| {
        if *b == pivot { return Ordering::Greater; }
        if *a == pivot { return Ordering::Less; }

        let angle_bp: Q64 = b.saturating_sub(pivot).to_angle();
        let angle_ap: Q64 = a.saturating_sub(pivot).to_angle();
        let angle_diff: Q64 = angle_bp.saturating_sub(angle_ap);
        if angle_diff < Q64::ZERO {
            Ordering::Less
        } else if angle_diff > Q64::ZERO {
            Ordering::Greater
        } else {
            // If the angles are equal, compare distances
            a.saturating_sub(pivot).length_squared().partial_cmp(&b.saturating_sub(pivot).length_squared()).unwrap_or(Ordering::Equal)
        }
    });

    // Initialize the stack with the first two points
    let mut stack = Vec::with_capacity(unique_points.len());
    stack.push(unique_points[0]);
    stack.push(unique_points[1]);

    // Process the remaining points
    for i in 2..unique_points.len() {
        while stack.len() >= 2 {
            let next_to_top = stack[stack.len() - 2];
            let top = stack[stack.len() - 1];
            if (unique_points[i] - next_to_top).cross(top - next_to_top) > Q64::ZERO {
                break;
            }
            stack.pop();
        }
        stack.push(unique_points[i]);
    }

    stack
}