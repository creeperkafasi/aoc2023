use std::{env::args, time};

mod p1;
mod p2;
mod p3;
mod p4;

fn main() {
    let mut day = 0;
    let mut timer = false;
    let mut benchmark = false;
    args().skip(1).for_each(|arg| {
        if let Ok(num) = arg.parse::<usize>() {
            day = num;
        } else {
            match arg.as_str() {
                "-t" => timer = true,
                "-b" => benchmark = true,
                _ => panic!("Unknown option: \"{arg}\""),
            }
        }
    });

    let f = match day {
        0 => panic!("Please enter day"),
        1 => p1::answer,
        2 => p2::answer,
        3 => p3::answer,
        4 => p4::answer,
        _ => panic!("Answer for day {day} not found"),
    };

    if timer {
        println!("Measuring run time for day {day}...");
        let tstart = time::Instant::now();
        f();
        let elapsed = tstart.elapsed();
        println!(
            "Day {day} completed in {} ms ({} µs)",
            elapsed.as_millis(),
            elapsed.as_micros()
        );
    } else if benchmark {
        println!("Measuring average run time for day {day} over 1000 calls...");
        let tstart = time::Instant::now();
        for _ in 0..1000 {
            f();
        }
        println!(
            "Day {day} benchmark completed in {:6} seconds",
            tstart.elapsed().as_secs_f32()
        );
        println!(
            "   Average: {} ms ({} µs)",
            tstart.elapsed().as_millis() / 1000,
            tstart.elapsed().as_micros() / 1000
        );
    }

    let answer = f();
    println!(
        "Answers to day {day}:\n    Part 1: {}\n    Part 2: {}",
        answer.0, answer.1
    );
}
