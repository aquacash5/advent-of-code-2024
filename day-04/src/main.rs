use itertools::Itertools;
use ndarray::{s, Array2};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    letters: Array2<u8>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    let input = input.trim();
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();
    Ok((
        input,
        InputData {
            letters: Array2::from_shape_vec(
                (row_count, col_count),
                input.bytes().filter(|&b| b != b'\n').collect_vec(),
            )
            .unwrap(),
        },
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    const WORD: &[u8; 4] = b"XMAS";

    let mut count = 0usize;
    // rows
    for win in input.letters.windows((WORD.len(), 1)) {
        if win.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if win.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    // columns
    for win in input.letters.windows((1, WORD.len())) {
        if win.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if win.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    // diagonal
    for win in input.letters.windows((WORD.len(), WORD.len())) {
        let diag = win.diag();
        if diag.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if diag.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }

        let slice = win.slice(s![.., ..;-1]);
        let diag = slice.diag();
        if diag.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if diag.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    Ok(count)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut count = 0usize;
    for window in input.letters.windows((3, 3)) {
        // M.S
        // .A.
        // M.S
        let a = window.get((0, 0)) == Some(&b'M')
            && window.get((2, 0)) == Some(&b'M')
            && window.get((1, 1)) == Some(&b'A')
            && window.get((2, 2)) == Some(&b'S')
            && window.get((0, 2)) == Some(&b'S');

        // M.M
        // .A.
        // S.S
        let b = window.get((0, 0)) == Some(&b'M')
            && window.get((2, 0)) == Some(&b'S')
            && window.get((1, 1)) == Some(&b'A')
            && window.get((2, 2)) == Some(&b'S')
            && window.get((0, 2)) == Some(&b'M');

        // S.M
        // .A.
        // S.M
        let c = window.get((0, 0)) == Some(&b'S')
            && window.get((2, 0)) == Some(&b'S')
            && window.get((1, 1)) == Some(&b'A')
            && window.get((2, 2)) == Some(&b'M')
            && window.get((0, 2)) == Some(&b'M');

        // S.S
        // .A.
        // M.M
        let d = window.get((0, 0)) == Some(&b'S')
            && window.get((2, 0)) == Some(&b'M')
            && window.get((1, 1)) == Some(&b'A')
            && window.get((2, 2)) == Some(&b'M')
            && window.get((0, 2)) == Some(&b'S');
        if a || b || c || d {
            count += 1;
        }
    }
    Ok(count)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                letters: array![
                    [77, 77, 77, 83, 88, 88, 77, 65, 83, 77],
                    [77, 83, 65, 77, 88, 77, 83, 77, 83, 65],
                    [65, 77, 88, 83, 88, 77, 65, 65, 77, 77],
                    [77, 83, 65, 77, 65, 83, 77, 83, 77, 88],
                    [88, 77, 65, 83, 65, 77, 88, 65, 77, 77],
                    [88, 88, 65, 77, 77, 88, 88, 65, 77, 65],
                    [83, 77, 83, 77, 83, 65, 83, 88, 83, 83],
                    [83, 65, 88, 65, 77, 65, 83, 65, 65, 65],
                    [77, 65, 77, 77, 77, 88, 77, 77, 77, 77],
                    [77, 88, 77, 88, 65, 88, 77, 65, 83, 88]
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 18);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 9);
    }
}
