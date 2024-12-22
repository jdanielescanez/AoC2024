use super::ram::{Position, Ram};

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::many1,
    sequence::{pair, separated_pair},
    IResult,
};

pub fn read_input(input: &str, size: usize, fallen_bytes: usize) -> IResult<&str, Ram> {
    let (input, bytes) = many1(pair(separated_pair(u32, tag(","), u32), newline))(input)?;
    let bytes = bytes
        .into_iter()
        .map(|tuple| Position::new((tuple.0 .0 as usize, tuple.0 .1 as usize)))
        .collect();
    Ok((input, Ram::new(bytes, size, fallen_bytes)))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!("{}\n{}\n{}\n", "54,47", "45,29", "41,65"), 70, 3),
            Ok((
                "",
                Ram::new(
                    vec![
                        Position::new((54, 47)),
                        Position::new((45, 29)),
                        Position::new((41, 65))
                    ],
                    70,
                    3
                )
            ))
        );
    }
}
