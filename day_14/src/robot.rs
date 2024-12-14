#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Robot {
    pub position: Position,
    pub velocity: Velocity,
}

impl Robot {
    pub fn move_steps(&mut self, steps: i32, limits: Position) {
        self.position.x = ((self.position.x as i32 + steps * self.velocity.x)
            .rem_euclid(limits.x as i32)) as usize;
        self.position.y = ((self.position.y as i32 + steps * self.velocity.y)
            .rem_euclid(limits.y as i32)) as usize;
    }

    pub fn get_quadrant(&self, limits: Position) -> Option<Quandrant> {
        let (mid_x, mid_y) = (limits.x / 2, limits.y / 2);
        if self.position.x == mid_x || self.position.y == mid_y {
            None
        } else if self.position.x < mid_x && self.position.y < mid_y {
            Some(Quandrant::First)
        } else if self.position.x > mid_x && self.position.y < mid_y {
            Some(Quandrant::Second)
        } else if self.position.x < mid_x && self.position.y > mid_y {
            Some(Quandrant::Third)
        } else {
            Some(Quandrant::Fourth)
        }
    }

    pub fn get_neighbour_positions(&self, limits: Position) -> Vec<Position> {
        vec![
            Position {
                x: (self.position.x + 1).rem_euclid(limits.x),
                y: self.position.y.rem_euclid(limits.y),
            },
            Position {
                x: self.position.x.rem_euclid(limits.x),
                y: (self.position.y + 1).rem_euclid(limits.y),
            },
            Position {
                x: ((self.position.x as i32 - 1).rem_euclid(limits.x as i32)) as usize,
                y: self.position.y.rem_euclid(limits.y),
            },
            Position {
                x: self.position.x.rem_euclid(limits.x),
                y: ((self.position.y as i32 - 1).rem_euclid(limits.y as i32)) as usize,
            },
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Quandrant {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}
