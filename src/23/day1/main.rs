use std::fs;


fn solve1() -> u32 {
    return fs::read_to_string("src/23/day1/input.txt").expect("error")
        .lines()
        .map(|s| {
            s.chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        }).map(|d: Vec<u32>| {
            d.first().unwrap() * 10 + d.last().unwrap()
        }).sum::<u32>();
}

fn solve2() -> u32 {
    return fs::read_to_string("src/23/day1/input.txt").expect("error")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .lines()
        .map(|s| {
            s.chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        }).map(|d: Vec<u32>| {
            d.first().unwrap() * 10 + d.last().unwrap()
        }).sum::<u32>();
}

#[cfg(test)]
pub mod test {
    use crate::{solve1, solve2};


    #[test]
    fn test() {
        println!("{}", solve1());
        println!("{}", solve2());
    }

}

fn main() {
}
