use geometry::vector::Vector;
use geometry::angle::Angle;
use config;

pub struct Ball {
    current_position: Vector,
    direction: Angle,
    distance: f64,
}


impl Ball {
    pub fn new(config: &config::Config) -> Ball {
        let starting_pos =  Vector { x: config.window_width / 2.0,
                                     y: config.window_height / 2.0 };

        Ball {
            current_position: starting_pos,
            direction: Angle(0.25), // TODO: make this random!!
            distance: 0.3, // no idea
        }
    }
}
