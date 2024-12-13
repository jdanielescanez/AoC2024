pub struct Solution {
    pub a: u64,
    pub b: u64,
}

impl Solution {
    pub fn new(a: f64, b: f64) -> Option<Self> {
        if a.fract() == 0.0 && b.fract() == 0.0 {
            Some(Solution {
                a: a as u64,
                b: b as u64,
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Behavior {
    pub x_a: i64,
    pub x_b: i64,
    pub x_prize: i64,
    pub y_a: i64,
    pub y_b: i64,
    pub y_prize: i64,
}

impl Behavior {
    pub fn get_min_solution(&self) -> Option<Solution> {
        let denominator_determinant = (self.x_a * self.y_b - self.x_b * self.y_a) as f64;
        let x_determinant = (self.x_prize * self.y_b - self.x_b * self.y_prize) as f64;
        let y_determinant = (self.x_a * self.y_prize - self.x_prize * self.y_a) as f64;

        Solution::new(
            x_determinant / denominator_determinant,
            y_determinant / denominator_determinant,
        )
    }

    pub fn set_unit_conversion_correction(&mut self) {
        self.x_prize += 10000000000000;
        self.y_prize += 10000000000000;
    }
}
