use std::fs;

pub fn answer() {
    println!("First answer:  {}", first());
    println!("Second answer: {}", second());
}

fn first() -> usize {
    let games = input();

    let mut sum = 0;
    for Card {
        id: _,
        winners,
        have,
    } in games
    {
        let f = have
            .iter()
            .filter(|n| winners.contains(n))
            .fold(0.5_f32, |acc, _| acc * 2.0);
        sum += f.floor() as usize;
    }
    sum
}

fn second() -> usize {
    let games = input();
    let mut cardcounts: Vec<usize> = vec![1; games.len()];

    // for Card { id, winners, have } in &games {
    //     cardcounts.insert(*id, 1);
    // }

    let mut i = 0;

    loop {
        match games.get(i) {
            Some(Card { id, winners, have }) => {
                let c = have.iter().filter(|n| winners.contains(n)).count();
                for _ in 0..cardcounts[*id - 1] {
                    for n in *id..id + c {
                        cardcounts[n] = cardcounts[n] + 1;
                        // games.insert(i + 1, games.iter().find(|c| c.id == n + 1).unwrap().clone());
                    }
                }
                // dbg!(id);
            }
            None => break,
        };

        i += 1;
    }

    cardcounts.iter().sum::<usize>()
    // for (winner, have) in games {
    //     have.iter()
    //         .enumerate()
    //         // .map(|(x, y)| (x + 1, y))
    //         .filter(|n| winner.contains(n));
    // }

    // todo!()
}

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    have: Vec<usize>,
}

fn input() -> Vec<Card> {
    let f = fs::read_to_string("inputs/4.txt").unwrap();
    f.split("\n")
        .map(|line| line.split(":").last().unwrap())
        .enumerate()
        .map(|(i, line)| {
            let mut s = line.split(" | ");
            let winners = s.next();
            let have = s.next();
            Card {
                id: i + 1,
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
