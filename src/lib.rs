use oort_api::prelude::*;

const UPPER_BOUND: f64 = (2.0 * PI) - 0.2;
const TWO_RAD: f64 = 2.0 * PI;

const BULLET_SPEED: f64 = 1000.0; // m/s

#[derive(Default)]
pub struct Ship {
    target: Option<TargetData>,
}

struct TargetData {
    init_pos: Vec2,
    velocity: Vec2,
}

impl TargetData {
    /// Given time in seconds calculate the targets position.
    /// Assume constant velocity.
    fn get_position(&self, time: f64) -> Vec2 {
        self.init_pos + time * self.velocity
    }
}

impl Ship {
    pub fn new() -> Ship {
        Ship { target: None }
    }

    pub fn tick(&mut self) {
        if self.target.is_none() {
            self.target = Some(TargetData {
                init_pos: target(),
                velocity: target_velocity(),
            });
        }

        let target_data = self.target.as_ref().unwrap();

        let target_vector = target_data.get_position(current_time()) - position();

        let target_angle = target_vector.angle();
        debug!("Target vector: {:?}", target_vector);
        debug!("Target angle: {:?}", target_angle);

        let stable_heading = match heading() {
            0.0..=0.2 => 0.0,
            UPPER_BOUND..=TWO_RAD => 0.0,
            val => val,
        };
        debug!("heading: {}", stable_heading);

        let steering_command = (target_angle - stable_heading);
        debug!("Steer: {}", steering_command);

        turn(steering_command);

        draw_line(
            position(),
            target_data.get_position(current_time()),
            0x00ff00,
        );

        // let dp = target() - position();
        // debug!("distance to target: {}", dp.length());
        // debug!("time to target: {}", dp.length() / BULLET_SPEED);

        // turn(1.0);
        fire(0);
    }
}
