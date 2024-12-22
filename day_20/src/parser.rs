use super::racetrack::{Cell, Position, Racetrack};

use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    sequence::pair,
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, Racetrack> {
    let (input, map) = (many1(pair(many1(one_of("#.ES")), newline)))(input)?;

    let mut start = Position::new((0, 0));
    let mut end = Position::new((0, 0));

    Ok((
        input,
        Racetrack {
            map: map
                .into_iter()
                .enumerate()
                .map(|(y, (row, _))| {
                    row.into_iter()
                        .enumerate()
                        .map(|(x, cell)| {
                            if cell == 'S' {
                                start = Position::new((x, y));
                            } else if cell == 'E' {
                                end = Position::new((x, y));
                            }
                            Cell::new(cell).unwrap()
                        })
                        .collect()
                })
                .collect(),
            start,
            end,
        },
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!("####\n#.E#\n#S.#\n####\n")),
            Ok((
                "",
                Racetrack {
                    map: vec![
                        vec![Cell::Wall; 4],
                        vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Wall],
                        vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Wall],
                        vec![Cell::Wall; 4],
                    ],
                    start: Position::new((1, 2)),
                    end: Position::new((2, 1)),
                }
            ))
        );
    }
}
