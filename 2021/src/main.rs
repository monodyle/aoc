use std::{panic, time::Instant};

mod solutions;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a problem number");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();
    let mut runtime = 0.0;

    for day in days {
        let handler = get_day(day);
        let timer = Instant::now();
        let (part_one, part_two) = handler();
        let elapsed_ms = timer.elapsed().as_nanos() as f64 / 1_000_000_000.0;

        println!("\nRun day {:02} in {}ms:", day, elapsed_ms);
        println!("- Part one: {}", part_one);
        println!("- Part two: {}", part_two);

        runtime += elapsed_ms;
    }

    println!("\nTotal runtime: {:.4}ms", runtime);
}

fn get_day(day: u8) -> fn() -> (u32, u32) {
    match day {
        1 => solutions::day_01::solve,
        _ => unimplemented!(),
    }
}
