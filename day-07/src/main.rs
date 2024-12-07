#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => a
                .saturating_mul(10u64.pow(b.ilog10() + 1))
                .saturating_add(b),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData(Vec<(u64, Vec<u64>)>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space1, u64},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };

    let nums = separated_list1(space1, u64);
    let item = separated_pair(u64, tag(": "), nums);
    let items = separated_list1(line_ending, item);
    let mut parser = map(items, InputData);
    parser(input)
}

fn can_calibrate(result: u64, nums: &[u64], operations: &[Operation]) -> bool {
    fn calibration_total(result: u64, total: u64, nums: &[u64], operations: &[Operation]) -> bool {
        let Some((head, tail)) = nums.split_first() else {
            return total == result;
        };

        operations
            .iter()
            .any(|o| calibration_total(result, o.apply(total, *head), tail, operations))
    }
    calibration_total(result, nums[0], &nums[1..], operations)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    use Operation::*;

    Ok(input
        .0
        .iter()
        .filter(|(result, nums)| can_calibrate(*result, nums, &[Add, Multiply]))
        .map(|(result, _)| *result)
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    use Operation::*;

    Ok(input
        .0
        .iter()
        .filter(|(result, nums)| can_calibrate(*result, nums, &[Add, Multiply, Concatenate]))
        .map(|(result, _)| *result)
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData(vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20])
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 3749);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 11387);
    }
}
