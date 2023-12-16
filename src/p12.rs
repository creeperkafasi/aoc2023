use std::fs;

pub fn answer() -> (usize, usize) {
    let f = fs::read_to_string("inputs/12.txt").unwrap();

    (
        f.lines()
            .map(|line| {
                let (map, cond) = line.split_once(" ").unwrap();

                first(
                    map.to_string(),
                    cond.split(",").map(|n| n.parse().unwrap()).collect(),
                    0,
                )
            })
            .sum(),
        0,
    )
}

fn string_with_replaced_char(s: &String, i: usize, c: char) -> String {
    let mut s = s.clone();
    s.replace_range(i..i + 1, c.to_string().as_str());
    s
}

fn first(map: String, cond: Vec<usize>, i: usize) -> usize {
    if i == map.len() {
        return if is_valid(map + ".", cond) { 1 } else { 0 };
    }
    if map.chars().nth(i) == Some('?') {
        return first(string_with_replaced_char(&map, i, '#'), cond.clone(), i + 1)
            + first(string_with_replaced_char(&map, i, '.'), cond.clone(), i + 1);
    } else {
        first(map, cond, i + 1)
    }
}

fn is_valid(map: String, cond: Vec<usize>) -> bool {
    let mut current = 0;
    let mut seen = vec![];

    for c in map.chars() {
        if c == '.' {
            if current > 0 {
                seen.push(current);
            }
            current = 0;
        } else {
            current += 1;
        }
    }

    seen == cond
}
