use itertools::Itertools;
use std::ops::Range;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    /// The point across from `self` of a circle centered on `other`.
    ///
    /// | Label | Name     |
    /// | ----- | -------- |
    /// | `S`   | self     |
    /// | `O`   | other    |
    /// | `A`   | antinode |
    ///
    /// ```ascii
    /// .......
    /// .....A.
    /// ..../..
    /// ...O...
    /// ../....
    /// .S.....
    /// .......
    /// ```
    fn antinode(&self, other: &Self) -> Point {
        Point {
            row: other.row - (self.row - other.row),
            col: other.col - (self.col - other.col),
        }
    }

    /// The points every N distance on a ray starting on `self` going through
    /// `other` where N is the distance between `self` and `other`. The first
    /// element is the antinode of `self` to `other`.
    ///
    /// | Label | Name            |
    /// | ----- | --------------- |
    /// | `S`   | self            |
    /// | `O`   | other           |
    /// | `A`   | first antinode  |
    /// | `B`   | second antinode |
    ///
    /// ```ascii
    /// .........
    /// .......B.
    /// ....../..
    /// .....A...
    /// ..../....
    /// ...O.....
    /// ../......
    /// .S.......
    /// .........
    /// ```
    fn antinodes(&self, other: &Self) -> Antinodes {
        Antinodes {
            p1: *self,
            p2: *other,
        }
    }

    fn within(&self, rows: &Range<isize>, cols: &Range<isize>) -> bool {
        rows.contains(&self.row) && cols.contains(&self.col)
    }
}

/// The points every N distance on a ray starting on `self` going through
/// `other` where N is the distance between `self` and `other`. The first
/// element is the antinode of `self` to `other`. See also
/// [antinodes function](Point::antinodes)
struct Antinodes {
    p1: Point,
    p2: Point,
}

impl Iterator for Antinodes {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        (self.p1, self.p2) = (self.p2, self.p1.antinode(&self.p2));
        Some(self.p2)
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    nodes: Vec<(u8, Point)>,
    rows: Range<isize>,
    cols: Range<isize>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    let rows = 0..input.lines().count() as isize;
    let cols = 0..input.lines().next().unwrap().len() as isize;
    let mut nodes: Vec<(u8, Point)> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.bytes().enumerate() {
            if c != b'.' {
                nodes.push((
                    c,
                    Point {
                        row: row as isize,
                        col: col as isize,
                    },
                ));
            }
        }
    }
    Ok((input, InputData { nodes, rows, cols }))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .nodes
        .iter()
        .cartesian_product(input.nodes.iter())
        .filter(|((n1, p1), (n2, p2))| n1 == n2 && p1 != p2)
        .map(|((_, p1), (_, p2))| p1.antinode(p2))
        .filter(|p| p.within(&input.rows, &input.cols))
        .unique()
        .count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .nodes
        .iter()
        .cartesian_product(input.nodes.iter())
        .filter(|((n1, p1), (n2, p2))| p1 != p2 && n1 == n2)
        .flat_map(|((_, p1), (_, p2))| {
            p1.antinodes(p2)
                .take_while(|p| p.within(&input.rows, &input.cols))
        })
        .chain(input.nodes.iter().map(|(_, p)| *p))
        .unique()
        .count())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                nodes: vec![
                    (48, Point { row: 1, col: 8 }),
                    (48, Point { row: 2, col: 5 }),
                    (48, Point { row: 3, col: 7 }),
                    (48, Point { row: 4, col: 4 }),
                    (65, Point { row: 5, col: 6 }),
                    (65, Point { row: 8, col: 8 }),
                    (65, Point { row: 9, col: 9 })
                ],
                rows: 0..12,
                cols: 0..12
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 14);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 34);
    }
}
