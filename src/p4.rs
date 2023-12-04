use std::fs;

#[allow(dead_code)]
pub fn answer() {
    let games = input();
    println!("First answer:  {}", first(games.clone()));
    println!("Second answer: {}", second(games.clone()));
}

fn first(games: Vec<Card>) -> usize {
    let mut sum = 0;
    for Card { winners, have, .. } in games {
        let f = have.iter().filter(|n| winners.contains(n)).count() as u32;
        sum += 2i32.pow(f) / 2;
    }
    sum as usize
}

fn second(games: Vec<Card>) -> usize {
    let mut cardcounts: Vec<usize> = vec![1; games.len()];

    games
        .iter()
        .enumerate()
        .for_each(|(i, Card { winners, have })| {
            let c = have.iter().filter(|n| winners.contains(n)).count();
            for n in 0..c {
                cardcounts[i + 1 + n] += cardcounts[i];
            }
        });

    cardcounts.iter().sum::<usize>()
}

#[derive(Clone, Debug)]
struct Card {
    winners: Vec<usize>,
    have: Vec<usize>,
}

fn input() -> Vec<Card> {
    let f = fs::read_to_string("inputs/4.txt").unwrap();
    f.split("\n")
        .map(|line| line.split(":").last().unwrap())
        .map(|line| {
            let mut s = line.split(" | ");
            let winners = s.next();
            let have = s.next();
            Card {
                winners: winners
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<_>>(),
                have: have
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>()
}
