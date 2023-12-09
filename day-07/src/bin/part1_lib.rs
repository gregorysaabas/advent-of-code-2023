use std::collections::HashMap;
use scan_fmt::scan_fmt;

pub fn part1(input: &str) -> u32 {
    let mut hand_bets = parse_input(input);
    hand_bets.sort_by_key(|x| x.hand.get_score());
    hand_bets.iter().enumerate().map(|(i, hb)| ((i + 1) as u32) * hb.bet)
        .sum()
}


fn parse_input(input: &str) -> Vec<HandBet> {
    input
        .lines()
        .map(|line| {
            let (hand_str, bet) = scan_fmt!(line, "{} {}", String, u32).ok().unwrap();
            let cards = hand_str.chars().map(|c| match c {
                '2' => CardValue::Two,
                '3' => CardValue::Three,
                '4' => CardValue::Four,
                '5' => CardValue::Five,
                '6' => CardValue::Six,
                '7' => CardValue::Seven,
                '8' => CardValue::Eight,
                '9' => CardValue::Nine,
                'T' => CardValue::Ten,
                'J' => CardValue::Jack,
                'Q' => CardValue::Queen,
                'K' => CardValue::King,
                'A' => CardValue::Ace,
                _ => panic!("Invalid card character: {}", c),
            }, ).collect();
            let card_count = Hand::card_count(&cards);
            let hand = Hand {
                cards,
                count: card_count,
            };

            HandBet { hand, bet }
        })
        .collect()
}


impl Hand {
    fn card_count(cards: &Vec<CardValue>) -> HashMap<CardValue, usize> {
        cards
            .iter()
            .fold(HashMap::new(), |mut count_map, &card| {
                *count_map.entry(card).or_insert(0) += 1;
                count_map
            })
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.count.len() == 1
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.count.len() == 2
            && self.count.iter().any(|(_, c)| *c == 4)
    }

    fn is_full_house(&self) -> bool {
        self.count.len() == 2
            && self.count.iter().any(|(_, c)| *c == 3)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.count.len() == 3
            && self.count.iter().any(|(_, c)| *c == 3)
    }

    fn is_two_pair(&self) -> bool {
        self.count.len() == 3
            && self.count.iter().any(|(_, c)| *c == 2)
    }

    fn is_one_pair(&self) -> bool {
        self.count.len() == 4
    }

    fn get_rank(&self) -> u32 {
        match self {
            h if h.is_five_of_a_kind() => 6,
            h if h.is_four_of_a_kind() => 5,
            h if h.is_full_house() => 4,
            h if h.is_three_of_a_kind() => 3,
            h if h.is_two_pair() => 2,
            h if h.is_one_pair() => 1,
            _ => 0
        }
    }

    fn get_tiebreaker(&self) -> u32 {
        self.cards.iter().enumerate()
            .map(|(i, c): (usize, &CardValue)| -> u32 { (*c as u32) * u32::pow(16, (4 - i) as u32) })
            .sum()
    }

    fn get_score(&self) -> u32 {
        self.get_rank() * u32::pow(16, 5) + self.get_tiebreaker()
    }
}

#[derive(Debug)]
struct HandBet {
    hand: Hand,
    bet: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum CardValue {
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
    Eight = 0x08,
    Nine = 0x09,
    Ten = 0x0A,
    Jack = 0x0B,
    Queen = 0x0C,
    King = 0x0D,
    Ace = 0x0E,
}

impl From<CardValue> for u32 {
    fn from(card_value: CardValue) -> u32 {
        card_value as u32
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<CardValue>,
    count: HashMap<CardValue, usize>,
}