use std::fs;

const MAXCUBES: (usize, usize, usize) = (12, 13, 14);

pub fn answer() {
    let games = input();
    println!("{}", first(games.clone()));
    println!("{}", second(games.clone()));
}

fn first(games: Vec<Game>) -> usize {
    games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.0 <= MAXCUBES.0 && round.1 <= MAXCUBES.1 && round.2 <= MAXCUBES.2
            })
        })
        .map(|game| game.id)
        .reduce(|acc, id| acc + id)
        .unwrap()
}

fn second(games: Vec<Game>) -> usize {
    games
        .iter()
        .map(|game| {
            game.rounds.iter().max_by_key(|r| r.0).unwrap().0
                * game.rounds.iter().max_by_key(|r| r.1).unwrap().1
                * game.rounds.iter().max_by_key(|r| r.2).unwrap().2
        })
        .reduce(|acc, power| acc + power)
        .unwrap()
}

#[derive(Clone)]
struct Game {
    id: usize,
    rounds: Vec<(usize, usize, usize)>,
}

fn input() -> Vec<Game> {
    let f = fs::read_to_string("inputs/2.txt").unwrap();
    f.split("\n")
        .map(|line| {
            let mut spl = line.split(": ");
            let id = spl
                .next()
                .unwrap()
                .split("Game ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let rounds = spl
                .next()
                .unwrap()
                .split("; ")
                .map(|round| {
                    let mut cubes = (0, 0, 0);
                    round.split(", ").for_each(|round| {
                        if round.contains("red") {
                            cubes.0 = round.split(" red").next().unwrap().parse().unwrap();
                        }
                        if round.contains("green") {
                            cubes.1 = round.split(" green").next().unwrap().parse().unwrap();
                        }
                        if round.contains("blue") {
                            cubes.2 = round.split(" blue").next().unwrap().parse().unwrap();
                        }
                    });
                    return cubes;
                })
                .collect::<Vec<(usize, usize, usize)>>();
            return Game {
                id: id,
                rounds: rounds,
            };
        })
        .collect()
}
