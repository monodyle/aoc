use std::{panic, time::Instant};

use aoc2022lib::Output;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

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

	let mut runtime = 0;

	for day in days {
		let handler = get_day(day);
		let timer = Instant::now();
		let (part_one, part_two) = handler();
		let elapsed_ms = timer.elapsed().as_micros();

		println!("\nRun day {:02} in {}µs:", day, elapsed_ms);
		println!("- Part one: {}", part_one);
		println!("- Part two: {}", part_two);

		runtime += elapsed_ms;
	}

	println!("\nTotal runtime: {:.4}µs", runtime);
}

fn get_day(day: u8) -> fn() -> (Output, Output) {
	match day {
		1 => day_01::solve,
		2 => day_02::solve,
		3 => day_03::solve,
		4 => day_04::solve,
		5 => day_05::solve,
		6 => day_06::solve,
		7 => day_07::solve,
		8 => day_08::solve,
		_ => unimplemented!(),
	}
}
