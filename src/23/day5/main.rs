use std::fs;


struct Seed {
    number: u64,
}

#[derive(Debug)]
struct Converter {
    source: u64,
    dest: u64,
    range: u64,
}

impl Converter {
    fn convert(&self, value: u64) -> Option<u64> {
        if !(self.source..self.source+self.range).contains(&value) {
            return None;
        }
        let diff: i64 = self.dest as i64 - self.source as i64;
        Some((value as i64 + diff) as u64)
    }
}



#[derive(Debug)]
struct ConverterMap {
    convert_ranges: Vec<Converter>,
}

impl ConverterMap {
    fn convert(&self, value: u64) -> u64 {
        // will only be one range that can convert the value
        for c in &self.convert_ranges {
            if let Some(converted) = c.convert(value) {
                return converted;
            }
        }
        value
    }
}

fn part1() -> u64 {
    let file = fs::read_to_string("src/23/day5/input.txt").expect("error");
    let groups: Vec<&str> = file.split("\n\n").collect();

    let seeds: Vec<u64> = groups.get(0).unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap()).collect();

    let mut converters_maps: Vec<ConverterMap> = Vec::new();
    for group in groups {
        let lines = group.split('\n');
        let mut converters: Vec<Converter> = Vec::new();
        for (i, line) in lines.enumerate() {
            if i == 0 {
                continue;
            }
            let nums: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            // skip last line
            if nums.len() == 0 {
                continue;
            }
            assert!(nums.len() == 3);
            converters.push(Converter {
                dest: *nums.get(0).unwrap(),
                source: *nums.get(1).unwrap(),
                range: *nums.get(2).unwrap(),
            })

        }
        converters_maps.push(ConverterMap {
            convert_ranges: converters

        })
    }
    let locations: Vec<u64> = seeds.into_iter().map(|l| {
        let mut current_num = l;
        for converter in &converters_maps {
            current_num = converter.convert(current_num);
        }
        current_num
    }).collect();
    *locations.iter().min().unwrap()
}

// TODO: solution
// 'compile' all Converters into one.
// find smallest range and try find a value in range
fn part2() -> u64 {
    let file = fs::read_to_string("src/23/day5/input.txt").expect("error");
    let groups: Vec<&str> = file.split("\n\n").collect();

    let seed_maps: Vec<u64> = groups.get(0).unwrap()
        .split_once(':').unwrap().1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap()).collect();
    let mut seeds: Vec<u64> = Vec::new();
    dbg!(&seed_maps);
    for i in 0..(seed_maps.len() / 2)  {
        dbg!(i);
        let first = *seed_maps.get(i * 2).unwrap();
        let second = *seed_maps.get(i * 2 + 1).unwrap();
        for i in 0..second {
            seeds.push(first + i);
        }
    }
    //dbg!(&seeds);

    let mut converters_maps: Vec<ConverterMap> = Vec::new();
    let mut a = 0;
    for group in groups {
        dbg!(a);
        a = a + 1;
        let lines = group.split('\n');
        let mut converters: Vec<Converter> = Vec::new();
        for (i, line) in lines.enumerate() {
            if i == 0 {
                continue;
            }
            let nums: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            // skip last line
            if nums.len() == 0 {
                continue;
            }
            assert!(nums.len() == 3);
            converters.push(Converter {
                dest: *nums.get(0).unwrap(),
                source: *nums.get(1).unwrap(),
                range: *nums.get(2).unwrap(),
            })

        }
        converters_maps.push(ConverterMap {
            convert_ranges: converters

        })
    }
    //let a 1_879_881_983
    let len: f64 = seeds.len() as f64;
    let mut a: f64 = 0.0;
    let locations: Vec<u64> = seeds.into_iter().map(|l| {
        dbg!(a / len, a, len);
        a = a + 1.0;
        let mut current_num = l;
        for converter in &converters_maps {
            current_num = converter.convert(current_num);
        }
        current_num
    }).collect();
    *locations.iter().min().unwrap()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use crate::{Converter, ConverterMap};


    #[test]
    fn test() {
        let conveter1 = Converter { source: 1, dest: 10, range: 2 };

        let converter_map = ConverterMap {
            convert_ranges: vec![conveter1],
        };

        assert_eq!(converter_map.convert(1), 10);
        assert_eq!(converter_map.convert(2), 11);
        assert_eq!(converter_map.convert(3), 3);
    }
}
