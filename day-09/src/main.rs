use std::slice::Iter;

use itertools::{chain, repeat_n, Itertools};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum DiskData {
    File(usize),
    FreeSpace,
}

#[derive(Debug, PartialEq)]
struct InputData {
    fs: Vec<DiskData>,
}

impl InputData {
    fn defrag(&self) -> Defragment<'_> {
        Defragment {
            list: self.fs.iter(),
        }
    }
}

fn parse(input: &str) -> ParseResult<InputData> {
    Ok((
        input,
        InputData {
            fs: input
                .bytes()
                .chain([b'0'])
                .tuples()
                .enumerate()
                .flat_map(|(id, (file, space))| {
                    chain!(
                        repeat_n(DiskData::File(id), (file - b'0').into()),
                        repeat_n(DiskData::FreeSpace, (space - b'0').into())
                    )
                })
                .collect_vec(),
        },
    ))
}

#[derive(Debug, Clone)]
struct Defragment<'a> {
    list: Iter<'a, DiskData>,
}

impl Iterator for Defragment<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.list
            .next()
            .and_then(|d| match d {
                DiskData::File(id) => Some(id),
                DiskData::FreeSpace => self
                    .list
                    .rfind(|d| matches!(d, DiskData::File(_)))
                    .and_then(|d| match d {
                        DiskData::File(id) => Some(id),
                        DiskData::FreeSpace => None,
                    }),
            })
            .copied()
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input.defrag().enumerate().map(|(i, id)| i * id).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<()> {
    Ok(())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parser() {
        use DiskData::*;

        assert_parser!(
            parse,
            INPUT,
            InputData {
                fs: vec![
                    File(0),
                    File(0),
                    FreeSpace,
                    FreeSpace,
                    FreeSpace,
                    File(1),
                    File(1),
                    File(1),
                    FreeSpace,
                    FreeSpace,
                    FreeSpace,
                    File(2),
                    FreeSpace,
                    FreeSpace,
                    FreeSpace,
                    File(3),
                    File(3),
                    File(3),
                    FreeSpace,
                    File(4),
                    File(4),
                    FreeSpace,
                    File(5),
                    File(5),
                    File(5),
                    File(5),
                    FreeSpace,
                    File(6),
                    File(6),
                    File(6),
                    File(6),
                    FreeSpace,
                    File(7),
                    File(7),
                    File(7),
                    FreeSpace,
                    File(8),
                    File(8),
                    File(8),
                    File(8),
                    File(9),
                    File(9),
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 1928);
    }

    #[test]
    fn test_part2() {
        // assert_part!(parse, part2, INPUT, ());
    }
}
