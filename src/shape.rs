use geometry::vector::Vector;
use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use rand;
use rand::Rng;
use ui;

pub struct Shape {
    pub relative_coords: Vec<Vector>,
    pub origin: Vector,
}

impl Shape {
    pub fn rotate(&mut self, angle: i32) {
        let rotation_speed = 100;
        let angle = angle % rotation_speed;
        let new_coords: Vec<Vector> = self.relative_coords.iter().map(|&vec| vec).collect();
        self.relative_coords = new_coords;
        // TODO.. work on..
    }

    pub fn absolute_coords(&self) -> Vec<Vector> {
        self.relative_coords.clone().iter().map(|&coord| coord + self.origin).collect()
    }

    pub fn to_line_segments(&self) -> LineSegments {
        let mut line_segments = LineSegments::empty();
        let mut coords = self.absolute_coords();
        let first_line_segment = coords.pop().unwrap();
        let mut previous_line_segment = first_line_segment.clone();
        while let Some(current_line_segment) = coords.pop() {
            let line_segment = LineSegment(previous_line_segment,
                                           current_line_segment.clone());
            previous_line_segment = current_line_segment;
            line_segments.push(line_segment);
        }
        line_segments.push(LineSegment(first_line_segment, previous_line_segment));
        line_segments
    }

    pub fn random_triangle() -> Self {
        let pointiness = 5.0;
        let mut rng = rand::thread_rng();
        let multiplier: f64 = rng.gen_range(2.0, 4.0);

        let half_x = (*ui::MAX_X / 2)as f64;
        let half_y = (*ui::MAX_Y / 2) as f64;

        let random_x = half_x / multiplier;
        let random_y = half_y / multiplier;


      // meant to be something pointy like this :|
      //
      //                    X
      //                   X X
      //                  X   X
      //                 X     X
      //         XXXXXXX       XXXXXXXXXXXXXXX
      // XXXXXXXX       X      XXXXXXXXXXXXXXX
      //         XXXXXXX X    X
      //                  X  X
      //                   XX


        Shape {
            relative_coords: vec!(
                Vector {x: random_x / pointiness, y: random_y / pointiness},
                Vector {x: 0.0, y: random_y},
                Vector {x: -random_x / pointiness, y: random_y / pointiness},
                Vector {x: -random_x, y: 0.0},
                Vector {x: -random_x / pointiness, y: -random_y / pointiness},
                Vector {x: 0.0, y: -random_y},
                Vector {x: random_x / pointiness, y: -random_y / pointiness},
                Vector {x: random_x, y: 0.0},
            ),
            origin: Vector {
                x: half_x,
                y: half_y,
            }

        }

    }
}


#[test]
fn test_to_line_segments() {
    let relative_coords = vec!(
        Vector { x: 1.0, y: 1.0 },
        Vector { x: -1.0, y: 1.0 },
        Vector { x: -1.0, y: -1.0 },
        Vector { x: 1.0, y: -1.0 },
    );

    let shape = Shape {
        origin: Vector {x: 0.0, y: 0.0},
        relative_coords: relative_coords.clone(),
    };

    assert_eq!(shape.to_line_segments(),
               LineSegments(vec!(
                   LineSegment(Vector { x: 1.0, y: -1.0 }, Vector { x: -1.0, y: -1.0 }),
                   LineSegment(Vector { x: -1.0, y: -1.0 }, Vector { x: -1.0, y: 1.0 }),
                   LineSegment(Vector { x: -1.0, y: 1.0 }, Vector { x: 1.0, y: 1.0 }),
                   LineSegment(Vector { x: 1.0, y: -1.0 }, Vector { x: 1.0, y: 1.0 })
               ))
    );
    let off_center_shape = Shape {
        origin: Vector {x: 10.0, y: 10.0},
        relative_coords: relative_coords,
    };
    assert_eq!(off_center_shape.to_line_segments(),
               LineSegments(vec!(
                   LineSegment(Vector { x: 11.0, y: 9.0 }, Vector { x: 9.0, y: 9.0 }),
                   LineSegment(Vector { x: 9.0, y: 9.0 }, Vector { x: 9.0, y: 11.0 }),
                   LineSegment(Vector { x: 9.0, y: 11.0 }, Vector { x: 11.0, y: 11.0 }),
                   LineSegment(Vector { x: 11.0, y: 9.0 }, Vector { x: 11.0, y: 11.0 })
               ))
    );
}
