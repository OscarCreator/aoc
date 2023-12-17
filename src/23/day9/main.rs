use std::fs;

use itertools::Itertools;

struct Sequence {
    nums: Vec<i64>,
}

impl Sequence {
    // estimate next value in sequence
    fn estimate_next(&self) -> i64 {
        let mut total = *self.nums.last().unwrap();
        let mut current = Sequence::diff(self);

        loop {
            total = total + current.nums.last().unwrap();
            if current.nums.iter().all_equal() {
                return total;
            }
            current = Sequence::diff(&current);
        }
    }

    fn estimate_previous(&self) -> i64 {
        let mut total = *self.nums.first().unwrap();
        let mut current = Sequence::diff(self);
        let mut iter = 1;
        loop {
            let first = current.nums.first().unwrap();
            total = if iter % 2 == 0 {
                total + first
            } else {
                total - first
            };
            iter = iter + 1;

            if current.nums.iter().all_equal() {
                return total;
            }
            current = Sequence::diff(&current);
        }
    }

    fn diff(sequence: &Self) -> Self {
        Sequence {
            nums: sequence.nums.windows(2).map(|n| n[1] - n[0]).collect(),
        }
    }

    fn new(line: &str) -> Self {
        Sequence {
            nums: line
                .split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect(),
        }
    }
}

fn part1() -> i64 {
    let sequences: Vec<Sequence> = fs::read_to_string("src/23/day9/input.txt")
        .unwrap()
        .lines()
        .map(|l| Sequence::new(l))
        .collect();

    sequences.iter().map(|s| s.estimate_next()).sum()
}

fn part2() -> i64 {
    let sequences: Vec<Sequence> = fs::read_to_string("src/23/day9/input.txt")
        .unwrap()
        .lines()
        .map(|l| Sequence::new(l))
        .collect();

    sequences.iter().map(|s| s.estimate_previous()).sum()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
    // sum all estimates
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::Sequence;

    // A0 A1 A2 A3 A4
    //  B0 B1 B2 B3
    //    C0 C1 C2
    //     D0 D1
    //
    // D0 = D1
    // C0 = C1 - D0
    // B0 = B1 - C0
    // A0 = A1 - B0 = A1 - (B1 - C0) = A1 - (B1 - (C1 - D0))
    // = A1 - B1 + (C1 - D0) = A1 - B1 + C1 - D1

    #[test_case("10 13 16 21 30 45", 68, 5)]
    #[test_case("1 3 6 10 15 21", 28, 0)]
    fn test(nums: &str, next: i64, previous: i64) {
        assert_eq!(Sequence::new(nums).estimate_next(), next);
        assert_eq!(Sequence::new(nums).estimate_previous(), previous);
    }
}
