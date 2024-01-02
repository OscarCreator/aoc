use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Expand {
    Column(usize),
    Row(usize)
}

struct Universe {
    expand_list: Vec<Expand>,
    galaxies: Vec<Galaxy>
}



impl Universe {
    fn new(content: String, expansion_size: usize) -> Self {
        let mut expand_list = Vec::new();

        for (y, line) in content.lines().enumerate() {
            if line.chars().all_equal() {
                expand_list.push(Expand::Row(y));
            }
        }

        for x in 0..content.lines().nth(0).unwrap().len() {
            for y in 0..content.lines().count() {
                let a = content.lines().nth(y).unwrap().chars().nth(x).unwrap();
                if a == '#' {
                    break; 
                }
                // if last item
                if y + 1 == content.lines().count() {
                    expand_list.push(Expand::Column(x));
                }
            }
        }

        let galaxies = Self::galaxies(content, &expand_list, expansion_size);
        Universe { expand_list, galaxies }
    }

    fn galaxies(content: String, expands: &Vec<Expand>, expansion_size: usize) -> Vec<Galaxy> {
        content.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| {
                if c == '#' {
                    // get quantity of expands which have less than x and y
                    let (columns, rows) = expands.iter().filter_map(|e| match e {
                        Expand::Column(e_x) => {
                            if e_x < &x {
                                Some((1, 0))
                            } else {
                                None
                            }
                        },
                        Expand::Row(e_y) => {
                            if e_y < &y {
                                Some((0, 1))
                            } else {
                                None
                            }
                        },
                    }).reduce(|acc, e| {
                        (acc.0 + e.0, acc.1 + e.1)
                    }).unwrap_or((0, 0));
                    Some(Galaxy {x: x + columns * expansion_size, y: y + rows * expansion_size})
                } else {
                    None
                }
            }).collect::<Vec<Galaxy>>()
        }).flatten().collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn cost(&self, other: &Galaxy) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn pairs<T: Copy>(v: Vec<T>) -> Vec<(T, T)> {
    v.iter().enumerate().map(|(i, e)| {
        v[i+1..v.len()].iter().map(|other| {
            (*e, *other)
        })
    }).flatten().collect()
}

fn part1() -> usize {
    let lines = fs::read_to_string("src/23/day11/input.txt").unwrap();

    let u = Universe::new(lines, 1);

    pairs(u.galaxies).iter().map(|(a, b)| a.cost(b)).sum()
}

fn part2() -> usize {
    let lines = fs::read_to_string("src/23/day11/input.txt").unwrap();

    let u = Universe::new(lines, 999999);

    pairs(u.galaxies).iter().map(|(a, b)| a.cost(b)).sum()
}


fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::Expand;
    use crate::Expand::*;

    use crate::Galaxy;
    use crate::Universe;
    use crate::pairs;

    #[test_case("..\n..", vec![Row(0), Row(1), Column(0), Column(1)])]
    #[test_case("#.\n.#", vec![])]
    #[test_case("..\n.#", vec![Row(0), Column(0)])]
    fn test_universe(lines: &str, expand_list: Vec<Expand>) {
        let universe = Universe::new(lines.to_owned(), 2);
        assert_eq!(universe.expand_list, expand_list)
    }

    #[test_case("..\n..", vec![])]
    #[test_case("#.\n.#", vec![Galaxy {x: 0, y: 0}, Galaxy {x: 1, y: 1}])]
    #[test_case("..\n.#", vec![Galaxy {x: 2, y: 2}])]
    fn test_galaxies(lines: &str, galaxies: Vec<Galaxy>) {
        let universe = Universe::new(lines.to_owned(), 2);
        assert_eq!(universe.galaxies, galaxies)
    }

    #[test_case(Galaxy {x: 0, y: 0}, Galaxy {x: 0, y: 1}, 1)]
    #[test_case(Galaxy {x: 3, y: 3}, Galaxy {x: 0, y: 1}, 5)]
    #[test_case(Galaxy {x: 0, y: 0}, Galaxy {x: 2, y: 3}, 5)]
    fn galaxy_cost(a: Galaxy, b: Galaxy, cost: usize) {
        assert_eq!(a.cost(&b), cost);
    }

    #[test]
    fn test_pair() {
        let a = vec![1, 2, 3, 4];
        assert_eq!(pairs(a), vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)])
    }
}
