use itertools::Itertools;
use std::cmp::Ordering;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    reports: Vec<Vec<u32>>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{newline, space1, u32},
        combinator::map,
        multi::separated_list1,
    };

    let report = separated_list1(space1, u32);
    let reports = separated_list1(newline, report);
    let mut parser = map(reports, |reports| InputData { reports });
    parser(input)
}

fn is_gap_safe(expected: Ordering, a: u32, b: u32) -> bool {
    use Ordering::*;
    matches!(
        (expected, a.cmp(&b), a.abs_diff(b)),
        (Greater, Greater, diff) | (Less, Less, diff) if (1..=3).contains(&diff)
    )
}

fn is_safe<'a>(mut report: impl Iterator<Item = &'a u32>) -> bool {
    let Some(first) = report.next() else {
        return false;
    };
    let Some(second) = report.next() else {
        return true;
    };
    let expected = first.cmp(second);
    for (&a, &b) in [first, second].into_iter().chain(report).tuple_windows() {
        if !is_gap_safe(expected, a, b) {
            return false;
        }
    }
    true
}

fn is_recoverable(report: &[u32]) -> bool {
    (0..report.len()).any(|i| {
        is_safe(
            report
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, m)| m),
        )
    })
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .reports
        .iter()
        .filter(|report| is_safe(report.iter()))
        .count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .reports
        .iter()
        .filter(|report| is_recoverable(report))
        .count())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                reports: vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 2);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 4);
    }
}
