use geometry::vector::Vector;
use geometry::angle::Angle;
use geometry::line_segment::LineSegment;
use geometry::line_segment_intersection;
use score::Score;
use state::State;
use io::Input;
use ui;

pub struct Ball {
    pub current_position: Vector,
    pub direction: Angle,
    pub distance: f64,
}

impl Ball {

    pub fn new() -> Ball {
        let starting_pos = Vector {
            x: *ui::MAX_X as f64 / 2.0,
            y: *ui::MAX_Y as f64 / 2.0,
        };

        Ball {
            current_position: starting_pos,
            direction: Angle::random_start(),
            distance: 0.1,
        }
    }

    pub fn update_position(&self, state: &State, input: &mut Input, score: &mut Score) -> Ball {

        if input.restart_toggle {
            input.restart_toggle = false;
            score.l_score = 0;
            return Ball::new();
        }

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

        let line_segments_on_board = &state.board_line_segments().clone();
        let line_which_intersects = line_segments_on_board.
            0.iter().
            filter(|&segment|
                   line_segment_intersection::intersects(segment, &line_to_new_pos).is_some()
            ).last();

        let next_position_and_direction = match line_which_intersects {
            Some(line_segment) => {
                let end_segment = LineSegment(Vector { x: *ui::MAX_X as f64 - 1.0, y: 0.0},
                                              Vector { x: *ui::MAX_X as f64 - 1.0, y: *ui::MAX_Y as f64 - 1.0 });
                if line_segment == &end_segment {
                    score.l_score += 1;
                }
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

}
