use std::{collections::HashMap, fs};

use itertools::Itertools;

struct Operation<'a> {
    label: &'a str,
    operator: char,
    focal_length: usize,
}

impl<'a> Operation<'a> {
    fn new(s: &'a str) -> Self {
        let (index, _) = s.chars().find_position(|&a| a == '=' || a == '-').unwrap();
        Operation {
            label: &s[0..index],
            operator: s.chars().nth(index).unwrap(),
            focal_length: s[(index + 1)..].parse().unwrap_or(0),
        }
    }

    fn box_number(&self) -> usize {
        hash(self.label)
    }
}

fn hash(s: &str) -> usize {
    let mut total = 0;
    for c in s.chars() {
        total += c as usize;
        total *= 17;
        total %= 256;
    }
    total
}

fn part1() -> usize {
    let file = fs::read_to_string("src/23/day15/input.txt").unwrap();
    file.trim().split(',').map(hash).sum()
}

fn part2() -> usize {
    let file = fs::read_to_string("src/23/day15/input.txt").unwrap();
    let mut map = HashMap::<usize, Vec<Operation>>::new();
    file.trim().split(',').for_each(|s| {
        let o = Operation::new(s);
        let bnum = o.box_number();
        match o.operator {
            '=' => {
                if let Some(v) = map.get_mut(&bnum) {
                    if let Some((i, _)) = v.iter().find_position(|a| a.label == o.label) {
                        v[i] = o;
                    } else {
                        v.push(o);
                    }
                } else {
                    map.insert(bnum, vec![o]);
                }
            }
            '-' => {
                if let Some(v) = map.get_mut(&bnum) {
                    if let Some((i, _)) = v.iter().find_position(|a| a.label == o.label) {
                        v.remove(i);
                    }
                }
            }
            _ => unreachable!("bla"),
        };
    });
    map.into_iter()
        .map(|(bnum, v)| {
            v.iter()
                .enumerate()
                .map(|(i, o)| {
                    (bnum + 1) * (i + 1) * o.focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::hash;

    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("qp=3", 97)]
    fn test_hash(s: &str, hash_value: usize) {
        assert_eq!(hash(s), hash_value);
    }
}
