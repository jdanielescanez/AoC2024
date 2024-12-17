use super::computer::Computer;

use nom::{
    bytes::complete::tag, character::complete::u32, multi::separated_list1, sequence::tuple,
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, Computer> {
    let (input, (_, a_register, _, b_register, _, c_register, _, program)) = tuple((
        tag("Register A: "),
        u32,
        tag("\nRegister B: "),
        u32,
        tag("\nRegister C: "),
        u32,
        tag("\n\nProgram: "),
        separated_list1(tag(","), u32),
    ))(input)?;

    Ok((
        input,
        Computer::new(a_register, b_register, c_register, program),
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!(
                "{}\n{}\n{}\n\n{}",
                "Register A: 729", "Register B: 0", "Register C: 0", "Program: 0,1,5,4,3,0"
            )),
            Ok(("", Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0])))
        );
    }
}
