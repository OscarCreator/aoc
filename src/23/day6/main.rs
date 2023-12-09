use std::fs;


fn simulate(hold_time: u64, max_time: u64) -> u64 {
    let speed = hold_time;
    let run_time = max_time - hold_time;
    speed * run_time
}

fn part1() -> u64 {
    let string = fs::read_to_string("src/23/day6/input.txt").unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let time_str: &str = lines.get(0).unwrap();
    let distance_str: &str = lines.get(1).unwrap();

    let (_, times_str) = time_str.split_once(':').unwrap();
    let (_, distances_str) = distance_str.split_once(':').unwrap();
    let times: Vec<u64> = times_str.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let distances: Vec<u64> = distances_str.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let mut win_counts = Vec::new();
    for (time, distance) in times.iter().zip(distances) {
        let mut win_count = 0;
        for t in 0..*time {
            if simulate(t, *time) > distance {
                win_count = win_count + 1;
            }
        }
        win_counts.push(win_count);
    }

    win_counts.into_iter().reduce(|a, b| a * b).unwrap()
}

fn part2() -> u64 {
    let string = fs::read_to_string("src/23/day6/input.txt").unwrap();
    let lines: Vec<&str> = string.lines().collect();

    let time_str: &str = lines.get(0).unwrap();
    let distance_str: &str = lines.get(1).unwrap();

    let (_, times_str) = time_str.split_once(':').unwrap();
    let (_, distances_str) = distance_str.split_once(':').unwrap();
    let time: u64 = times_str.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().unwrap();
    let distance: u64 = distances_str.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().unwrap();

    let mut win_counts = Vec::new();
    let mut win_count = 0;
    for t in 0..time {
        if simulate(t, time) > distance {
            win_count = win_count + 1;
        }
    }
    win_counts.push(win_count);

    win_counts.into_iter().reduce(|a, b| a * b).unwrap()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}
