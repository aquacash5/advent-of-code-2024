use itertools::Itertools;
use nom::ToUsize;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{line_ending, space1, u32},
        combinator::map,
        multi::separated_list0,
        sequence::separated_pair,
    };

    let pair = separated_pair(u32, space1, u32);
    let lines = separated_list0(line_ending, pair);
    let mut parser = map(lines, |lines| {
        let (left, right) = lines.into_iter().unzip();
        InputData { left, right }
    });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u32> {
    let left = input.left.iter().sorted();
    let right = input.right.iter().sorted();
    Ok(left.zip(right).map(|(&a, &b)| a.abs_diff(b)).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let right_counts = input.right.iter().counts();
    Ok(input
        .left
        .iter()
        .map(|i| i.to_usize() * right_counts.get(i).unwrap_or(&0))
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                left: vec![3, 4, 2, 1, 3, 3],
                right: vec![4, 3, 5, 3, 9, 3]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 11);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 31);
    }
}
