use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Paths {
    left: u16,
    right: u16,
}
impl Paths {
    fn walk(self, d: Direction) -> u16 {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/8.txt").unwrap();

    let (moves_string, nodes_string) = f.split_once("\n\n").unwrap();

    let mut moves: Vec<Direction> = moves_string
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid move"),
        })
        .collect();

    let mut nodes: HashMap<u16, Paths> = HashMap::new();
    nodes_string.split("\n").for_each(|line| {
        let (node_id, paths_str) = line.split_once(" = ").unwrap();
        let paths = paths_str.get(1..9).unwrap().split_once(", ").unwrap();
        nodes.insert(
            node_id
                .bytes()
                .enumerate()
                .map(|(i, b)| ((('Z' as u8 - b) as u16) << ((2 - i) * 5)))
                .sum(),
            Paths {
                left: paths
                    .0
                    .bytes()
                    .enumerate()
                    .map(|(i, b)| ((('Z' as u8 - b) as u16) << ((2 - i) * 5)))
                    .sum(),
                right: paths
                    .1
                    .bytes()
                    .enumerate()
                    .map(|(i, b)| ((('Z' as u8 - b) as u16) << ((2 - i) * 5)))
                    .sum(),
            },
        );
    });

    (
        first(moves.clone(), nodes.clone()),
        second(moves.clone(), nodes.clone()),
    )
}

fn first(moves: Vec<Direction>, nodes: HashMap<u16, Paths>) -> usize {
    // return 0;
    let mut location = "AAA"
        .bytes()
        .enumerate()
        .map(|(i, b)| ((('Z' as u8 - b) as u16) << (i * 5)))
        .sum();

    let mut i = 0;

    while location != 0 {
        location = nodes
            .get(&location)
            .unwrap()
            .clone()
            .walk(*moves.get(i % moves.len()).unwrap());

        i += 1;
    }
    i
}

fn second(moves: Vec<Direction>, nodes: HashMap<u16, Paths>) -> usize {
    let mut locations: Vec<u16> = nodes
        .iter()
        .filter(|node| node.0 & (31) == 25)
        .map(|node| *node.0)
        .collect();
    let mut i = 0;
    dbg!(locations.clone());

    while locations.iter().any(|loc| loc & 31 != 0) {
        let mv = *moves.get(i % moves.len()).unwrap();
        for location in locations.iter_mut() {
            *location = match mv {
                Direction::Left => nodes.get(&location).unwrap().left,
                Direction::Right => nodes.get(&location).unwrap().right,
            }
        }
        if i % 10000000 == 0 {
            println!("{}", i);
        }
        i += 1
    }

    i
}
