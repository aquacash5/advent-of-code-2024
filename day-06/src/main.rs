use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashSet, ops::Range};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn next_in_direction(&self, direction: Direction) -> Option<Self> {
        let &Point { row, col } = self;
        match direction {
            Direction::North => row.checked_sub(1).map(|row| Point { row, col }),
            Direction::East => Some(Point { row, col: col + 1 }),
            Direction::South => Some(Point { row: row + 1, col }),
            Direction::West => col.checked_sub(1).map(|col| Point { row, col }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    location: Point,
    direction: Direction,
}

#[derive(Debug)]
struct Walk<'a> {
    guard: Guard,
    points: &'a HashSet<Point>,
    rows: &'a Range<usize>,
    columns: &'a Range<usize>,
    first: bool,
}

impl Iterator for Walk<'_> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.guard);
        }
        let next = self
            .guard
            .location
            .next_in_direction(self.guard.direction)?;
        if self.points.contains(&next) {
            self.guard.direction = self.guard.direction.rotate();
            Some(self.guard)
        } else if self.rows.contains(&next.row) && self.columns.contains(&next.col) {
            self.guard.location = next;
            Some(self.guard)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct InputData {
    points: HashSet<Point>,
    start: Point,
    rows: Range<usize>,
    columns: Range<usize>,
}

impl InputData {
    fn walk(&self) -> Walk {
        Walk {
            guard: Guard {
                location: self.start,
                direction: Direction::North,
            },
            points: &self.points,
            rows: &self.rows,
            columns: &self.columns,
            first: true,
        }
    }

    fn alternate(&self, p: Point) -> Self {
        let mut alt = self.clone();
        alt.points.insert(p);
        alt
    }
}

fn parse(input: &str) -> ParseResult<InputData> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut start: Option<Point> = None;
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.bytes().enumerate() {
            match c {
                b'#' => {
                    points.insert(Point { row, col });
                }
                b'^' => start = Some(Point { row, col }),
                _ => (),
            }
        }
    }
    Ok((
        input,
        InputData {
            points,
            start: start.unwrap(),
            rows: 0..height,
            columns: 0..width,
        },
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input.walk().map(|g| g.location).unique().count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .walk()
        .skip(1)
        .map(|g| g.location)
        .unique()
        .par_bridge()
        .filter(|p| {
            let mut seen: HashSet<Guard> = HashSet::new();
            for guard in input.alternate(*p).walk() {
                if !seen.insert(guard) {
                    return true;
                }
            }
            false
        })
        .count())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                points: HashSet::from([
                    Point { row: 0, col: 4 },
                    Point { row: 7, col: 8 },
                    Point { row: 3, col: 2 },
                    Point { row: 1, col: 9 },
                    Point { row: 4, col: 7 },
                    Point { row: 8, col: 0 },
                    Point { row: 9, col: 6 },
                    Point { row: 6, col: 1 }
                ]),
                start: Point { row: 6, col: 4 },
                rows: 0..10,
                columns: 0..10
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 41);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 6);
    }
}
