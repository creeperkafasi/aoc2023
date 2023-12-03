use std::fs;

const DIGITS: &'static [&str] = &[
    "zero", "0", "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
    "seven", "7", "eight", "8", "nine", "9",
];

#[allow(dead_code)]
pub fn answer() {
    let inp = input();
    println!("{}", one(inp.clone()).iter().sum::<u64>());
    println!("{}", two(inp.clone()).iter().sum::<u64>());
}

fn one(inp: Vec<String>) -> Vec<u64> {
    inp.iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .map(|digits| {
            format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<u64>()
                .unwrap()
        })
        .collect()
}

fn two(inp: Vec<String>) -> Vec<u64> {
    inp.iter()
        .map(|line| {
            let mut digs = DIGITS
                .iter()
                .enumerate()
                .map(|(di, ds)| line.match_indices(ds).map(move |index| (index, di / 2)))
                .flatten()
                .collect::<Vec<_>>();
            digs.sort_by(|a, b| a.0.cmp(&b.0));
            digs.first()
                .map(|f| {
                    format!("{}{}", f.1, digs.last().unwrap().1)
                        .parse::<u64>()
                        .unwrap()
                })
                .unwrap()
        })
        .collect::<Vec<u64>>()
}

fn input() -> Vec<String> {
    return fs::read_to_string("inputs/1.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
}
