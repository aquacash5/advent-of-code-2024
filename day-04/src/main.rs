use ndarray::{s, Array2};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    letters: Array2<u8>,
}

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let input = input.trim();
    let row_count = input.lines().count();
    let col_count = input.lines().next().unwrap().len();
    Ok((
        input,
        InputData {
            letters: Array2::from_shape_vec(
                (row_count, col_count),
                input.bytes().filter(|&b| b != b'\n').collect(),
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
    for window in input.letters.windows((WORD.len(), 1)) {
        if window.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if window.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    // columns
    for window in input.letters.windows((1, WORD.len())) {
        if window.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if window.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    // diagonal
    for window in input.letters.windows((WORD.len(), WORD.len())) {
        let diagonal = window.diag();
        if diagonal.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if diagonal.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }

        let slice = window.slice(s![.., ..;-1]);
        let diagonal = slice.diag();
        if diagonal.iter().zip(WORD.iter()).all(|(a, b)| a == b) {
            count += 1;
        }
        if diagonal.iter().zip(WORD.iter().rev()).all(|(a, b)| a == b) {
            count += 1;
        }
    }
    Ok(count)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .letters
        .windows((3, 3))
        .into_iter()
        .filter(|w| {
            matches!(
                (
                    w.get((0, 0)),
                    w.get((2, 0)),
                    w.get((2, 2)),
                    w.get((0, 2)),
                    w.get((1, 1)),
                ),
                (
                    Some(&b'M'),
                    Some(&b'M'),
                    Some(&b'S'),
                    Some(&b'S'),
                    Some(&b'A')
                ) | (
                    Some(&b'S'),
                    Some(&b'M'),
                    Some(&b'M'),
                    Some(&b'S'),
                    Some(&b'A')
                ) | (
                    Some(&b'S'),
                    Some(&b'S'),
                    Some(&b'M'),
                    Some(&b'M'),
                    Some(&b'A')
                ) | (
                    Some(&b'M'),
                    Some(&b'S'),
                    Some(&b'S'),
                    Some(&b'M'),
                    Some(&b'A')
                )
            )
        })
        .count())
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
