use std::collections::HashMap;
use std::fs;

struct CharBox {
    v: Vec<Vec<u32>>,
    size: Point,
}

impl CharBox {
    // get cost of point
    fn cost(&self, point: &Point) -> Option<u32> {
        if point.is_in(self.size) {
            return Some(self.v[point.y][point.x]);
        }
        None
    }

    fn new(s: &str) -> Self {
        Self {
            v: s.lines()
                .into_iter()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
            size: Point::new(s.lines().nth(0).unwrap().chars().count(), s.lines().count()),
        }
    }

    // get possible moves from `node` with the max move distance = `range` and minimum block before
    // turn = `turn_range`
    fn possible_moves(&self, node: &Move, range: usize, turn_range: usize) -> Vec<Move> {
        use Direction::*;

        let mut res = vec![];
        match node.dir {
            UP => {
                if node.count < range {
                    if let Some(p) = node.p.add(0, -1) {
                        res.push(Move::new(p, UP, node.count + 1));
                    }
                }
                if node.count >= turn_range {
                    if let Some(p) = node.p.add(-1, 0) {
                        res.push(Move::new(p, LEFT, 1));
                    }
                    if let Some(p) = node.p.add(1, 0) {
                        res.push(Move::new(p, RIGHT, 1));
                    }
                }
            }
            DOWN => {
                if node.count < range {
                    if let Some(p) = node.p.add(0, 1) {
                        res.push(Move::new(p, DOWN, node.count + 1));
                    }
                }
                if node.count >= turn_range {
                    if let Some(p) = node.p.add(-1, 0) {
                        res.push(Move::new(p, LEFT, 1));
                    }
                    if let Some(p) = node.p.add(1, 0) {
                        res.push(Move::new(p, RIGHT, 1));
                    }
                }
            }
            RIGHT => {
                if node.count < range {
                    if let Some(p) = node.p.add(1, 0) {
                        res.push(Move::new(p, RIGHT, node.count + 1));
                    }
                }
                if node.count >= turn_range {
                    if let Some(p) = node.p.add(0, -1) {
                        res.push(Move::new(p, UP, 1));
                    }
                    if let Some(p) = node.p.add(0, 1) {
                        res.push(Move::new(p, DOWN, 1));
                    }
                }
            }
            LEFT => {
                if node.count < range {
                    if let Some(p) = node.p.add(-1, 0) {
                        res.push(Move::new(p, LEFT, node.count + 1));
                    }
                }
                if node.count >= turn_range {
                    if let Some(p) = node.p.add(0, -1) {
                        res.push(Move::new(p, UP, 1));
                    }
                    if let Some(p) = node.p.add(0, 1) {
                        res.push(Move::new(p, DOWN, 1));
                    }
                }
            }
        };

        res.into_iter().filter(|m| m.p.is_in(self.size)).collect()
    }

    /// store vertex as `Move`
    /// only if the `Move`s are identical we can compare cost of vertex,
    /// if they are not identical then should all be considered
    fn solve(&self, range: usize, turn_range: usize) -> u32 {
        use Direction::*;

        let mut current_moves = vec![
            Move::new(Point::new(1, 0), RIGHT, 2),
            Move::new(Point::new(0, 1), DOWN, 2),
        ];
        let mut visited = HashMap::<Move, u32>::new();
        visited.insert(current_moves[0], self.cost(&current_moves[0].p).unwrap());
        visited.insert(current_moves[1], self.cost(&current_moves[1].p).unwrap());

        let mut current_best = 0;
        while !current_moves.is_empty() {
            current_best = *visited
                .iter()
                .filter_map(|(m, c)| {
                    if m.p.x + 1 == self.size.x && m.p.y + 1 == self.size.y {
                        return Some(c);
                    }
                    None
                })
                .min()
                .unwrap_or(&u32::MAX);

            current_moves = current_moves
                .iter()
                .map(|m| {
                    let current_cost = *visited.get(m).unwrap();
                    let neighbours = self.possible_moves(&m, range, turn_range);
                    neighbours
                        .into_iter()
                        .filter(|neighbour| {
                            let neighbour_cost = self.cost(&neighbour.p).unwrap();
                            let move_cost = current_cost + neighbour_cost;
                            if move_cost > current_best {
                                // scap as cost is to high already
                                return false;
                            }
                            if let Some(&other_cost) = visited.get(neighbour) {
                                // if better score
                                if move_cost < other_cost {
                                    visited.insert(*neighbour, move_cost);
                                    return true;
                                } else {
                                    // scrap move
                                    return false;
                                }
                            } else {
                                // new move, add it
                                visited.insert(*neighbour, move_cost);
                                return true;
                            }
                        })
                        .collect::<Vec<Move>>()
                })
                .flatten()
                .collect::<Vec<Move>>();
            if current_best != u32::MAX {
                println!("current best: {}", current_best);
            }
        }

        current_best
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Move {
    p: Point,
    dir: Direction,
    count: usize,
}

impl Move {
    fn new(p: Point, dir: Direction, count: usize) -> Self {
        Move { p, dir, count }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn is_in(&self, other: Self) -> bool {
        self.x < other.x && self.y < other.y
    }

    fn add(&self, x: i32, y: i32) -> Option<Self> {
        if let (Ok(nx), Ok(ny)) = (
            (self.x as i32 + x).try_into(),
            (self.y as i32 + y).try_into(),
        ) {
            return Some(Self { x: nx, y: ny });
        }
        None
    }
}

fn part1() -> u32 {
    // max three blocks in a single direction
    // then turn left or right
    let string = fs::read_to_string("src/23/day17/input.txt").unwrap();
    let crucible = CharBox::new(&string);

    crucible.solve(3, 1)
}

fn part2() -> u32 {
    let string = fs::read_to_string("src/23/day17/input.txt").unwrap();
    let crucible = CharBox::new(&string);

    crucible.solve(10, 4)
}

fn main() {
    println!("p1: {}", part1());
    println!("p2: {}", part2());
}

#[cfg(test)]
mod test {
    use crate::{CharBox, Point};
    use test_case::test_case;

    #[test_case("111\n222\n333", Point {x: 3, y: 3})]
    #[test_case("111\n222\n333\n444\n555\n666", Point {x: 3, y: 6})]
    fn size(area: &str, size: Point) {
        let string = area.to_owned();
        let crucible = CharBox::new(&string);
        assert_eq!(crucible.size, size);
    }

    #[test_case("111\n222\n333\n444\n555\n666", Point {x: 2, y: 5}, 6)]
    #[test_case("111\n222\n333\n444\n555\n666", Point {x: 2, y: 2}, 3)]
    #[test_case("111\n222\n333\n444\n555\n666", Point {x: 2, y: 1}, 2)]
    fn cost(area: &str, size: Point, cost: u32) {
        let string = area.to_owned();
        let crucible = CharBox::new(&string);
        assert_eq!(crucible.cost(&size).unwrap(), cost);
    }

    #[test_case("111\n222\n333\n444", 11, 17)]
    #[test_case("11111\n22222\n33333\n44444\n55555", 20, 18)]
    #[test_case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        102,
        94
    )]
    fn test_bigger(area: &str, res: u32, res2: u32) {
        let string = area.to_owned();
        let crucible = CharBox::new(&string);
        assert_eq!(crucible.solve(3, 1), res);
        assert_eq!(crucible.solve(10, 4), res2);
    }
}
