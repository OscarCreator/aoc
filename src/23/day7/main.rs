use std::{fs, cmp::Ordering};

use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd)]
enum CamelCardType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialOrd, PartialEq)]
struct CamelCard<'a> {
    hand: &'a str,
    bid: u32,
}

impl<'a> CamelCard<'a> {
    fn camel_type(&self) -> CamelCardType {
        // TODO
        let char_counts = self.hand.chars().counts();
        let max: Vec<(&char, &usize)> = char_counts.iter().max_set_by(|a, b| a.1.cmp(b.1));
        let card = match max.get(0).unwrap().1 {
            5 => Some(CamelCardType::FiveOfAKind),
            4 => Some(CamelCardType::FourOfAKind),
            3 => {
                if *char_counts.iter().min_set_by(|a, b| a.1.cmp(b.1)).get(0).unwrap().1 == 2 {
                    Some(CamelCardType::FullHouse)
                } else {
                    Some(CamelCardType::ThreeOfAKind)
                }
            },
            _ => None
        };

        if card.is_none() {
            return match max.len() {
                2 => CamelCardType::TwoPair,
                1 => CamelCardType::OnePair,
                _ => CamelCardType::HighCard,
            }
        }
        card.unwrap()
    }
}

impl<'a> Ord for CamelCard<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let other_type = other.camel_type();
        let self_type = self.camel_type();
        if self_type < other_type {
            Ordering::Greater
        } else if other_type == self_type {
            let stregth = |c: char| -> u32 {
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    '2'..='9' => c.to_digit(10).unwrap(),
                    _ => panic!("error ordering"),
                }
            };
            for (self_char, other_char) in self.hand.chars().into_iter().zip(other.hand.chars()) {
                if self_char == other_char {
                    continue;
                }
                if stregth(self_char) > stregth(other_char) {
                    return Ordering::Greater;
                }
                return Ordering::Less;
            }
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

fn part1() -> u32 {
    let file = fs::read_to_string("src/23/day7/input.txt").unwrap();
    let lines = file.lines();
    
    let mut cards = Vec::new();
    for line in lines {
        let (hand, bid) = line.split_once(' ').unwrap();
        cards.push(CamelCard {
            hand,
            bid: bid.parse().unwrap(),
        });
    }
    cards.sort_by(|c1, c2| c1.cmp(c2));
    cards.iter().enumerate().map(|(i, v)| v.bid * (i as u32 + 1)).sum()
}

fn main() {
    println!("p1: {}", part1());
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;
    use crate::{CamelCardType, CamelCard};


    #[test_case("AAAAA", CamelCardType::FiveOfAKind)]
    #[test_case("AAAAB", CamelCardType::FourOfAKind)]
    #[test_case("AAA1B", CamelCardType::ThreeOfAKind)]
    #[test_case("AAABB", CamelCardType::FullHouse)]
    #[test_case("AA1BB", CamelCardType::TwoPair)]
    #[test_case("A132A", CamelCardType::OnePair)]
    #[test_case("B132A", CamelCardType::HighCard)]
    fn test_camel_type(hand: &str, camel_type: CamelCardType) {
        let card = CamelCard { hand, bid: 1, };
        assert_eq!(card.camel_type(), camel_type);
    }

    #[test_case("AAAAA", "AAAAB", Ordering::Greater)]
    #[test_case("AAAAA", "KKKKK", Ordering::Greater)]
    #[test_case("AAKKK", "AAQQQ", Ordering::Greater)]
    #[test_case("KAAKK", "AAAKK", Ordering::Less)]
    fn test_card_cmp(hand1: &str, hand2: &str, ordering: Ordering) {
        let card1 = CamelCard { hand: hand1, bid: 1 };
        let card2 = CamelCard { hand: hand2, bid: 1 };
        assert_eq!(card1.cmp(&card2), ordering)
    }
}
