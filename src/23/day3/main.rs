use std::{fs, ops::RangeBounds, num};

#[derive(Debug)]
struct Number {
    column: usize,
    row: usize,
    value: u32,
}

impl Number {
    fn len(&self) -> usize {
        (self.value.checked_ilog10().unwrap_or(0) + 1) as usize
    }
}

#[derive(Debug)]
struct Symbol {
    column: usize,
    row: usize,
}

fn solve1() -> u32 {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let file_string = fs::read_to_string("src/23/day3/input.txt").expect("error");

    let mut current_number = None;
    for (row, line) in file_string.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => match current_number {
                    None => current_number = Some(char.to_digit(10).unwrap()),
                    Some(number) => {
                        current_number = Some(number * 10 + char.to_digit(10).unwrap());
                        if column == 139 {
                            if let Some(number) = current_number {
                                dbg!(column, number.ilog10());
                                numbers.push(Number {
                                    column: column - number.ilog10() as usize,
                                    row: row + 1,
                                    value: number,
                                });
                                current_number = None;
                            }
                        }
                    }
                },
                '.' => {
                    if let Some(number) = current_number {
                        dbg!(column, number.ilog10());
                        numbers.push(Number {
                            column: column - number.ilog10() as usize,
                            row: row + 1,
                            value: number,
                        });
                        current_number = None;
                    }
                }
                _ => {
                    if let Some(number) = current_number {
                        numbers.push(Number {
                            column: column - number.ilog10() as usize,
                            row: row + 1,
                            value: number,
                        });
                        current_number = None;
                    }
                    symbols.push(Symbol { column: column + 1, row: row + 1 });
                }
            }
        }
    }
    dbg!(&numbers);

    return numbers.iter()
        .map(|number| {
            let x_range = number.row-1..=number.row+1;
            for symbol in &symbols {
                if x_range.contains(&symbol.row) {

                    if (number.column-1..=number.column + number.len()).contains(&symbol.column) {
                        dbg!("x & y", &number, symbol, &x_range);
                        return number.value;
                    }
                }
            }
            0
        }).sum::<u32>();
}

fn solve2() -> u32 {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let file_string = fs::read_to_string("src/23/day3/input.txt").expect("error");

    let mut current_number = None;
    for (row, line) in file_string.lines().enumerate() {
        for (column, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => match current_number {
                    None => current_number = Some(char.to_digit(10).unwrap()),
                    Some(number) => {
                        current_number = Some(number * 10 + char.to_digit(10).unwrap());
                        if column == 139 {
                            if let Some(number) = current_number {
                                dbg!(column, number.ilog10());
                                numbers.push(Number {
                                    column: column - number.ilog10() as usize,
                                    row: row + 1,
                                    value: number,
                                });
                                current_number = None;
                            }
                        }
                    }
                },
                '.' => {
                    if let Some(number) = current_number {
                        dbg!(column, number.ilog10());
                        numbers.push(Number {
                            column: column - number.ilog10() as usize,
                            row: row + 1,
                            value: number,
                        });
                        current_number = None;
                    }
                }
                _ => {
                    if let Some(number) = current_number {
                        numbers.push(Number {
                            column: column - number.ilog10() as usize,
                            row: row + 1,
                            value: number,
                        });
                        current_number = None;
                    }
                    symbols.push(Symbol { column: column + 1, row: row + 1 });
                }
            }
        }
    }
    dbg!(&numbers);

    return symbols.iter()
        .map(|symbol| {
            let x_range = symbol.row-1..=symbol.row+1;
            let mut count = Vec::new();
            for number in &numbers {
                if x_range.contains(&number.row) {

                    if (number.column-1..=number.column + number.len()).contains(&symbol.column) {
                        dbg!("x & y", &number, symbol, &x_range);
                        count.push(number.value);
                    }
                }
            }
            if count.len() == 2 {
                return count.into_iter().reduce(|a, b| a * b).unwrap();
            }
            0
        }).sum::<u32>();
}

fn main() {
    println!("p1: {}", solve1());
    println!("p2: {}", solve2());
}
