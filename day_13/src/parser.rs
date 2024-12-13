use super::behavior::Behavior;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, Vec<Behavior>> {
    separated_list1(multispace1, read_behavior)(input)
}

fn read_behavior(behavior: &str) -> IResult<&str, Behavior> {
    let (input, (_, x_a, _, y_a, _, x_b, _, y_b, _, x_prize, _, y_prize)) = tuple((
        tag("Button A: X+"),
        i32,
        tag(", Y+"),
        i32,
        tag("\nButton B: X+"),
        i32,
        tag(", Y+"),
        i32,
        tag("\nPrize: X="),
        i32,
        tag(", Y="),
        i32,
    ))(behavior)?;

    Ok((
        input,
        Behavior {
            x_a,
            y_a,
            x_b,
            y_b,
            x_prize,
            y_prize,
        },
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_read_behavior() {
        assert_eq!(
            read_behavior("Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400"),
            Ok((
                "",
                Behavior {
                    x_a: 94,
                    y_a: 34,
                    x_b: 22,
                    y_b: 67,
                    x_prize: 8400,
                    y_prize: 5400,
                }
            ))
        );
    }

    #[test]
    fn test_read_input() {
        assert_eq!(
            read_input(&format!(
                "{}\n\n{}",
                "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400",
                "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=5, Y=6"
            )),
            Ok((
                "",
                vec![
                    Behavior {
                        x_a: 94,
                        y_a: 34,
                        x_b: 22,
                        y_b: 67,
                        x_prize: 8400,
                        y_prize: 5400,
                    },
                    Behavior {
                        x_a: 1,
                        y_a: 2,
                        x_b: 3,
                        y_b: 4,
                        x_prize: 5,
                        y_prize: 6,
                    },
                ]
            ))
        );
    }
}
