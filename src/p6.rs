use std::fs;

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/6.txt").unwrap();
    let (time_line, record_line) = f.split_once("\n").unwrap();

    (
        first(time_line.to_string(), record_line.to_string()),
        second(time_line.to_string(), record_line.to_string()),
    )
}

fn first(time_line: String, record_line: String) -> usize {
    let times: Vec<usize> = time_line
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let records: Vec<usize> = record_line
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| {
            let time = *t as f32;
            let record = *r as f32;

            let delta = time.powi(2) - 4.0 * record;

            let x1 = ((-time + delta.sqrt()) / -2.0).floor() + 1.0;
            let x2 = ((-time - delta.sqrt()) / -2.0).ceil();

            (x2 - x1) as usize
        })
        .fold(1, |acc, n| acc * n)
}

fn second(time_line: String, record_line: String) -> usize {
    let time: f64 = time_line
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();
    let record: f64 = record_line
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();

    let delta = time.powi(2) - 4.0 * record;

    let x1 = ((-time + delta.sqrt()) / -2.0).floor() + 1.0;
    let x2 = ((-time - delta.sqrt()) / -2.0).ceil();

    (x2 - x1) as usize
}
