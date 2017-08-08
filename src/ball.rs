use board::Board;
use geometry::vector::Vector;
use geometry::angle::Angle;
use geometry::line_segments::LineSegments;
use geometry::line_segment::LineSegment;
use geometry::line_segment_intersection;
use config;

pub struct Ball {
    pub current_position: Vector,
    pub direction: Angle,
    pub distance: f64,
}


impl Ball {

    pub fn update_position(&self, line_segments: &LineSegments) -> Ball {

        let unit_vector = self.direction.to_vector();
        //  TODO: sort out this nasty hack to avoid transposing the to_vector method to deal with flipped y coordinates
        // let unit_vector = Vector {
        //     x: unit_vector.x,
        //     y: -unit_vector.y,
        // };

        let new_pos = Vector {
            x: self.current_position.x + (unit_vector.x * self.distance),
            y: self.current_position.y + (unit_vector.y * self.distance),
        };

        let line_to_new_pos = LineSegment(self.current_position, new_pos);

        // TODO: make this guy correct... we may be bouncing off the wrong wall (we're currently randomly choosing one)..
        let line_which_intersects = line_segments.0.iter().
            filter(|&segment|
                   line_segment_intersection::intersects(&segment, &line_to_new_pos).is_some()
            ).last();

        let next_position_and_direction = match line_which_intersects {
            Some(line_segment) => {
                // TODO: recompute this if we actually reflect once.. otherwise it could be WRONG :scream:
                // TODO: make this guy more efficient.
                // We're recalculating an intersection
                // that we've already done
                let intersection_point = line_segment_intersection::intersects(line_segment, &line_to_new_pos).unwrap();
                let distance_to_intersection = LineSegment(self.current_position, intersection_point).distance();
                let distance_left = self.distance - distance_to_intersection;
                let new_angle = self.direction.reflect(line_segment);

                let angle_vector = new_angle.to_vector();

                // let angle_vector = Vector {
                //     x: angle_vector.x,
                //     y: -angle_vector.y,
                // };

                let new_position = Vector {
                    x: intersection_point.x + angle_vector.x * distance_left,
                    y: intersection_point.y + angle_vector.y * distance_left,
                };

                (new_position, new_angle)

            }
            None => (new_pos, self.direction),
        };

        Ball {
            current_position: next_position_and_direction.0,
            direction: next_position_and_direction.1,
            distance: self.distance,
        }
    }

    pub fn new(config: &config::Config) -> Ball {
        let starting_pos = Vector {
            x: config.window_width / 2.0,
            y: config.window_height / 2.0,
        };

        Ball {
            current_position: starting_pos,
            direction: Angle(0.35), // TODO: make this random!!
            distance: 1.0,
        }
    }

    // pub fn is_cloned_obj(&self) -> Ball {
    //     let mut new_one = self;
    //     new_one.distance += 2.0;
    // }

    // pub fn new_position(&self, board: &Board) -> &Ball {
    //     let ball = self.clone();
    //     ball.current_position.x += ball.distance;
    //     ball.current_position.y += ball.distance;
    //     ball
    // }

}
