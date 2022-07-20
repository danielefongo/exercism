/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;

#[derive(Eq, PartialOrd, PartialEq)]
struct Hand<'a> {
    rank: Ranking,
    description: &'a str,
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl<'a> Hand<'a> {
    pub fn from_str(hand: &'a str) -> Self {
        let mut cards: Vec<Card> = hand.split(" ").map(Card::from_str).collect();
        cards.sort();
        cards.reverse();

        Hand {
            rank: Ranking::from(&cards),
            description: hand,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Ranking {
    HighCard(u32, Vec<u32>),
    OnePair(u32, Vec<u32>),
    TwoPair(u32, u32, u32),
    ThreeOfAKind(u32, Vec<u32>),
    Straight(u32),
    Flush(Vec<u32>),
    FullHouse(u32, u32),
    FourOfAKind(u32, Vec<u32>),
    StraightFlush(u32),
}

impl From<&Vec<Card>> for Ranking {
    fn from(cards: &Vec<Card>) -> Self {
        [
            Self::straight_flush,
            Self::four_of_a_kind,
            Self::full_house,
            Self::flush,
            Self::straight,
            Self::three_of_a_kind,
            Self::two_pair,
            Self::one_pair,
            Self::high_card,
        ]
        .iter()
        .find_map(|rule| rule(&cards))
        .unwrap()
    }
}

impl Ranking {
    fn straight_flush(cards: &Vec<Card>) -> Option<Ranking> {
        let val = straight_rule(cards)?;
        let _ = kind_rule(cards)?;
        Some(Ranking::StraightFlush(val))
    }

    fn four_of_a_kind(cards: &Vec<Card>) -> Option<Ranking> {
        let (val, rest) = value_rule(cards, 4)?;
        let rest_values = rest.iter().map(|card| card.value).collect();
        Some(Ranking::FourOfAKind(val, rest_values))
    }

    fn full_house(cards: &Vec<Card>) -> Option<Ranking> {
        let (val3, _) = value_rule(cards, 3)?;
        let (val2, _) = value_rule(cards, 2)?;
        Some(Ranking::FullHouse(val3, val2))
    }

    fn flush(cards: &Vec<Card>) -> Option<Ranking> {
        let _ = kind_rule(cards)?;
        let all_values = cards.iter().map(|card| card.value).collect();
        Some(Ranking::Flush(all_values))
    }

    fn straight(cards: &Vec<Card>) -> Option<Ranking> {
        let val = straight_rule(cards)?;
        Some(Ranking::Straight(val))
    }

    fn three_of_a_kind(cards: &Vec<Card>) -> Option<Ranking> {
        let (val, rest) = value_rule(cards, 3)?;
        let rest_values = rest.iter().map(|card| card.value).collect();
        Some(Ranking::ThreeOfAKind(val, rest_values))
    }

    fn two_pair(cards: &Vec<Card>) -> Option<Ranking> {
        let (val, remaining_cards) = value_rule(cards, 2)?;
        let (val2, last_card_vect) = value_rule(&remaining_cards, 2)?;
        let last_card = last_card_vect.iter().last()?;
        Some(Ranking::TwoPair(val, val2, last_card.value))
    }

    fn one_pair(cards: &Vec<Card>) -> Option<Ranking> {
        let (val, rest) = value_rule(cards, 2)?;
        let rest_values = rest.iter().map(|card| card.value).collect();
        Some(Ranking::OnePair(val, rest_values))
    }

    fn high_card(cards: &Vec<Card>) -> Option<Ranking> {
        let (val, rest) = value_rule(cards, 1)?;
        let rest_values = rest.iter().map(|card| card.value).collect();
        Some(Ranking::HighCard(val, rest_values))
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
enum Kind {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Card {
    pub value: u32,
    pub kind: Kind,
}

impl Card {
    fn from_str(card: &str) -> Self {
        let value = match &card[0..(card.len() - 1)] {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            other => other.to_string().parse::<u32>().unwrap(),
        };
        let kind = match card.chars().last().unwrap() {
            'S' => Kind::Spade,
            'H' => Kind::Heart,
            'C' => Kind::Club,
            _ => Kind::Diamond,
        };
        Card { value, kind }
    }
}

fn kind_rule(cards: &Vec<Card>) -> Option<u32> {
    let kind = &cards.iter().next()?.kind;
    if cards.iter().all(|card| card.kind == *kind) {
        cards.iter().map(|card| card.value).max()
    } else {
        None
    }
}

fn value_rule(cards: &Vec<Card>, qty: u32) -> Option<(u32, Vec<Card>)> {
    let value = (1..=14).rev().find(|value| {
        let cards: Vec<u32> = cards
            .iter()
            .filter(|card| card.value == *value)
            .map(|card| card.value)
            .collect();

        cards.len() == (qty as usize)
    })?;

    Some((
        value,
        cards
            .iter()
            .cloned()
            .filter(|card| card.value != value)
            .collect(),
    ))
}

fn straight_rule(cards: &Vec<Card>) -> Option<u32> {
    let mut cards_values_with_ace_as_one: Vec<u32> = cards
        .iter()
        .map(|card| match card.value {
            14 => 1,
            v => v,
        })
        .collect();
    cards_values_with_ace_as_one.sort_by(|a, b| b.cmp(a));

    let cards_values: Vec<u32> = cards.iter().map(|card| card.value).collect();

    if check_sequence(cards_values) {
        cards.iter().map(|it| it.value).max()
    } else if check_sequence(cards_values_with_ace_as_one) {
        cards
            .iter()
            .filter(|it| it.value != 14)
            .map(|it| it.value)
            .max()
    } else {
        None
    }
}

fn check_sequence(cards_values: Vec<u32>) -> bool {
    cards_values.windows(2).all(|window| {
        let first = window.first().unwrap();
        let second = window.last().unwrap();
        first - second == 1
    })
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut result: Vec<Hand> = hands.iter().map(|hand| Hand::from_str(hand)).collect();
    result.sort();
    result.reverse();

    let first_winner = result.first().unwrap();
    result
        .iter()
        .take_while(|hand| first_winner.cmp(&hand) == Ordering::Equal)
        .map(|hand| hand.description)
        .collect()
}
