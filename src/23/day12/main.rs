use std::{vec, fmt::Debug, fs};

use itertools::Itertools;

struct A {
    line: String,
    continues_groups: Vec<usize>,
}

impl A {
    fn new(line: String) -> Self {
        let (code, group_str) = line.split_once(' ').unwrap();
        let groups: Vec<usize> = group_str.split(',').map(|i| i.parse().unwrap()).collect();
        
        Self { line: code.to_owned(), continues_groups: groups }
    }

    fn new2(line: String) -> Self {
        let (code, group_str) = line.split_once(' ').unwrap();
        let mut groups: Vec<usize> = group_str.split(',').map(|i| i.parse().unwrap()).collect();
        let copied_code = format!("{0}?{0}?{0}?{0}?{0}", code);
        let mut copied_groups = groups.clone();
        copied_groups.append(&mut groups.clone());
        copied_groups.append(&mut groups.clone());
        copied_groups.append(&mut groups.clone());
        copied_groups.append(&mut groups.clone());
        
        Self { line: copied_code, continues_groups: copied_groups }
    }

    // brute force:
    // create all different possible combinations of replacing '?'
    // count valid
    fn combinations(&self) -> usize {
        let total_count: usize = self.continues_groups.iter().sum();
        let counts = self.line.chars().counts();
        let hash_count = counts.get(&'#').unwrap_or(&0);
        let unknown_points: Vec<usize> = self.line.chars().enumerate().filter_map(|(i, c)| {
            if c == '?' {
                Some(i)
            } else {
                None
            }
        }).collect();
        let replace_count = total_count - hash_count;
        let combinations = groups(&unknown_points, replace_count);

        combinations.into_iter().map(|combination| {
            let mut line = self.line.clone();
            for index in combination {
                line.replace_range(index..=index, "#");
            }
            let line = line.replace("?", ".");
            let splits = line.split(".").filter(|s| !s.is_empty()).map(|s| s.len()).collect_vec();
            if splits.eq(&self.continues_groups) {
                1
            } else {
                0
            }
        }).sum::<usize>().max(1)
    }

}

fn groups<T>(v: &Vec<T>, size: usize) -> Vec<Vec<T>> 
where 
    T: Copy + Clone + Debug,
{
    if size == 1 {
        return v.iter().map(|&e| vec![e]).collect();
    } else if size == 0 {
        return vec![];
    }
    v.iter().enumerate().map(|(i, e)| {
        let groups = groups(v[i+1..v.len()].to_vec().as_ref(), size - 1);
        groups.into_iter().map(|mut group| {
            group.push(*e);
            group
        }).collect_vec()
    }).flatten().collect()
}

fn part1() -> usize {
    let file = fs::read_to_string("src/23/day12/input.txt").unwrap();
    let lines = file.lines().collect_vec();
    lines.into_iter().map(|line| {
        let a = A::new(line.to_owned());
        let count = a.combinations();
        dbg!(&line, count);
        count
    }).sum()
}

// TODO need to optimize, takes to long to run
//
// instead of doing brute force we need to narrow it down from the edges
// like:
// ?#?#?#?#?#?#?#? 1,3,1,6
// > .#.#?#?#?#?#?#? 3,1,6
// > .#.###.#?#?#?#? 1,6
// > .#.###.#.#?#?#? 6
// > .#.###.#.###### 6
// = 1
//
// 1. find first cluster (chars closed of with '.'/'?' or have max '#' == to current group)
// 2. for each new possible string continue with next group
// 3.1   ('?#', 1) brute force find possible strings
//   > .#.#?#?#?#?#?#? 3,1,6
// 3.2   ('#?#?#', 3) brute force find possible strings
//   > .#.###.#?#?#?#? 1,6
// 3.3   ('#', 1) brute force find possible strings
//   > .#.###.#.#?#?#? 6
// 3.4   ('#?#?#?', 6) brute force find possible strings
//   > .#.###.#.######
// 4. sum counts
//   > 1
fn part2() -> usize {
    let file = fs::read_to_string("src/23/day12/input.txt").unwrap();
    let lines = file.lines().collect_vec();
    lines.into_iter().map(|line| {
        let a = A::new2(line.to_owned());
        let count = a.combinations();
        dbg!(&line, count);
        count
    }).sum()
}


fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use std::vec;

    use test_case::test_case;

    use crate::A;
    use crate::groups;

    #[test_case(".???#?.??? 1,3,3", 1)]
    #[test_case(".????#?.??? 1,3,3", 3)]
    #[test_case("??.??#??##.?.?# 2,3,2,1,2", 2)]
    #[test_case("?#???#.?##? 1,1,2", 1)]
    fn test(line: &str, count: usize) {
        let a = A::new(line.to_owned());
        //assert_eq!(a.continues_groups, vec![1, 3, 3]);
        assert_eq!(a.combinations(), count);
    }

    #[test_case("????.#...#... 4,1,1", 16)]
    #[test_case("????.######..#####. 1,6,5", 2500)]
    #[test_case(".??..??...?##. 1,1,3", 16384)]
    fn test2(line: &str, count: usize) {
        let a = A::new2(line.to_owned());
        //assert_eq!(a.continues_groups, vec![1, 3, 3]);
        assert_eq!(a.combinations(), count);
    }

    #[test]
    fn test_groups() {

        let a = vec![1, 2];
        assert_eq!(groups(&a, 1), vec![vec![1], vec![2]]);
        let a = vec![1, 2, 3];
        assert_eq!(groups(&a, 2), vec![vec![2, 1], vec![3, 1], vec![3, 2]]);
        let a = vec![1, 2, 3, 4];
        assert_eq!(groups(&a, 3), vec![vec![3, 2, 1], vec![4, 2, 1], vec![4, 3, 1], vec![4, 3, 2]]);
    }

}
