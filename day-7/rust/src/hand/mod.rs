use std::cmp::Ordering;
use std::collections::{BTreeMap};
use std::str::FromStr;
use crate::hand::Card::{*};
use crate::hand::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Eq, Debug, Copy, Clone)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: [Card;5],
    pub bid: u32
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut parts = s.trim().split(" ");
        let hand_str = parts.next().expect("Expected two parts").trim();
        let bid_str = parts.next().expect("Expected two parts").trim();

        let mut cards_arr: [Card;5] = [Two;5];
        hand_str.chars().enumerate().for_each( |(i, c)| {
            cards_arr[i] = match c {
                'A' => Ace,
                'K' => King,
                'Q' => Queen,
                'J' => Jack,
                'T' => Ten,
                '9' => Nine,
                '8' => Eight,
                '7' => Seven,
                '6' => Six,
                '5' => Five,
                '4' => Four,
                '3' => Three,
                '2' => Two,
                _ => Two
            }
        });

        return Ok(Hand{
            hand_type: determine_best_hand(&cards_arr),
            cards: cards_arr,
            bid: u32::from_str(bid_str).expect("Expected bid to be valid")
        })
    }
}

fn determine_best_hand(cards: &[Card;5]) -> HandType {
    let mut card_counts = BTreeMap::new();

    cards.iter().for_each( | c: &Card | {
        let orig = card_counts.get(&c);
        card_counts.insert(c, 1 + orig.unwrap_or_else(|| {&0u8}));
    });

    let mut counts: Vec<&u8> = card_counts.values().collect();
    counts.sort();
    counts.reverse();

    match counts.as_slice() {
        [5] => FiveOfAKind,
        [4, 1] => FourOfAKind,
        [3, 2] => FullHouse,
        [3, 1, 1] => ThreeOfAKind,
        [2, 2, 1] => TwoPair,
        [2, 1, 1, 1] => OnePair,
        _ => HighCard
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type.eq(&other.hand_type) &&
            self.cards.eq(&other.cards)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> String {
        String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483")
    }

    fn get_expected_hands() -> Vec<Hand> {
        vec!{
            Hand {
                hand_type: OnePair,
                cards: [Three, Two, Ten, Three, King],
                bid: 765
            },
            Hand {
                hand_type: ThreeOfAKind,
                cards: [Ten, Five, Five, Jack, Five],
                bid: 684
            },
            Hand {
                hand_type: TwoPair,
                cards: [King, King, Six, Seven, Seven],
                bid: 28
            },
            Hand {
                hand_type: TwoPair,
                cards: [King, Ten, Jack, Jack, Ten],
                bid: 220
            },
            Hand {
                hand_type: ThreeOfAKind,
                cards: [Queen, Queen, Queen, Jack, Ace],
                bid: 483
            },
        }
    }

    #[test]
    fn test_hand_construction() {
        let example_input = get_example_input();
        let hand_strs: Vec<&str> = example_input.split("\n").collect();

        let mut hands: Vec<Hand> = Vec::new();
        hand_strs.iter().for_each( | s: &&str | {
            hands.push(Hand::from_str(s).expect("Unable to read input"));
        });

        let expected_hands = get_expected_hands();

        for hand in hands {
            assert!(expected_hands.contains(&hand));
        }
    }

    #[test]
    fn test_hand_equality() {
        let hand1 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 1};
        let hand2 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 2};
        let hand3 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Nine, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 4};
        let hand4 = Hand{ hand_type: HandType::FourOfAKind, cards: [Card::Nine, Card::Five, Card::Five, Card::Five, Card::Five], bid: 5};

        assert_eq!(hand1, hand2);
        assert_eq!(hand2, hand1);
        assert_ne!(hand1, hand3);
        assert_ne!(hand1, hand4);
        assert_ne!(hand2, hand3);
        assert_ne!(hand2, hand4);
        assert_ne!(hand3, hand4);
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 1};
        let hand2 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 2};
        let hand3 = Hand{ hand_type: HandType::ThreeOfAKind, cards: [Card::Nine, Card::Five, Card::Five, Card::Jack, Card::Five], bid: 4};
        let hand4 = Hand{ hand_type: HandType::FourOfAKind, cards: [Card::Nine, Card::Five, Card::Five, Card::Five, Card::Five], bid: 5};

        assert_eq!(Ordering::Equal, hand1.cmp(&hand2));
        assert_eq!(Ordering::Less, hand1.cmp(&hand3));
        assert_eq!(Ordering::Greater, hand1.cmp(&hand4));
    }
}