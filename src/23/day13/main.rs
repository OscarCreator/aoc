use itertools::Itertools;
use std::{fmt::Debug, fs};

struct Pattern {
    line: String,
}

impl Pattern {
    fn is_smudge_reflection<T>(i: usize, v: &Vec<T>) -> Option<usize>
    where
        T: PartialEq + IntoIterator + Clone + Debug,
        T::Item: PartialEq + Debug,
    {
        let mut x1 = i;
        let mut x2 = i + 1;
        let mut partial_eq = false;
        loop {
            let top = v.get(x1);
            let bottom = v.get(x2);
            if let (Some(t), Some(b)) = (top, bottom) {
                // return None if more than one element differs
                for (e1, e2) in t.clone().into_iter().zip(b.clone().into_iter()) {
                    if e1 != e2 {
                        if partial_eq {
                            return None;
                        }
                        partial_eq = true;
                    }
                }
            } else {
                break;
            }
            if x1 == 0 {
                break;
            }
            x1 -= 1;
            x2 += 1;
        }
        if partial_eq {
            Some(i + 1)
        } else {
            None
        }
    }

    fn is_reflection<T: PartialEq>(i: usize, v: &Vec<T>) -> Option<usize> {
        let mut x1 = i;
        let mut x2 = i + 1;
        loop {
            let top = v.get(x1);
            let bottom = v.get(x2);
            if let (Some(t), Some(b)) = (top, bottom) {
                if t != b {
                    return None;
                }
            } else {
                break;
            }
            if x1 == 0 {
                break;
            }
            x1 -= 1;
            x2 += 1;
        }
        Some(i + 1)
    }

    // find identical lines, both rows and columns
    fn solve<'a, F>(&'a self, f: F) -> usize
    where
        F: Fn(usize, &Vec<Vec<char>>) -> Option<usize>,
    {
        let rows: Vec<Vec<char>> = self
            .line
            .lines()
            .into_iter()
            .map(|s| s.chars().collect())
            .collect_vec();
        for i in 0..(rows.len() - 1) {
            if let Some(a) = f(i, &rows) {
                return a * 100;
            }
        }

        let columns: Vec<Vec<char>> = (0..self.line.lines().nth(0).unwrap().len())
            .map(|x| {
                self.line
                    .lines()
                    .enumerate()
                    .map(|(_, line)| line.chars().nth(x).unwrap())
                    .collect_vec()
            })
            .collect_vec();

        for i in 0..(columns.len() - 1) {
            if let Some(c) = f(i, &columns) {
                return c;
            }
        }
        unreachable!("should always have one column/row")
    }
}

fn part1() -> usize {
    parse()
        .iter()
        .map(|p| p.solve(Pattern::is_reflection))
        .sum()
}

fn part2() -> usize {
    parse()
        .iter()
        .map(|p| p.solve(Pattern::is_smudge_reflection))
        .sum()
}

fn parse() -> Vec<Pattern> {
    let file = fs::read_to_string("src/23/day13/input.txt").unwrap();
    let patterns = file.split("\n\n");
    patterns
        .map(|pattern| Pattern {
            line: pattern.to_owned(),
        })
        .collect()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use crate::Pattern;

    use test_case::test_case;

    #[test_case(
        "#..#.#........#
#..######..####
.##..#.#.##.#.#
#..##..........
######........#
#..####......##
.##.##.#...##.#",
        2,
        10
    )]
    #[test_case(
        "###..#....#####..
.#........#.#.###
.#........#.#.###
###..#....#####..
#.#.#####....####
.#.###..####.#..#
#..#..#.####...#.
#..#..#.####...#.
.#.###..##.#.#..#",
        200,
        700
    )]
    fn test(s: &str, sum: usize, sum2: usize) {
        let p = Pattern { line: s.to_owned() };
        assert_eq!(p.solve(Pattern::is_reflection), sum);
        assert_eq!(p.solve(Pattern::is_smudge_reflection), sum2);
    }
}
