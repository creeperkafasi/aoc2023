use std::fs;

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/11.txt").unwrap();
    let width = f.find("\n").unwrap() + 1;

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for (y, line) in f.split("\n").enumerate() {
        if line.chars().all(|c| c == '.') {
            empty_rows.push(y);
        }
    }

    (0..width).for_each(|x| {
        if f.lines().all(|line| line.chars().nth(x) == Some('.')) {
            empty_cols.push(x);
        }
    });

    let galaxies = f
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '#' => Some(Galaxy { x: x, y: y }),
                _ => None,
            })
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
                    .filter(|x| x.clamp(&&g1.x.min(g2.x), &&g1.x.max(g2.x)) == x)
                    .count();

                let ey = empty_rows
                    .iter()
                    .filter(|y| y.clamp(&&g1.y.min(g2.y), &&g1.y.max(g2.y)) == y)
                    .count();

                (*g1, *g2, ex, ey)
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    (
        info.iter()
            .map(|(g1, g2, ex, ey)| {
                return g2.x.abs_diff(g1.x) + ex + g2.y.abs_diff(g1.y) + ey;
            })
            .sum(),
        info.iter()
            .map(|(g1, g2, ex, ey)| {
                return g2.x.abs_diff(g1.x) + ex * 999_999 + g2.y.abs_diff(g1.y) + ey * 999_999;
            })
            .sum(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}
