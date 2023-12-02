use std::fs;


fn solve1() -> u32 {
    return fs::read_to_string("src/23/day2/input.txt").expect("error")
        .lines()
        .filter_map(|l| {
            let (game_text, cubes_text) = l.split_once(':').unwrap();
            let game: u32 = game_text.replace("Game", "").trim().parse().unwrap();

            for s in cubes_text.split(';') {
                for ele in s.split(',') {
                    let (num, color) = ele.trim().split_once(' ').unwrap();
                    let count: u32 = num.parse().unwrap();
                    match color.trim() {
                        "blue" => {
                            if count > 14 {
                                return None;
                            }
                        }
                        "red" => {
                            if count > 12 {
                                return None;
                            }
                        }
                        "green" => {
                            if count > 13 {
                                return None;
                            }
                        }
                        _ => panic!("should never happen")
                    }
                }
            }
            Some(game)
        }).sum();
}

fn solve2() -> u32 {
    return fs::read_to_string("src/23/day2/input.txt").expect("error")
        .lines()
        .map(|l| {
            let (_, cubes_text) = l.split_once(':').unwrap();

            let mut min_blue = 0;
            let mut min_red = 0;
            let mut min_green = 0;
            for s in cubes_text.split(';') {
                for ele in s.split(',') {
                    let (num, color) = ele.trim().split_once(' ').unwrap();
                    let count: u32 = num.parse().unwrap();
                    match color.trim() {
                        "blue" => {
                            if count > min_blue {
                                min_blue = count;
                            }
                        }
                        "red" => {
                            if count > min_red {
                                min_red = count;
                            }
                        }
                        "green" => {
                            if count > min_green {
                                min_green = count;
                            }
                        }
                        _ => panic!("should never happen")
                    }
                }
            }
            min_green * min_red * min_blue
        }).sum();
}


fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
