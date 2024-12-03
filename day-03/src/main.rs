#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(u32, u32),
    Other(char),
    Do,
    Dont,
}

#[derive(Debug, PartialEq)]
struct InputData {
    operations: Vec<Operation>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, u32},
        combinator::map,
        multi::many1,
        sequence::{delimited, separated_pair},
    };
    let multiply = map(
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")),
        |(a, b)| Operation::Multiply(a, b),
    );
    let do_ = map(tag("do()"), |_| Operation::Do);
    let dont = map(tag("don't()"), |_| Operation::Dont);
    let other = map(anychar, Operation::Other);
    let maybe = alt((multiply, do_, dont, other));
    let mut parser = map(many1(maybe), |operations| InputData { operations });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u32> {
    Ok(input
        .operations
        .iter()
        .map(|operation| match operation {
            Operation::Multiply(a, b) => a * b,
            _ => 0,
        })
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u32> {
    Ok(input
        .operations
        .iter()
        .fold((true, 0), |(enabled, total), operation| match operation {
            Operation::Multiply(a, b) => (enabled, if enabled { total + (a * b) } else { total }),
            Operation::Other(_) => (enabled, total),
            Operation::Do => (true, total),
            Operation::Dont => (false, total),
        })
        .1)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parser() {
        use Operation::*;
        assert_parser!(
            parse,
            INPUT1,
            InputData {
                operations: vec![
                    Other('x'),
                    Multiply(2, 4),
                    Other('%'),
                    Other('&'),
                    Other('m'),
                    Other('u'),
                    Other('l'),
                    Other('['),
                    Other('3'),
                    Other(','),
                    Other('7'),
                    Other(']'),
                    Other('!'),
                    Other('@'),
                    Other('^'),
                    Other('d'),
                    Other('o'),
                    Other('_'),
                    Other('n'),
                    Other('o'),
                    Other('t'),
                    Other('_'),
                    Multiply(5, 5),
                    Other('+'),
                    Other('m'),
                    Other('u'),
                    Other('l'),
                    Other('('),
                    Other('3'),
                    Other('2'),
                    Other(','),
                    Other('6'),
                    Other('4'),
                    Other(']'),
                    Other('t'),
                    Other('h'),
                    Other('e'),
                    Other('n'),
                    Other('('),
                    Multiply(11, 8),
                    Multiply(8, 5),
                    Other(')')
                ]
            }
        );
        assert_parser!(
            parse,
            INPUT2,
            InputData {
                operations: vec![
                    Other('x'),
                    Multiply(2, 4),
                    Other('&'),
                    Other('m'),
                    Other('u'),
                    Other('l'),
                    Other('['),
                    Other('3'),
                    Other(','),
                    Other('7'),
                    Other(']'),
                    Other('!'),
                    Other('^'),
                    Dont,
                    Other('_'),
                    Multiply(5, 5),
                    Other('+'),
                    Other('m'),
                    Other('u'),
                    Other('l'),
                    Other('('),
                    Other('3'),
                    Other('2'),
                    Other(','),
                    Other('6'),
                    Other('4'),
                    Other(']'),
                    Other('('),
                    Multiply(11, 8),
                    Other('u'),
                    Other('n'),
                    Do,
                    Other('?'),
                    Multiply(8, 5),
                    Other(')')
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT1, 161);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT2, 48);
    }
}
