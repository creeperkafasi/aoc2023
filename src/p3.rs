use std::{fs, vec};

#[allow(dead_code)]
pub fn answer() {
    let file = fs::read_to_string("inputs/3.txt").unwrap();

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut buf = String::new();
    let width = file.find("\n").unwrap() + 1;
    // Walk over the string and record the numbers and symbols
    file.chars().enumerate().for_each(|(i, ch)| {
        if ch.is_numeric() {
            buf.push(ch);
        } else {
            if !buf.is_empty() {
                numbers.push(Number {
                    value: buf.parse().unwrap(),
                    x: i % width - buf.len() + 1,
                    y: i / width + 1,
                    len: buf.len(),
                });
                buf.clear();
            }
            if ch != '.' && ch != '\n' {
                symbols.push(Symbol {
                    value: ch,
                    x: i % width + 1,
                    y: i / width + 1,
                    numbers: vec![],
                })
            }
        }
    });

    symbols.iter_mut().for_each(|symbol| {
        symbol.numbers = numbers
            .iter()
            .filter(|number| {
                // dbg!(
                //     number.value,

                //     symbol.x
                // );
                ((number.x - 1)..(number.x + number.len + 1)).contains(&(symbol.x))
                    && (number.y as isize - symbol.y as isize).abs() <= 1
            })
            .map(|r| *r)
            .collect::<Vec<Number>>()
    });

    let mut part_numbers_sum = 0;
    let mut gear_ratios_sum = 0;
    for sym in symbols {
        part_numbers_sum += sym.numbers.iter().fold(0, |acc, e| acc + e.value);
        if sym.value == '*' && sym.numbers.len() == 2 {
            gear_ratios_sum += sym.numbers[0].value * sym.numbers[1].value;
        }
    }

    println!(
        "Sum of all part numbers: {}\n Sum of all gear ratios: {}",
        part_numbers_sum, gear_ratios_sum
    );
}

#[derive(Clone, Debug)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
    numbers: Vec<Number>,
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
    len: usize,
}
