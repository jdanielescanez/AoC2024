use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

pub fn read_input(input: &str) -> IResult<&str, (String, Vec<&str>)> {
    let (input, (pattern, _, _, designs)) = tuple((
        separated_list1(tag(", "), alpha1),
        newline,
        newline,
        many1(pair(alpha1, newline)),
    ))(input)?;
    let regex_string = format!("^(({}))+$", pattern.join(")|("));
    let designs = designs.into_iter().map(|tuple| tuple.0).collect();
    Ok((input, (regex_string, designs)))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!(
                "{}\n\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                "r, wr, b, g, bwu, rb, gb, br",
                "brwrr",
                "bggr",
                "gbbr",
                "rrbgbr",
                "ubwu",
                "bwurrg",
                "brgr",
                "bbrgwb"
            )),
            Ok((
                "",
                (
                    "^((r)|(wr)|(b)|(g)|(bwu)|(rb)|(gb)|(br))+$".to_string(),
                    vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]
                )
            ))
        );
    }
}
