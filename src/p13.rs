use std::fs;

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/13.txt").unwrap();

    (
        f.split("\n\n")
            .map(|map| {
                let lines = map.lines().collect::<Vec<_>>();

                let height = lines.len();
                let width = lines.first().unwrap().len();

                (1..height)
                    .find(|y| {
                        let (top, bottom) = lines.split_at(*y);

                        let mut tskip = 0;
                        let mut bskip = 0;
                        if top.len() > bottom.len() {
                            tskip = top.len() - bottom.len();
                        }
                        if bottom.len() > top.len() {
                            bskip = bottom.len() - top.len();
                        }

                        let top = top.iter().skip(tskip);
                        let bottom = bottom.iter().rev().skip(bskip);

                        top.zip(bottom).all(|(a, b)| a == b)
                    })
                    .and_then(|horizontal_reflection| Some(horizontal_reflection * 100))
                    .or_else(|| {
                        (1..width).find(|x| {
                            lines.iter().all(|line| {
                                let (left, right) = line.split_at(*x);

                                let mut lskip = 0;
                                let mut rskip = 0;
                                if left.len() > right.len() {
                                    lskip = left.len() - right.len();
                                }
                                if right.len() > left.len() {
                                    rskip = right.len() - left.len();
                                }

                                let left = left.chars().skip(lskip);
                                let right = right.chars().rev().skip(rskip);

                                left.zip(right).all(|(a, b)| a == b)
                            })
                        })
                    })
                    .or_else(|| {
                        println!("{}", map);
                        panic!()
                    })
                    .unwrap()
            })
            .sum::<usize>(),
        0,
    )
}
