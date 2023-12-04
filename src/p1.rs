use std::fs;

const DIGITS: &'static [&str] = &[
    "zero", "0", "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
    "seven", "7", "eight", "8", "nine", "9",
];

pub fn answer() -> (usize, usize) {
    let inp = input();
    (one(inp.clone()), two(inp.clone()))
}

fn one(inp: Vec<String>) -> usize {
    inp.iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>()
        })
        .map(|digits| 10 * digits[0] as usize + digits[digits.len() - 1] as usize)
        .sum()
}

fn two(inp: Vec<String>) -> usize {
    inp.iter()
        .map(|line| {
            let mut digs = DIGITS
                .iter()
                .enumerate()
                .map(|(di, ds)| line.match_indices(ds).map(move |index| (index, di / 2)))
                .flatten()
                .collect::<Vec<_>>();
            digs.sort_by_key(|d| d.0 .0);
            10 * digs.first().unwrap().1 + digs.last().unwrap().1
        })
        .sum()
}

fn input() -> Vec<String> {
    return fs::read_to_string("inputs/1.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
}
