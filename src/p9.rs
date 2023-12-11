use std::fs;

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/9.txt").unwrap();

    f.split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|nums| {
            let mut layers: Vec<Vec<i32>> = vec![nums];
            loop {
                let difs = differences(&layers.last().unwrap());
                layers.push(difs.clone());
                if difs.iter().all(|n| *n == 0) {
                    break;
                }
            }
            layers
        })
        .map(|mut layers| {
            layers.reverse();
            let lc = layers.len();
            for li in 0..lc {
                let d = if li > 0 {
                    (
                        *layers.get(li - 1).unwrap().first().unwrap(),
                        *layers.get(li - 1).unwrap().last().unwrap(),
                    )
                } else {
                    (0, 0)
                };
                let first = *layers.get(li).unwrap().first().unwrap();
                let last = *layers.get(li).unwrap().last().unwrap();
                layers.get_mut(li).unwrap().insert(0, first - d.0);
                layers.get_mut(li).unwrap().push(last + d.1);
            }
            (
                *layers.last().unwrap().first().unwrap(),
                *layers.last().unwrap().last().unwrap(),
            )
        })
        .fold((0, 0), |acc, e| {
            (acc.0 + e.0 as usize, acc.1 + e.1 as usize)
        })
}

fn differences(nums: &Vec<i32>) -> Vec<i32> {
    nums.windows(2).map(|n| n[1] - n[0]).collect()
}
