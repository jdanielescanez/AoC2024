pub struct Solution {
    pub a: u32,
    pub b: u32,
}

impl Solution {
    pub fn new(a: f64, b: f64) -> Option<Self> {
        if a.fract() == 0.0 && b.fract() == 0.0 {
            Some(Solution {
                a: a as u32,
                b: b as u32,
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Behavior {
    pub x_a: i32,
    pub x_b: i32,
    pub x_prize: i32,
    pub y_a: i32,
    pub y_b: i32,
    pub y_prize: i32,
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
}
