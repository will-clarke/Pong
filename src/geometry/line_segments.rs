use std::mem;
use config;
use geometry::vector::Vector;
use geometry::line_segment::LineSegment;
use ui;

pub struct LineSegments(pub Vec<LineSegment>);
pub struct LineSegmentRefs<'a>(pub Vec<&'a LineSegment>);


impl LineSegments {
    pub fn new_top_and_bottom_guards() -> LineSegments {
        let max_win_x = *ui::MAX_X as f64;
        let max_win_y = *ui::MAX_Y as f64;
        let top = LineSegment(Vector { x: 0.0, y: 0.0 },
                              Vector { x: max_win_x, y: 0.0 });
        let bottom = LineSegment(Vector { x: 0.0, y: max_win_y - 1.0 },
                                 Vector { x: max_win_x, y: max_win_y - 1.0 });
        let right = LineSegment(Vector { x: max_win_x - 1.0, y: 0.0},
                                Vector { x: max_win_x - 1.0, y: max_win_y - 1.0 });
        LineSegments(vec!(top, bottom, right))
    }

    pub fn to_intermediate_vectors(&self) -> Vec<Vector> {
        self.0.iter().flat_map (|line|
                                LineSegments::points_on_a_line(line.0, line.1)).
            collect()
    }

    // Bresenham's line algorithm
    pub fn points_on_a_line(point_a: Vector, point_b: Vector) -> Vec<Vector> {
        let mut x1 = point_a.x;
        let mut y1 = point_a.y;
        let mut x2 = point_b.x;
        let mut y2 = point_b.y;
        let steep = (y2 - y1).abs() > (x2 - x1).abs();

        if steep {
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
        }

        if x1 > x2
        {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
        }

        let dx = x2 - x1;
        let dy = (y2 - y1).abs();

        let mut error = dx / 2.0;
        let ystep = if y1 < y2 { 1 } else { -1 };
        let mut y = y1 as i32;

        let max_x = x2 as i32;

        (x1 as i32..max_x).map( |x|
                                 {
                                     let v = if steep {
                                         Vector::new(y, x)
                                     } else {
                                         Vector::new(x, y)
                                     };

                                     error = error - dy;
                                     if error < 0.0 {
                                         y += ystep;
                                         error += dx;
                                     };
                                     v
                                 }
        ).collect::<Vec<Vector>>()
    }

}
