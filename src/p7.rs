// five of a kind
// four of a kind
// full house (3 same + 2 same)
// three of a kind
// two pair
// one pair
// high card (all different)

use std::{fs, vec};

#[derive(Debug, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    kind: HandKind,
    bet: usize,
    cards: String,
    counts: Vec<usize>,
}

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/7.txt").unwrap();
    let hands = input(f);
    // dbg!(second(hands));

    (first(hands), 0)
    // todo!()
}

fn first(hands: Vec<Hand>) -> usize {
    let mut hands = hands.clone();

    hands.sort_by(|a, b| {
        if (a.kind as usize) == (b.kind as usize) {
            return cards_value(a.cards.clone(), false).cmp(&cards_value(b.cards.clone(), false));
        } else {
            (a.kind as usize).cmp(&(b.kind as usize))
        }
    });

    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bet);

    res
}

// fn second(hands: Vec<Hand>)-> usize {
//     for hand in hands {
//         hand.

//     }
//     todo!()
// }

fn input(f: String) -> Vec<Hand> {
    f.split("\n")
        .map(|line| {
            let (cards, bet) = line.split_once(" ").unwrap();

            let counts = cards
                .chars()
                .map(|c| cards.chars().filter(|c1| *c1 == c).count())
                .collect::<Vec<usize>>();

            let mut sorted_counts = counts.clone();
            sorted_counts.sort();

            Hand {
                bet: bet.parse().unwrap(),
                cards: cards.to_string(),
                counts: counts,
                kind: match sorted_counts.get(..5).unwrap() {
                    [1, 1, 1, 1, 1] => HandKind::HighCard,
                    [1, 1, 1, 2, 2] => HandKind::OnePair,
                    [1, 2, 2, 2, 2] => HandKind::TwoPair,
                    [1, 1, 3, 3, 3] => HandKind::ThreeOfAKind,
                    [2, 2, 3, 3, 3] => HandKind::FullHouse,
                    [1, 4, 4, 4, 4] => HandKind::FourOfAKind,
                    [5, 5, 5, 5, 5] => HandKind::FiveOfAKind,
                    _ => panic!("Bad Deck"),
                },
            }
        })
        .collect()
}

fn cards_value(cards: String, part2: bool) -> usize {
    cards
                    .chars()
                    .enumerate()
                    .map(|(i, ch)| match ch {
                        '2' => 1,
                        '3' => 2,
                        '4' => 3,
                        '5' => 4,
                        '6' => 5,
                        '7' => 6,
                        '8' => 7,
                        '9' => 8,
                        'T' => 9,
                        'J' => if part2{0}else{10},
                        'Q' => 11,
                        'K' => 12,
                        'A' => 13,
                        _ => panic!()
                    } <<(4*(6-i))).reduce(|acc,e| acc+e).unwrap()
}

/*
   If we have a joker in the deck
   we can transform the card in this order:
   high card -> one pair
   one pair  -> three of a kind
   two pair  -> full house
   three oak -> four of a kind
   full house-> four of a kind
   four oak  -> five oak
*/
fn joker(hand: &mut Hand) {
    hand.kind = match hand.kind {
        HandKind::HighCard => HandKind::OnePair,
        HandKind::OnePair => HandKind::ThreeOfAKind,
        HandKind::TwoPair => HandKind::FullHouse,
        HandKind::ThreeOfAKind => HandKind::FourOfAKind,
        HandKind::FullHouse => HandKind::FourOfAKind,
        HandKind::FourOfAKind => HandKind::FiveOfAKind,
        HandKind::FiveOfAKind => HandKind::FiveOfAKind,
    }
}
