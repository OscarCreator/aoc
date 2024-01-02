use std::{fs, ops::{DerefMut, SubAssign, AddAssign, Div, Not}, borrow::BorrowMut};

use itertools::Itertools;

struct StringBox {
    content: String,
}

impl StringBox {
    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.content.lines().nth(y)?.chars().nth(x)
    }

    fn find(&self, c: char) -> Point {
        for (y, line) in self.content.lines().enumerate() {
            if let Some(x) = line.find(c) {
                return Point {
                    x,
                    y,
                    c,
                    dir: Direction::UNKNOWN,
                    count: 0,
                };
            }
        }
        unreachable!("can't find char: {}", c);
    }

    fn possible_moves(p: &Point) -> Vec<Move> {
        use Direction::*;

        match &p.dir {
            NORTH => {
                vec![Move { x: p.x, y: p.y - 1 }]
            }
            EAST => {
                vec![Move { x: p.x + 1, y: p.y }]
            }
            SOUTH => {
                vec![Move { x: p.x, y: p.y + 1 }]
            }
            WEST => {
                vec![Move { x: p.x - 1, y: p.y }]
            }
            UNKNOWN => {
                vec![
                    Move { x: p.x + 1, y: p.y },
                    Move { x: p.x - 1, y: p.y },
                    Move { x: p.x, y: p.y + 1 },
                    Move { x: p.x, y: p.y - 1 },
                ]
            }
        }
    }

    fn permiteter(&self) -> Vec<Point> {
        let mut points = vec![];
        let start: Point = self.find('S');
        points.push(start);
        let mut next: Point = self.next(&start);

        while next.c != 'S' {
            points.push(next);
            next = self.next(&next);
        }
        points
    }

    fn area(&self) -> f32 {
        use Direction::*;

        let mut points = self.permiteter();


        let furthest_point = points
            .iter()
            .reduce(|a, b| {
                if b.x >= a.x && b.y >= a.y {
                    b
                } else {
                    a
                }
            });

        match furthest_point {
            Some(p) => {
                assert_eq!(p.c, 'J');
                match p.dir {
                    Direction::NORTH => { 
                        // counter clockwise, nothing to do
                    }, 
                    Direction::EAST => panic!("east"),
                    Direction::SOUTH => panic!("south"),
                    Direction::WEST => {
                        // clockwise, need to flip

                        points.reverse();
                        for p in points.iter_mut() {
                            p.flip();
                        }
                    },
                    Direction::UNKNOWN => panic!("unknown"),
                }
            },
            None => {
                unreachable!("furthest point")
            },
        }

        let (s_index, s) = points.iter().find_position(|p| p.c == 'S').unwrap();
        let previous_index = (s_index - 1) % points.len();
        let next_index = (s_index + 1) % points.len();
        //assert_eq!(s_index, 0);
        let previous = points.get(previous_index).unwrap();
        let next = points.get(next_index).unwrap();

        let (new_c, new_dir) = match (previous.dir, next.dir) {
            (EAST, NORTH) => ('J', NORTH),
            (EAST, SOUTH) => ('7', SOUTH),
            (EAST, WEST) => {
                match next.c {
                    '7' => ('J', NORTH),
                    'J' => ('7', SOUTH),
                    _ => panic!("east")
                }
            },

            (WEST, NORTH) => ('L', NORTH),
            (WEST, SOUTH) => ('F', SOUTH),
            (WEST, EAST) => {
                match next.c {
                    'F' => ('L', NORTH),
                    'L' => ('F', SOUTH),
                    _ => panic!("west")
                }
            },

            (NORTH, EAST) => ('F', EAST),
            (NORTH, WEST) => ('7', WEST),
            (NORTH, SOUTH) => {
                match next.c {
                    '7' => ('F', EAST),
                    'F' => ('7', WEST),
                    _ => panic!("north")
                }
            },

            (SOUTH, EAST) => ('L', EAST),
            (SOUTH, WEST) => ('J', WEST),
            (SOUTH, NORTH) => {
                match next.c {
                    '7' => ('J', WEST),
                    'J' => ('7', EAST),
                    _ => panic!("south")
                }
            },

            (EAST, EAST) => ('-', EAST),
            (WEST, WEST) => ('-', WEST),
            (NORTH, NORTH) => ('|', NORTH),
            (SOUTH, SOUTH) => ('|', SOUTH),
            _ => panic!("ee {:?}, {:?}", previous, next)
        };

        let s = points.get_mut(s_index).unwrap();
        s.c = new_c;
        s.dir = new_dir;

        // get offset of points by 0.5 in
        let mut offset_points: Vec<Vec2f> = points.into_iter().map(|p| {
            let x: f32 = p.x as f32;
            let y: f32 = p.y as f32;
            match (p.c, p.dir) {
                ('L', EAST) => Vec2f {x: x + 0.5, y: y - 0.5},
                ('L', NORTH) => Vec2f {x: x - 0.5, y: y + 0.5},
                ('F', SOUTH) => Vec2f {x: x + 0.5, y: y + 0.5},
                ('F', EAST) => Vec2f {x: x - 0.5, y: y - 0.5},
                ('J', NORTH) => Vec2f {x: -0.5 + x, y: -0.5 + y},
                ('J', WEST) => Vec2f {x: 0.5 + x, y: 0.5 + y},
                ('7', WEST) => Vec2f {x: -0.5 + x, y: 0.5 + y},
                ('7', SOUTH) => Vec2f {x: 0.5 + x, y: -0.5 + y},
                ('|', NORTH) => Vec2f {x: -0.5 + x, y},
                ('|', SOUTH) => Vec2f {x: 0.5 + x, y},
                ('-', EAST) => Vec2f {x: 0.0 + x, y: -0.5 + y},
                ('-', WEST) => Vec2f {x: 0.0 + x, y: 0.5 + y},
                _ => panic!("ee {:?}", p)
            }
        }).collect();

        let first = offset_points.first().unwrap();
        let last = offset_points.last().unwrap();

        // shoelace formula to get area
        (last.x * first.y - first.x * last.y + offset_points.windows(2).map(|p| {
            let a = &p[0];
            let b = &p[1];
            a.x * b.y - b.x * a.y
        }).sum::<f32>()).div(2.0).abs()
    }

    fn next(&self, p: &Point) -> Point {
        use Direction::*;

        let moves = Self::possible_moves(&p);

        for m in moves {
            let next_char = self.get(m.x, m.y);
            let next_direction = match (next_char, &p.dir) {
                (Some('|'), NORTH | SOUTH) => Some(p.dir),
                (Some('|'), UNKNOWN) if m.y > p.y => Some(SOUTH),
                (Some('|'), UNKNOWN) if m.y < p.y => Some(NORTH),
                (Some('-'), WEST | EAST) => Some(p.dir),
                (Some('-'), UNKNOWN) if m.x > p.x => Some(EAST),
                (Some('-'), UNKNOWN) if m.x < p.x => Some(WEST),
                (Some('L'), UNKNOWN) if m.x < p.x => Some(NORTH),
                (Some('L'), UNKNOWN) if m.y < p.y => Some(EAST),
                (Some('L'), WEST) => Some(NORTH),
                (Some('L'), SOUTH) => Some(EAST),
                (Some('J'), UNKNOWN) if m.x > p.x => Some(NORTH),
                (Some('J'), UNKNOWN) if m.y < p.y => Some(WEST),
                (Some('J'), SOUTH) => Some(WEST),
                (Some('J'), EAST) => Some(NORTH),
                (Some('7'), UNKNOWN) if m.x > p.x => Some(SOUTH),
                (Some('7'), UNKNOWN) if m.y > p.y => Some(WEST),
                (Some('7'), NORTH) => Some(WEST),
                (Some('7'), EAST) => Some(SOUTH),
                (Some('F'), UNKNOWN) if m.x < p.x => Some(SOUTH),
                (Some('F'), UNKNOWN) if m.y > p.y => Some(EAST),
                (Some('F'), NORTH) => Some(EAST),
                (Some('F'), WEST) => Some(SOUTH),
                // this will terminate it
                (Some('S'), _) => Some(UNKNOWN),
                _ => None,
            };
            if let Some(dir) = next_direction {
                return Point {
                    x: m.x,
                    y: m.y,
                    c: next_char.unwrap(),
                    dir,
                    count: p.count + 1,
                };
            }
        }
        unreachable!("should always find anther");
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2f {
    x: f32,
    y: f32,
}

impl std::ops::Sub for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl std::ops::Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


struct Move {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    UNKNOWN,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    c: char,
    // direction where we came from
    dir: Direction,
    count: usize,
}

impl Point {
    fn flip(&mut self) {
        use Direction::*;
        match (self.c, self.dir) {
            ('F', EAST) => self.dir = SOUTH,
            ('F', _) => self.dir = EAST,
            ('L', EAST) => self.dir = NORTH,
            ('L', _) => self.dir = EAST,
            ('J', NORTH) => self.dir = WEST,
            ('J', _) => self.dir = NORTH,
            ('7', SOUTH) => self.dir = WEST,
            ('7', _) => self.dir = SOUTH,
            ('-', WEST) => self.dir = EAST,
            ('-', EAST) => self.dir = WEST,
            ('|', NORTH) => self.dir = SOUTH,
            ('|', SOUTH) => self.dir = NORTH,
            ('S', _) => {}, // ignore
            _ => panic!("point flip error {:?}", self)
        }
    }
}

fn part1() -> usize {
    let content = fs::read_to_string("src/23/day10/input.txt").unwrap();

    let sb = StringBox { content };
    num::Integer::div_ceil(&(sb.permiteter().last().unwrap().count), &2)
}

fn part2() -> f32 {
    let content = fs::read_to_string("src/23/day10/input.txt").unwrap();

    let sb = StringBox { content };
    sb.area()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{Direction::*, Point, StringBox};

    #[test_case(".....\n.S-7.\n.|.|.\n.L-J.\n.....\n")]
    fn test_get(lines: &str) {
        let sb = StringBox {
            content: lines.to_owned(),
        };
        assert_eq!(sb.get(0, 0).unwrap(), '.');
        assert_eq!(sb.get(1, 1).unwrap(), 'S');
    }

    #[test_case(".....\n.S-7.\n.|.|.\n.L-J.\n.....\n")]
    fn test_next(lines: &str) {
        let sb = StringBox {
            content: lines.to_owned(),
        };
        let start: Point = sb.find('S');
        assert_eq!(
            start,
            Point {
                x: 1,
                y: 1,
                c: 'S',
                dir: UNKNOWN,
                count: 0
            }
        );

        let mut next = sb.next(&start);
        assert_eq!(
            next,
            Point {
                x: 2,
                y: 1,
                c: '-',
                dir: EAST,
                count: 1
            }
        );
        next = sb.next(&next);
        assert_eq!(
            next,
            Point {
                x: 3,
                y: 1,
                c: '7',
                dir: SOUTH,
                count: 2
            }
        );
    }

    //..F7F7
    //..|LJ|
    //F-S..|
    //|.F--J
    //|.L-7.
    //L---J.
    #[test_case(".....\n.S-7.\n.|.|.\n.L-J.\n.....\n", 4)]
    #[test_case("...F7.\n..FJ|.\n.SJ.L7\n.|F--J\n.LJ...", 8)]
    #[test_case("..F7F7\n..|LJ|\nF-S..|\n|.F--J\n|.L-7.\nL---J.", 13)]
    fn test_full(lines: &str, count: usize) {
        let sb = StringBox {
            content: lines.to_owned(),
        };
        let mut next: Point = sb.find('S');

        loop {
            println!("{:?}", next);
            next = sb.next(&next);
            if next.c == 'S' {
                println!("break");
                break;
            }
        }

        assert_eq!(num::Integer::div_ceil(&(next.count - 1), &2), count);
    }

    
    // problem is `x` are counted
    //...........
    //.S-------7.
    //.|F-----7|.
    //.||...x.||.
    //.||...x.||.
    //.|L-7.F-J|.
    //.|..|.|..|.
    //.L--J.L--J.
    //...........
    #[test_case("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........", 4.0)]
    #[test_case(".....\n.S-7.\n.|.|.\n.L-J.\n.....\n", 1.0)]
    #[test_case("..\n.S7\n.LJ", 0.0)]
    fn test_area(lines: &str, area: f32) {
        let sb = StringBox {
            content: lines.to_owned(),
        };

        assert_eq!(sb.area(), area);
    }
}
