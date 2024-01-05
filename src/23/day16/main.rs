use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    fs,
};

use itertools::Itertools;

struct CharBox {
    v: Vec<Vec<char>>,
    energized: HashMap<Beam, usize>,
    size: Point,
}

impl Debug for CharBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if let Some(_) = self
                    .energized
                    .iter()
                    .find(|(b, _)| b.p.x == x && b.p.y == y)
                {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl CharBox {
    fn new(s: &str, start: &Beam) -> Self {
        let mut map = HashMap::new();
        map.insert(*start, 1);
        Self {
            v: s.lines().map(|l| l.chars().collect()).collect(),
            energized: map,
            size: Point {
                x: s.lines().nth(0).unwrap().len(),
                y: s.lines().count(),
            },
        }
    }

    /// get char at x,y position
    fn get(&self, p: Point) -> Option<char> {
        if !p.is_in(self.size) {
            return None;
        }
        Some(self.v[p.y][p.x])
    }

    /// navigate to next position, return the beams as result of this char
    fn next(&mut self, mut b: Beam) -> Vec<Beam> {
        let mut beams = vec![];

        match self.get(b.p) {
            Some(c) => match (c, b.dir) {
                ('.', _)
                | ('|', Direction::Up | Direction::Down)
                | ('-', Direction::Right | Direction::Left) => {
                    if let Some(_) = b.next() {
                        if let None = self.energized.get(&b) {
                            if b.p.is_in(self.size) {
                                self.energized.insert(b, 1);
                                beams.push(b);
                            }
                        }
                    }
                }
                ('|', Direction::Left | Direction::Right)
                | ('-', Direction::Up | Direction::Down) => {
                    let splits = b.split();

                    for splitbeam in splits {
                        if splitbeam.p.is_in(self.size) {
                            if let None = self.energized.get(&splitbeam) {
                                self.energized.insert(splitbeam, 1);
                                beams.push(splitbeam);
                            }
                        }
                    }
                }
                ('/' | '\\', _) => {
                    if let Some(_) = b.deflect(c) {
                        if let None = self.energized.get(&b) {
                            if b.p.is_in(self.size) {
                                self.energized.insert(b, 1);
                                beams.push(b);
                            }
                        }
                    }
                }
                _ => panic!("error"),
            },
            None => {}
        }
        beams
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn is_in(&self, other: Point) -> bool {
        self.x < other.x && self.y < other.y
    }

    fn add(&self, x: i32, y: i32) -> Option<Self> {
        let tx = self.x as i32 + x;
        let ty = self.y as i32 + y;
        if let (Ok(nx), Ok(ny)) = (tx.try_into(), ty.try_into()) {
            return Some(Self { x: nx, y: ny });
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Beam {
    p: Point,
    dir: Direction,
}

impl Beam {
    fn next(&mut self) -> Option<()> {
        match self.dir {
            Direction::Up => {
                self.p.y = self.p.y.checked_sub(1)?;
            }
            Direction::Down => self.p.y = self.p.y.checked_add(1)?,
            Direction::Left => self.p.x = self.p.x.checked_sub(1)?,
            Direction::Right => self.p.x = self.p.x.checked_add(1)?,
        }
        Some(())
    }

    fn split(&self) -> Vec<Beam> {
        let mut res = vec![];
        match self.dir {
            Direction::Up | Direction::Down => {
                if let Some(pleft) = self.p.add(-1, 0) {
                    res.push(Beam {
                        p: pleft,
                        dir: Direction::Left,
                    });
                }

                if let Some(pright) = self.p.add(1, 0) {
                    res.push(Beam {
                        p: pright,
                        dir: Direction::Right,
                    });
                }
            }
            Direction::Left | Direction::Right => {
                if let Some(pup) = self.p.add(0, -1) {
                    res.push(Beam {
                        p: pup,
                        dir: Direction::Up,
                    });
                }

                if let Some(pdown) = self.p.add(0, 1) {
                    res.push(Beam {
                        p: pdown,
                        dir: Direction::Down,
                    });
                }
            }
        }
        res
    }

    fn deflect(&mut self, c: char) -> Option<()> {
        match (c, self.dir) {
            ('/', Direction::Up) | ('\\', Direction::Down) => {
                self.p.x += 1;
                self.dir = Direction::Right;
            }
            ('\\', Direction::Up) | ('/', Direction::Down) => {
                // Left
                self.p.x = self.p.x.checked_sub(1)?;
                self.dir = Direction::Left;
            }
            ('/', Direction::Left) | ('\\', Direction::Right) => {
                // Down
                self.p.y += 1;
                self.dir = Direction::Down;
            }
            ('\\', Direction::Left) | ('/', Direction::Right) => {
                // Up
                self.p.y = self.p.y.checked_sub(1)?;
                self.dir = Direction::Up;
            }
            _ => panic!("can't deflect {:?}", self),
        }
        Some(())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calc_energized(file: &String, start: Beam) -> usize {
    let mut cb = CharBox::new(file, &start);

    let mut next_beams = cb.next(start);
    while !next_beams.is_empty() {
        next_beams = next_beams.iter().map(|b| cb.next(*b)).flatten().collect();
    }
    cb.energized.into_iter().unique_by(|b| b.0.p).count()
}

fn part1() -> usize {
    let file = fs::read_to_string("src/23/day16/input.txt").unwrap();

    let start = Beam {
        p: Point::new(0, 0),
        dir: Direction::Right,
    };
    calc_energized(&file, start)
}

fn part2() -> usize {
    let file = fs::read_to_string("src/23/day16/input.txt").unwrap();

    let width = file.lines().nth(0).unwrap().len();
    let height = file.lines().count();
    let mut starts = Vec::<Beam>::new();

    for x in 0..width {
        starts.push(Beam {
            p: Point::new(x, 0),
            dir: Direction::Down,
        });

        starts.push(Beam {
            p: Point::new(x, height - 1),
            dir: Direction::Up,
        });
    }

    for y in 0..height {
        starts.push(Beam {
            p: Point::new(0, y),
            dir: Direction::Right,
        });

        starts.push(Beam {
            p: Point::new(width - 1, y),
            dir: Direction::Left,
        });
    }

    starts
        .iter()
        .map(|s| calc_energized(&file, *s))
        .max()
        .unwrap()
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::{Beam, CharBox, Direction, Point};

    #[test]
    fn beam_split() {
        let b = Beam {
            p: Point { x: 1, y: 1 },
            dir: Direction::Right,
        };

        let sb1 = Beam {
            p: Point { x: 1, y: 0 },
            dir: Direction::Up,
        };

        let sb2 = Beam {
            p: Point { x: 1, y: 2 },
            dir: Direction::Down,
        };
        assert_eq!(b.split(), vec![sb1, sb2]);
    }

    #[test]
    fn beam_split_2() {
        let b = Beam {
            p: Point { x: 0, y: 0 },
            dir: Direction::Right,
        };

        let sb1 = Beam {
            p: Point { x: 0, y: 1 },
            dir: Direction::Down,
        };

        assert_eq!(b.split(), vec![sb1]);

        let sb2 = Beam {
            p: Point { x: 1, y: 1 },
            dir: Direction::Right,
        };

        assert_eq!(sb1.split(), vec![sb2]);

        let sb3 = Beam {
            p: Point { x: 1, y: 2 },
            dir: Direction::Down,
        };

        let sb4 = Beam {
            p: Point { x: 1, y: 0 },
            dir: Direction::Up,
        };

        assert_eq!(sb2.split(), vec![sb4, sb3]);
    }

    #[test]
    fn beam_deflect() {
        let mut b = Beam {
            p: Point { x: 1, y: 1 },
            dir: Direction::Right,
        };

        let bd = Beam {
            p: Point { x: 1, y: 2 },
            dir: Direction::Down,
        };

        b.deflect('\\');
        assert_eq!(b, bd);
    }

    #[test]
    fn point_in() {
        let p = Point::new(0, 0);
        let s = Point::new(3, 3);
        assert_eq!(p.is_in(s), true);
    }

    #[test]
    fn point_add() {
        let p = Point::new(0, 0);
        let n = p.add(0, -1);
        assert_eq!(n, None);
        let n = p.add(0, 1);
        assert_eq!(n, Some(Point { x: 0, y: 1 }));
    }

    #[test]
    fn charbox_next() {
        let start = Beam {
            p: Point { x: 0, y: 0 },
            dir: Direction::Right,
        };
        let mut cb = CharBox::new("\\...\n../.\n-...\n|...", &start);

        let r1 = Beam {
            p: Point { x: 0, y: 1 },
            dir: Direction::Down,
        };
        let n1 = cb.next(start);
        assert_eq!(n1, vec![r1]);
        let n2 = cb.next(n1[0]);
        let r2 = Beam {
            p: Point { x: 0, y: 2 },
            dir: Direction::Down,
        };
        assert_eq!(n2, vec![r2]);
        let n3 = cb.next(n2[0]);
        let r3 = Beam {
            p: Point { x: 1, y: 2 },
            dir: Direction::Right,
        };
        assert_eq!(n3, vec![r3]);
    }

    //\.....
    //../...
    //-..\..
    //|../..
    //......
    //\../..
    #[test]
    fn char_box_get() {
        let start = Beam {
            p: Point { x: 0, y: 0 },
            dir: Direction::Right,
        };
        let cb = CharBox::new("\\...\n../.\n-...\n|...", &start);
        assert_eq!(cb.get(Point::new(0, 3)), Some('|'));
        assert_eq!(cb.get(Point::new(2, 1)), Some('/'));
    }

    //\.....
    //../...
    //-..\..
    //|../..
    //......
    //\../..
    #[test]
    fn char_box_energized() {
        let start = Beam {
            p: Point { x: 0, y: 0 },
            dir: Direction::Right,
        };

        let mut cb = CharBox::new("\\.....\n../...\n-..\\..\n|../..\n......\n\\../..", &start);
        let mut next_beams = cb.next(start);
        while !next_beams.is_empty() {
            next_beams = next_beams.iter().map(|b| cb.next(*b)).flatten().collect();
        }
        let points = cb.energized.into_iter().unique_by(|b| b.0.p).count();
        assert_eq!(points, 18);
    }
}
