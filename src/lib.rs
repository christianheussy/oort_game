use oort_api::prelude::*;
const UPPER_BOUND: f64 = (2.0 * PI) - 0.2;
const TWO_RAD: f64 = 2.0 * PI;

#[derive(Default)]
pub struct Ship {}

impl Ship {
    /// Constant bullet speed m/s
    const BULLET_SPEED: f64 = 1000.0;

    pub fn new() -> Ship {
        Ship {}
    }

    fn stable_heading(&self) -> f64 {
        let raw_heading = heading();

        let filtered = match raw_heading {
            0.0..=0.2 => 0.0,
            UPPER_BOUND..=TWO_RAD => 0.0,
            val => val,
        };

        debug!("Filtered heading: {}", filtered);
        filtered
    }

    // TODO: steer negative
    fn steer_to(&self, target_angle: f64) {
        let steering_command = target_angle - self.stable_heading();
        debug!("Steer: {}", steering_command);
        turn(steering_command);
    }

    fn firing_solution(
        &self,
        position_bullet: Vec2,
        position_target: Vec2,
        velocity_target: Vec2,
    ) -> Option<Vec2> {
        // Subtract the target's position from the bullet's position
        let position_delta = position_bullet - position_target;

        // Determine the distance between the target and the bullet
        let d = position_delta.length();

        // Determine the magnitude of the target's velocity vector (speed_target)
        let speed_target = velocity_target.length();

        // Use the law of cosines to obtain quadratic
        let a = Self::BULLET_SPEED.powf(2.0) - speed_target.powf(2.0);
        let b = 2.0 * position_delta.dot(position_target);
        let c = (d * -1.0).powf(2.0);

        // Determine if the quadratic has a solution
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Solve quadratic to determine intercept time
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        // Determine valid t
        let t = {
            if t1 < 0.0 && t2 < 0.0 {
                return None;
            } else if t1 < 0.0 {
                t2
            } else if t2 < 0.0 {
                t1
            } else {
                t1.min(t2)
            }
        };

        // Now that t is known determine interception point
        let intercept_point = position_target + t * velocity_target;

        // Calculate velocity vector
        let velocity_vector = (intercept_point - position_bullet) / t;

        Some(velocity_vector)
    }

    pub fn tick(&mut self) {
        debug!("Pb: {}", position());
        debug!("Pt: {}", target());
        debug!("Vt: {}", target_velocity());
        if let Some(solution) = self.firing_solution(position(), target(), target_velocity()) {
            self.steer_to(solution.angle());

            // TODO: only fire if error within threshold
            fire(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firing_solution() {
        let ship = Ship::new();
        let pb = Vec2::new(0., 0.);
        let pt = Vec2::new(-2435., 1112.);
        let vt = Vec2::new(117., -382.);
        let solution = ship.firing_solution(pb, pt, vt);
        let angle = solution.unwrap().angle();
        println!("faldskjdfs")
    }
}
