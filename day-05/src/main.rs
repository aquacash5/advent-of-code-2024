use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    orders: HashMap<u32, Vec<u32>>,
    manuals: Vec<Vec<u32>>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{newline, u32},
        combinator::map,
        multi::separated_list1,
        sequence::{pair, separated_pair},
    };

    let order = separated_pair(u32, tag("|"), u32);
    let orders = separated_list1(newline, order);
    let pages = separated_list1(tag(","), u32);
    let manuals = separated_list1(newline, pages);
    let groups = separated_pair(orders, pair(newline, newline), manuals);
    let mut parser = map(groups, |(orders, manuals)| InputData {
        orders: orders.into_iter().into_group_map(),
        manuals,
    });
    parser(input)
}

fn in_correct_order(orders: &HashMap<u32, Vec<u32>>, manual: &[u32]) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    for page in manual {
        if let Some(afters) = orders.get(page) {
            if afters.iter().any(|after| seen.contains(after)) {
                return false;
            }
        }
        seen.insert(*page);
    }
    true
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u32> {
    Ok(input
        .manuals
        .iter()
        .filter(|pages| in_correct_order(&input.orders, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u32> {
    Ok(input
        .manuals
        .iter()
        .filter(|pages| !in_correct_order(&input.orders, pages))
        .map(|pages| {
            pages
                .iter()
                .sorted_by(|&a, &b| {
                    input.orders.get(b).map_or(Ordering::Equal, |afters| {
                        if afters.iter().contains(a) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                })
                .collect_vec()
        })
        .map(|pages| pages[&pages.len() / 2])
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                orders: HashMap::from([
                    (53, vec![29, 13]),
                    (29, vec![13]),
                    (97, vec![13, 61, 47, 29, 53, 75]),
                    (47, vec![53, 13, 61, 29]),
                    (61, vec![13, 53, 29]),
                    (75, vec![29, 53, 47, 61, 13])
                ]),
                manuals: vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47]
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 143);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 123);
    }
}
