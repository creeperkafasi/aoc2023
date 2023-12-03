use core::num;
use std::{fs, io::Write, os::fd::AsFd};

const NEIGHBOURS: [(usize, usize); 8] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (0, 1),
    // (1, 2),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
];

#[allow(dead_code)]
pub fn answer() {
    let map = input();
    let width = map[0].len();
    let height = map.len();

    let mut nmap: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for r in 0..height {
        for c in 0..width {
            if !(map[r][c] as char).is_numeric() && (map[r][c] as char) != '.' {
                for (dx, dy) in NEIGHBOURS {
                    nmap[r + dy - 1][c + dx - 1] = true;
                }
            }
        }
    }

    let mut res = 0;
    for r in 0..height {
        let mut buf = String::new();
        let mut valid = false;
        for c in 0..width {
            if (map[r][c] as char).is_numeric() {
                buf = format!("{}{}", buf, map[r][c] as char);
                if nmap[r][c] {
                    valid = true;
                }
            }
            if !(map[r][c] as char).is_numeric() || c == width-1 {
                if valid {
                    // let mut f = fs::OpenOptions::new()
                    //     .append(true)
                    //     .create(true)
                    //     .open("/tmp/day3")
                    //     .unwrap();
                    // f.write_all(format!("{}\n", buf.clone()).as_bytes());
                    // dbg!(buf.clone());
                    res += buf.parse::<u128>().unwrap();
                }
                buf = "".to_string();
                valid = false;
            }
        }
    }

    // for r in nmap {
    //     for c in r {
    //         print!(
    //             "{}",
    //             match c {
    //                 true => '#',
    //                 false => '.',
    //             }
    //         )
    //     }
    //     println!()
    // }

    println!("{}", res);
}

fn input() -> Vec<Vec<u8>> {
    fs::read_to_string("inputs/3.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect()
}
