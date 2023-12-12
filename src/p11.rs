use itertools::Itertools;
use std::{borrow::BorrowMut, fs};

pub fn answer() -> (usize, usize) {
    let mut f = fs::read_to_string("inputs/11.txt").unwrap();
    let width = f.find("\n").unwrap() + 1;
    let height = f.split("\n").count();

    let mut di = 0;

    for (y, line) in f.clone().split("\n").enumerate() {
        if line.chars().all(|c| c == '.') {
            f.insert_str((y + di) * width, format!("{}\n", line).as_str());
            di += 1;
        }
    }

    // println!("{}", f);

    let f0 = f.clone();

    (0..(width - 1)).for_each(|x| {
        di = 0;
        if f0.lines().all(|line| line.chars().nth(x + di) == Some('.')) {
            // dbg!(x + di);
            f.clone()
                .match_indices("\n")
                .chain([(f.clone().char_indices().last().unwrap().0 + 1, "")])
                .for_each(|(i, _)| {
                    // println!("{}\n", f.clone());
                    f.insert(i + x + di - width + 1, '.');
                    di += 1;
                });
        }
    });

    // println!("{}", f);10276166

    let galaxies = f
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(Galaxy { x: x, y: y }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    (
        galaxies
            .iter()
            .combinations(2)
            .unique()
            .map(|g| {
                let g1 = g.first().unwrap();
                let g2 = g.last().unwrap();

                g2.x.abs_diff(g1.x) + g2.y.abs_diff(g1.y)
            })
            .sum(),
        0,
    )
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy {
    x: usize,
    y: usize,
}
