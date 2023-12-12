use std::{collections::HashMap, fs};

#[derive(Debug)]
enum TileKind {
    Start,      // S
    Horizontal, // |
    Vertical,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Empty,      // .
}

impl TileKind {
    fn step(&self, dx: usize, dy: usize) -> Option<(usize, usize)> {
        match *self {
            TileKind::Horizontal => match (dx, dy) {
                (0, 1) => Some((0, 1)),
                (2, 1) => Some((2, 1)),
                _ => None,
            },
            TileKind::Vertical => match (dx, dy) {
                (1, 0) => Some((1, 0)),
                (1, 2) => Some((1, 2)),
                _ => None,
            },
            TileKind::NorthEast => match (dx, dy) {
                (0, 1) => Some((1, 0)),
                (1, 2) => Some((2, 1)),
                _ => None,
            },
            TileKind::NorthWest => match (dx, dy) {
                (2, 1) => Some((1, 0)),
                (1, 2) => Some((0, 1)),
                _ => None,
            },
            TileKind::SouthWest => match (dx, dy) {
                (2, 1) => Some((1, 2)),
                (1, 0) => Some((0, 1)),
                _ => None,
            },
            TileKind::SouthEast => match (dx, dy) {
                (0, 1) => Some((1, 2)),
                (1, 0) => Some((2, 1)),
                _ => None,
            },
            TileKind::Start => Some((1, 1)),
            _ => panic!("Don't call step with nondirectional tiles"),
        }
    }
}

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/10.txt").unwrap();
    let mut start_pos = (0, 0);
    let map: HashMap<(usize, usize), TileKind> = f
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x + 1, y + 1),
                        match c {
                            'S' => {
                                start_pos = (x + 1, y + 1);
                                TileKind::Start
                            }
                            '|' => TileKind::Vertical,
                            '-' => TileKind::Horizontal,
                            'L' => TileKind::NorthEast,
                            'J' => TileKind::NorthWest,
                            '7' => TileKind::SouthWest,
                            'F' => TileKind::SouthEast,
                            '.' => TileKind::Empty,
                            _ => TileKind::Empty,
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // dbg!(start_pos);

    let mut pos;
    let mut loop_tiles: Vec<((usize, usize), &TileKind)> = vec![];
    'start: for (sdx, sdy) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
        pos = start_pos;
        let (mut dx, mut dy) = (sdx, sdy);
        loop_tiles = vec![];
        // println!("--------\nWalking\n--------");
        'walk: loop {
            match map.get(&(pos.0 + dx - 1, pos.1 + dy - 1)) {
                Some(TileKind::Start) => {
                    // println!("Back to Start");
                    break 'start;
                }
                Some(TileKind::Empty) => {
                    break 'walk;
                }
                None => {
                    break 'walk;
                }
                Some(tile) => {
                    // println!("-------------------");
                    // dbg!(pos, dx, dy, tile, tile.step(dx, dy));
                    if let Some((ndx, ndy)) = tile.step(dx, dy) {
                        pos = (pos.0 + dx - 1, pos.1 + dy - 1);
                        (dx, dy) = (ndx, ndy);
                        loop_tiles.push((pos, tile));
                    } else {
                        break 'walk;
                    }
                }
            }
        }
    }

    (loop_tiles.len() / 2 + 1, 0)
}
