use qmath::prelude::*;
use qmath::vec2::QVec2;
use qgeometry::prelude::*;
use qgeometry::algorithm::*;

#[test]
fn test_gjk_no_collision() {
    // Two squares that don't touch
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(2.0, 0.0),
        qvec2!(3.0, 0.0),
        qvec2!(3.0, 1.0),
        qvec2!(2.0, 1.0),
    ]);
    
    assert_eq!(gjk(&square1, &square2), false);
}

#[test]
fn test_gjk_collision() {
    // Two squares that overlap
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(0.5, 0.5),
        qvec2!(1.5, 0.5),
        qvec2!(1.5, 1.5),
        qvec2!(0.5, 1.5),
    ]);
    
    assert_eq!(gjk(&square1, &square2), true);
}

#[test]
fn test_gjk_touching() {
    // Two squares that touch at one point
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(1.0, 1.0),
        qvec2!(2.0, 1.0),
        qvec2!(2.0, 2.0),
        qvec2!(1.0, 2.0),
    ]);
    
    // Touching at a point should be considered a collision
    assert_eq!(gjk(&square1, &square2), true);
}

#[test]
fn test_gjk_triangle_collision() {
    // Triangle and square that intersect
    let triangle = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(0.5, 1.0),
    ]);
    
    let square = QPolygon::new_from_parts(vec![
        qvec2!(0.25, 0.25),
        qvec2!(0.75, 0.25),
        qvec2!(0.75, 0.75),
        qvec2!(0.25, 0.75),
    ]);
    
    assert_eq!(gjk(&triangle, &square), true);
}

#[test]
fn test_gjk_triangle_no_collision() {
    // Triangle and square that don't intersect
    let triangle = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(0.5, 1.0),
    ]);
    
    let square = QPolygon::new_from_parts(vec![
        qvec2!(2.0, 2.0),
        qvec2!(3.0, 2.0),
        qvec2!(3.0, 3.0),
        qvec2!(2.0, 3.0),
    ]);
    
    assert_eq!(gjk(&triangle, &square), false);
}

#[test]
fn test_gjk_contained() {
    // Square completely inside another square
    let outer_square = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(3.0, 0.0),
        qvec2!(3.0, 3.0),
        qvec2!(0.0, 3.0),
    ]);
    
    let inner_square = QPolygon::new_from_parts(vec![
        qvec2!(1.0, 1.0),
        qvec2!(2.0, 1.0),
        qvec2!(2.0, 2.0),
        qvec2!(1.0, 2.0),
    ]);
    
    assert_eq!(gjk(&outer_square, &inner_square), true);
}

#[test]
fn test_graham_scan_simple() {
    // Test graham scan with a simple square
    let points = vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ];
    
    let hull = andrew_graham_scan(&points);
    assert_eq!(hull.len(), 4);
    // Check that all expected points are in the hull
    assert!(hull.contains(&qvec2!(0.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 1.0)));
    assert!(hull.contains(&qvec2!(0.0, 1.0)));
}

#[test]
fn test_graham_scan_with_duplicates() {
    // Test graham scan with duplicate points
    let points = vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
        qvec2!(0.0, 0.0), // duplicate
        qvec2!(1.0, 0.0), // duplicate
    ];
    
    let hull = andrew_graham_scan(&points);
    assert_eq!(hull.len(), 4);
    // Check that all expected points are in the hull
    assert!(hull.contains(&qvec2!(0.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 1.0)));
    assert!(hull.contains(&qvec2!(0.0, 1.0)));
}

#[test]
fn test_graham_scan_triangle() {
    // Test graham scan with a triangle
    let points = vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(0.5, 1.0),
    ];
    
    let hull = andrew_graham_scan(&points);
    assert_eq!(hull.len(), 3);
    // Check that all expected points are in the hull
    assert!(hull.contains(&qvec2!(0.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 0.0)));
    assert!(hull.contains(&qvec2!(0.5, 1.0)));
}

#[test]
fn test_graham_scan_colinear() {
    // Test graham scan with colinear points
    let points = vec![
        qvec2!(0.0, 0.0),
        qvec2!(0.5, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ];
    
    let hull = andrew_graham_scan(&points);
    // Should have 4 points for the convex hull (colinear points in the middle of an edge are removed)
    assert_eq!(hull.len(), 4);
    assert!(hull.contains(&qvec2!(0.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 0.0)));
    assert!(hull.contains(&qvec2!(1.0, 1.0)));
    assert!(hull.contains(&qvec2!(0.0, 1.0)));
}

#[test]
fn test_get_minkowski_difference_simple() {
    // Test Minkowski difference of two simple squares
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let diff = get_minkowski_difference(&square1, &square2);
    // The Minkowski difference of a square with itself should be a square centered at origin
    // with twice the size
    let diff_points = diff.points();
    assert_eq!(diff_points.len(), 4);
    
    // Check that the resulting polygon is a square centered at origin with side length 2
    let positions: Vec<QVec2> = diff_points.iter().map(|p| p.pos()).collect();
    assert!(positions.contains(&qvec2!(-1.0, -1.0)));
    assert!(positions.contains(&qvec2!(1.0, -1.0)));
    assert!(positions.contains(&qvec2!(1.0, 1.0)));
    assert!(positions.contains(&qvec2!(-1.0, 1.0)));
}

#[test]
fn test_get_minkowski_difference_offset() {
    // Test Minkowski difference of two squares at different positions
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(2.0, 2.0),
        qvec2!(3.0, 2.0),
        qvec2!(3.0, 3.0),
        qvec2!(2.0, 3.0),
    ]);
    
    let diff = get_minkowski_difference(&square1, &square2);
    let diff_points = diff.points();
    assert_eq!(diff_points.len(), 4);
    
    // Check that the resulting polygon is a square centered at (-2, -2) with side length 2
    let positions: Vec<QVec2> = diff_points.iter().map(|p| p.pos()).collect();
    assert!(positions.contains(&qvec2!(-3.0, -3.0)));
    assert!(positions.contains(&qvec2!(-1.0, -3.0)));
    assert!(positions.contains(&qvec2!(-1.0, -1.0)));
    assert!(positions.contains(&qvec2!(-3.0, -1.0)));
}

#[test]
fn test_get_minkowski_difference_triangle() {
    // Test Minkowski difference of two triangles
    let triangle1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(0.5, 1.0),
    ]);
    
    let triangle2 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(0.5, 1.0),
    ]);
    
    let diff = get_minkowski_difference(&triangle1, &triangle2);
    let diff_points = diff.points();
    // The result should be a hexagon (Minkowski difference of two triangles)
    assert_eq!(diff_points.len(), 6);
}

#[test]
fn test_epa_simple() {
    // Test EPA with two overlapping squares
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(0.5, 0.5),
        qvec2!(1.5, 0.5),
        qvec2!(1.5, 1.5),
        qvec2!(0.5, 1.5),
    ]);
    
    let separation_vector = epa(&square1, &square2);
    // Should return some separation vector since the shapes overlap
    assert!(separation_vector.is_some());
}

#[test]
fn test_epa_no_collision() {
    // Test EPA with two non-overlapping squares
    let square1 = QPolygon::new_from_parts(vec![
        qvec2!(0.0, 0.0),
        qvec2!(1.0, 0.0),
        qvec2!(1.0, 1.0),
        qvec2!(0.0, 1.0),
    ]);
    
    let square2 = QPolygon::new_from_parts(vec![
        qvec2!(2.0, 0.0),
        qvec2!(3.0, 0.0),
        qvec2!(3.0, 1.0),
        qvec2!(2.0, 1.0),
    ]);
    
    let separation_vector = epa(&square1, &square2);
    // Should return None since the shapes don't overlap
    assert!(separation_vector.is_none());
}