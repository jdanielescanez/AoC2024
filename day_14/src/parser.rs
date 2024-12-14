use super::robot::{Position, Robot, Velocity};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(multispace1, read_robot)(input)
}

fn read_robot(robot: &str) -> IResult<&str, Robot> {
    let (input, (_, position, _, velocity)) = tuple((
        tag("p="),
        separated_pair(i32, tag(","), i32),
        tag(" v="),
        separated_pair(i32, tag(","), i32),
    ))(robot)?;

    Ok((
        input,
        Robot {
            position: Position {
                x: position.0 as usize,
                y: position.1 as usize,
            },
            velocity: Velocity {
                x: velocity.0,
                y: velocity.1,
            },
        },
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_read_robot() {
        assert_eq!(
            read_robot("p=0,4 v=3,-3"),
            Ok((
                "",
                Robot {
                    position: Position { x: 0, y: 4 },
                    velocity: Velocity { x: 3, y: -3 },
                }
            ))
        );
    }

    #[test]
    fn test_read_input() {
        assert_eq!(
            read_input(&format!("{}\n{}", "p=6,3 v=-1,-3", "p=10,3 v=-1,2")),
            Ok((
                "",
                vec![
                    Robot {
                        position: Position { x: 6, y: 3 },
                        velocity: Velocity { x: -1, y: -3 },
                    },
                    Robot {
                        position: Position { x: 10, y: 3 },
                        velocity: Velocity { x: -1, y: 2 },
                    },
                ]
            ))
        );
    }
}
