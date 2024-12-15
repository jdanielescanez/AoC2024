use super::warehouse::{Cell, Direction, Position, Warehouse};

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, Warehouse> {
    let (input, (map, _, directions)) = tuple((
        many1(pair(many1(one_of("#@.O")), newline)),
        newline,
        separated_list1(newline, many1(one_of("^v><"))),
    ))(input)?;

    let mut robot_position = Position { x: 0, y: 0 };

    Ok((
        input,
        Warehouse {
            moves: directions
                .into_iter()
                .flatten()
                .map(|direction| Direction::new(direction).unwrap())
                .collect(),
            map: map
                .into_iter()
                .enumerate()
                .map(|(x, (row, _))| {
                    row.into_iter()
                        .enumerate()
                        .map(|(y, cell)| {
                            if cell == '@' {
                                robot_position = Position { x, y };
                            }
                            Cell::new(cell).unwrap()
                        })
                        .collect()
                })
                .collect(),
            robot_position,
        },
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!(
                "{}\n{}",
                "####\n#O.#\n#@.#\n####\n", "^^v^^\n<<>\n>vv"
            )),
            Ok((
                "",
                Warehouse {
                    robot_position: Position { x: 2, y: 1 },
                    map: vec![
                        vec![Cell::Wall; 4],
                        vec![Cell::Wall, Cell::Box, Cell::Empty, Cell::Wall],
                        vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Wall],
                        vec![Cell::Wall; 4],
                    ],
                    moves: vec![
                        Direction::Up,
                        Direction::Up,
                        Direction::Down,
                        Direction::Up,
                        Direction::Up,
                        Direction::Left,
                        Direction::Left,
                        Direction::Right,
                        Direction::Right,
                        Direction::Down,
                        Direction::Down,
                    ],
                }
            ))
        );
    }
}
