use std::{
    fmt::{Debug, Write},
    fs::{self},
};

#[derive(PartialEq, Eq, Clone)]
struct Platform {
    v: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for l in &self.v {
            for &c in l {
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Platform {
    fn new(s: &str) -> Self {
        let columns = s.lines().count();
        let rows = s.lines().nth(0).unwrap().chars().count();
        Self {
            v: s.lines().map(|s| s.chars().collect()).collect(),
            rows,
            columns,
        }
    }

    fn get(&self, x: usize, y: usize, dir: Direction) -> char {
        match dir {
            Direction::North => self.v[y][x],
            Direction::East => self.v[self.columns - 1 - x][self.rows - 1 - y],
            Direction::South => self.v[self.rows - y - 1][x],
            Direction::West => self.v[self.columns - 1 - x][y],
        }
    }

    fn size(&self, dir: Direction) -> (usize, usize) {
        match dir {
            Direction::North | Direction::South => (self.columns, self.rows),
            Direction::East | Direction::West => (self.rows, self.columns),
        }
    }

    fn set(&mut self, dest: (usize, usize), c: char, dir: Direction) {
        match dir {
            Direction::North => self.v[dest.1][dest.0] = c,
            Direction::East => self.v[self.columns - 1 - dest.0][self.rows - 1 - dest.1] = c,
            Direction::South => self.v[self.rows - dest.1 - 1][dest.0] = c,
            Direction::West => self.v[self.columns - 1 - dest.0][dest.1] = c,
        }
    }

    fn replace(&mut self, from: (usize, usize), to: (usize, usize), dir: Direction) {
        if from != to {
            self.set(from, '.', dir);
            self.set(to, 'O', dir);
        }
    }

    fn tilt(&mut self, dir: Direction) {
        let size = self.size(dir);
        // start at direction e.g North = top + 1 unit
        // go though each chars left to right, top to bottom
        for y in 0..size.1 {
            for x in 0..size.0 {
                // if found a 'O' navigate in the direction until found '#'/'O'
                if self.get(x, y, dir) == 'O' {
                    let mut blocking = y;
                    for i in (0..y).rev() {
                        match self.get(x, i, dir) {
                            '#' | 'O' => {
                                break;
                            }
                            _ => {
                                blocking = i;
                            }
                        }
                    }
                    // move the 'O' to one before that index
                    self.replace((x, y), (x, blocking), dir);
                }
            }
        }
    }

    fn load(&self) -> usize {
        (0..self.rows)
            .map(|y| {
                (0..self.columns)
                    .map(|x| match self.get(x, y, Direction::North) {
                        'O' => self.rows - y,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn parse() -> Platform {
    let file = fs::read_to_string("src/23/day14/input.txt").unwrap();
    Platform::new(&file)
}

fn part1() -> usize {
    let mut p = parse();
    p.tilt(Direction::North);
    p.load()
}

// get stabilized count
// get cycle count
// cycle_index = (1_000_000_000 - stabilized_count) % cycle_count
fn part2() -> usize {
    let mut p = parse();
    let dirs = vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut loads = vec![p.load()];
    loop {
        for dir in &dirs {
            p.tilt(*dir);
        }

        let current_load = p.load();
        loads.push(current_load);
        if let Some((stable, cycle)) = has_repeating_slice(&loads) {
            let index = (1_000_000_000 - stable) % cycle + stable;
            dbg!(&p);
            return *loads.get(index).unwrap();
        }
    }
}

/// return (stabilized_count, cycle_count)
fn has_repeating_slice(v: &Vec<usize>) -> Option<(usize, usize)> {
    let mut i = v.len() - 2;
    loop {
        let part = &v[i..v.len()];
        let mut double_part = part.to_vec();
        double_part.append(&mut part.to_vec());

        if part.len() > i {
            // checking outside of range
            return None;
        }
        let double_range = (i - part.len())..v.len();
        if v[double_range] == double_part {
            return Some((i - 1 - part.len(), part.len()));
        }
        if i == 0 {
            return None;
        }
        i -= 1;
    }
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{has_repeating_slice, Direction, Platform};

    #[test_case(&[1,2,3], None)]
    #[test_case(&[0,1,2,3,1,2,3], Some((0, 3)))]
    fn test_subrepeating(v: &[usize], expect: Option<(usize, usize)>) {
        assert_eq!(has_repeating_slice(&v.to_vec()), expect);
    }

    //...#.
    //#....
    //O.OO.
    //...#.
    //.O.O.
    #[test_case(
        "...#.\n#....\nO.OO.\n...#.\n.O.O.",
        ".OO#.\n#..O.\nO....\n...#.\n...O.",
        Direction::North,
        18
    )]
    #[test_case(
        "...#.\n#....\nO.OO.\n...#.\n.O.O.",
        "...#.\n#....\n...O.\n...#.\nOOOO.",
        Direction::South,
        7
    )]
    #[test_case(
        "...#.\n#....\nO.OO.\n...#.\n.O.O.",
        "...#.\n#....\nOOO..\n...#.\nOO...",
        Direction::West,
        11
    )]
    #[test_case(
        "...#.\n#....\nO.OO.\n...#.\n.O.O.",
        "...#.\n#....\n..OOO\n...#.\n...OO",
        Direction::East,
        11
    )]
    #[test_case(".O\n..", "..\n.O", Direction::South, 1)]
    fn test_tilt(input: &str, expect: &str, dir: Direction, load: usize) {
        let mut p = Platform::new(input);
        p.tilt(dir);
        assert_eq!(p, Platform::new(expect));
        assert_eq!(p.load(), load)
    }

    #[test_case("..\nOO", "O.\n.O")]
    fn test_replace(input: &str, expect: &str) {
        let mut p = Platform::new(input);
        p.replace((0, 1), (0, 0), Direction::North);
        assert_eq!(p, Platform::new(expect))
    }

    #[test_case("12\n34", '1', Direction::North)]
    #[test_case("12\n34", '3', Direction::West)]
    #[test_case("12\n34", '3', Direction::South)]
    #[test_case("12\n34", '4', Direction::East)]
    fn test_get00(input: &str, expect: char, dir: Direction) {
        let p = Platform::new(input);
        assert_eq!(p.get(0, 0, dir), expect)
    }

    #[test_case("12\n34", '2', Direction::North)]
    #[test_case("12\n34", '1', Direction::West)]
    #[test_case("12\n34", '4', Direction::South)]
    #[test_case("12\n34", '2', Direction::East)]
    fn test_get10(input: &str, expect: char, dir: Direction) {
        let p = Platform::new(input);
        assert_eq!(p.get(1, 0, dir), expect)
    }
}
