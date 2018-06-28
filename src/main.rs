use std::env;
use std::process;
use std::str::FromStr;

fn invalid_length_exception() {
	println!("usage: target/debug/calc <operation> <num1> <num2> ... <numN>");
	println!("example: target/debug/calc sum 4 6 10 12");
	process::exit(0);
}

fn main() {
	let mut numbers = Vec::new();
	let length = env::args().len();
	let operation = env::args().nth(1);

	if length < 3
	{
		invalid_length_exception();
	}
	for arg in env::args().skip(2)	//Need to ignore first two arguments
	{
    numbers.push(u64::from_str(&arg)
			.expect("error parsing argument"));	
	}
	if numbers.len() == 1
	{
		println!("{}", numbers[0]);
		process::exit(0);
	}
}
