use std::fs;


fn solve1() -> u32 {
    return fs::read_to_string("src/23/day1/input.txt")
        .expect("error")
        .lines()
        .filter_map(|s| {
            Some(s.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect())
        }).map(|d: Vec<u32>| {
            let a = format!("{}{}", d.first().unwrap(), d.last().unwrap());
            a.parse::<u32>().unwrap()
        }).sum::<u32>();
}

fn solve2() -> u32 {
    return fs::read_to_string("src/23/day1/input.txt")
        .expect("error")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .lines()
        .filter_map(|s| {
            Some(s.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect())
        }).map(|d: Vec<u32>| {
            let a = format!("{}{}", d.first().unwrap(), d.last().unwrap());
            a.parse::<u32>().unwrap()
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
