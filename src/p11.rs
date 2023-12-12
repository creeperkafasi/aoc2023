use std::{borrow::BorrowMut, fs};

pub fn answer() -> (usize, usize) {
    let mut f = fs::read_to_string("inputs/11.txt").unwrap();
    let width = f.find("\n").unwrap() + 1;
    let height = f.split("\n").count();

    let mut di = 0;

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for (y, line) in f.split("\n").enumerate() {
        if line.chars().all(|c| c == '.') {
            empty_rows.push(y);
        }
    }

    // println!("{}", f);

    let f0 = f.clone();

    (0..(width - 1)).for_each(|x| {
        if f.lines().all(|line| line.chars().nth(x) == Some('.')) {
            empty_cols.push(x);
            // // dbg!(x + di);
            // f.clone()
            //     .match_indices("\n")
            //     .chain([(f.clone().char_indices().last().unwrap().0 + 1, "")])
            //     .for_each(|(i, _)| {
            //         // println!("{}\n", f.clone());
            //         f.insert(i + x + di - width + 1, '.');
            //         di += 1;
            //     });
        }
    });

    // dbg!(empty_cols, empty_rows);

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

    let info = (0..galaxies.len())
        .map(|i| {
            let g1 = galaxies.get(i).unwrap();
            (i..galaxies.len()).map(|j| {
                let g2 = galaxies.get(j).unwrap();

                let ex = empty_cols
                    .iter()
                    .filter(|x| (**x).clamp(g1.x.min(g2.x), g1.x.max(g2.x)) == **x)
                    .count();

                let ey = empty_rows
                    .iter()
                    .filter(|y| (**y).clamp(g1.y.min(g2.y), g1.y.max(g2.y)) == **y)
                    .count();

                (*g1, *g2, ex, ey)
            }).collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // let info = galaxies
    //     .iter()
    //     .combinations(2)
    //     .unique()
    //     .map(|g| {
    //         let g1 = g.first().unwrap();
    //         let g2 = g.last().unwrap();

    //         let ex = empty_cols
    //             .iter()
    //             .filter(|x| (**x).clamp(g1.x.min(g2.x), g1.x.max(g2.x)) == **x)
    //             .count();

    //         let ey = empty_rows
    //             .iter()
    //             .filter(|y| (**y).clamp(g1.y.min(g2.y), g1.y.max(g2.y)) == **y)
    //             .count();

    //         (*g1, *g2, ex, ey)
    //     })
    //     .collect::<Vec<_>>();

    (
        info.iter()
            .map(|(g1, g2, ex, ey)| {
                g2.x.abs_diff(g1.x) + ex * 1 + g2.y.abs_diff(g1.y) + ey * 1
            })
            .sum(),
        info.iter()
            .map(|(g1, g2, ex, ey)| {
                g2.x.abs_diff(g1.x) + ex * 999_999 + g2.y.abs_diff(g1.y) + ey * 999_999
            })
            .sum(),
    )
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}
