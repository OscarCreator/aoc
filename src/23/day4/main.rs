use std::fs;

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    hand_numbers: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
struct ScratchCard {
    number: u32,
}

impl Card {
    fn points(&self) -> u32 {
        match self.matches() {
            0 => return 0,
            n => {
                let mut total = 1;
                for _ in 1..n {
                    total = total * 2;
                }
                return total;
            }
        }
    }

    fn matches(&self) -> u32 {
        self.hand_numbers.iter().filter(|i| self.winning_numbers.contains(i)).count() as u32
    }

    fn get_cards(&self) -> Vec<ScratchCard> {
        let mut cards: Vec<ScratchCard> = Vec::new();

        let index = self.number;
        for i in 1..=self.matches() {
            cards.push(ScratchCard { number: i + index })
        }
        cards
    }
}


fn part1() -> u32 {
    return fs::read_to_string("src/23/day4/input.txt").expect("error")
        .lines()
        .map(|l| {
            let (win, hand_nums) = l.split_once('|').unwrap();
            let (card, win_nums) = win.split_once(':').unwrap();
            let (_, card_number) = card.split_once(' ').unwrap();

            Card {
                number: card_number.trim().parse().unwrap(),
                winning_numbers: win_nums.split_whitespace().map(|s| s.parse().unwrap()).collect(),
                hand_numbers: hand_nums.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            }
        }).map(|c| c.points()).sum();
}

fn part2() -> u32 {
    let cards: Vec<Card> = fs::read_to_string("src/23/day4/input.txt").expect("error")
        .lines()
        .map(|l| {
            let (win, hand_nums) = l.split_once('|').unwrap();
            let (card, win_nums) = win.split_once(':').unwrap();
            let (_, card_number) = card.split_once(' ').unwrap();

            Card {
                number: card_number.trim().parse().unwrap(),
                winning_numbers: win_nums.split_whitespace().map(|s| s.parse().unwrap()).collect(),
                hand_numbers: hand_nums.split_whitespace().map(|s| s.parse().unwrap()).collect(),
            }
        }).collect();

    let mut scratch_cards: Vec<ScratchCard> = cards.iter().map(|c| ScratchCard { number: c.number }).collect();
    
    cards.iter().for_each(|c| {
        let num_scratch_cards = scratch_cards.iter().filter(|s| s.number == c.number).count();
        if num_scratch_cards == 0 {
            return;
        }

        let win_cards = c.get_cards();
        for _ in 0..num_scratch_cards {
            scratch_cards.extend(&win_cards);
        }
    });
    scratch_cards.len() as u32
}


fn main() {
    println!("p1: {}", part1());
    println!("p1: {}", part2());
}
