use std::{fs, ops::Range, vec};

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/5.txt").unwrap();

    (first(f.to_string()), 0)
}

fn first(f: String) -> usize {
    let mut seeds: Vec<usize> = vec![];

    for data in f.split("\n\n") {
        if data.starts_with("seeds:") {
            seeds = data
                .split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
        } else {
            let mut nseeds = seeds.clone();
            data.split("\n").skip(1).for_each(|line| {
                // if line.ends_with(":") {
                //     return;
                // }
                let mut s = line.split_whitespace();
                let dst: usize = s.next().unwrap().parse().unwrap();
                let src: usize = s.next().unwrap().parse().unwrap();
                let rng: usize = s.next().unwrap().parse().unwrap();

                // println!("{}..{} -> {}..{}", src, src + rng, dst, dst + rng);
                seeds
                    .iter_mut()
                    .enumerate()
                    .filter(|(i, seed)| (src..src + rng).contains(seed))
                    .for_each(|(i, seed)| {
                        // println!("{seed}->{}", *seed + dst - src);
                        // dbg!(seed.,src);
                        nseeds[i] += dst;
                        nseeds[i] -= src;
                        // println!("{seed}")
                    });
            });
            seeds = nseeds;
        }
    }
    seeds.sort();
    *seeds.first().unwrap()
}
