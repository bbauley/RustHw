/*
 * Brandon Bauley
 * CS 410P Rust Programming
 * Homework #1 - Write Some Rust
 * Apply operations on an arbitrary number of arguments
 * example 1: sum 1 2 3 4 5
	 15
 * example 2: gcd 12 24 36
	 12
*/

use std::env;
use std::process;
use std::str::FromStr;

fn invalid_length_exception() {
	println!("usage: target/debug/calc <operation> <num1> <num2> ... <numN>");
	println!("example: target/debug/calc sum 4 6 10 12");
	process::exit(0);
}

//Returns the sum of all of the values inside of the vector
fn sum(numbers: &Vec<u64>) -> u64 {
	let mut sum: u64 = 0;
	for num in numbers {
		sum += num;
	}
	sum
}

//Returns the product of all of the values inside of the vector
fn product(numbers: &Vec<u64>) -> u64 {
	let mut product: u64 = 1;
	for num in numbers {
		product *= num;
	}
	product
}

//Returns the greatest common divisor from a vector of integers
fn gcd_wrapper(numbers: &Vec<u64>) -> u64 {
	let mut result = gcd(numbers[0], numbers[1]);
	for i in 2..numbers.len() {
		if result == 0 {
			break;
		}
		result = gcd(result, numbers[i]);
	}
	result
}

//Finds the greatest common divisor of a pair of numbers and returns the result
fn gcd(mut n: u64, mut m: u64) -> u64 {
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m = m % n;
	}
	n
}

//Returns the least common multiple of all integers in a vector
fn lcm(numbers: &Vec<u64>) -> u64 {
	let mut gcd_variable = gcd(numbers[0], numbers[1]);
	let mut product = numbers[0] * numbers[1];
	if gcd_variable == 0 {
		gcd_variable = 1;
	}
	let mut result = product / gcd_variable;
	for i in 2..numbers.len() {
		gcd_variable = gcd(result, numbers[i]);
		product = result * numbers[i];
		if gcd_variable == 0 {
			gcd_variable = 1;
		}
		result = product / gcd_variable;
	}
	result
}

fn main() {
	let mut numbers = Vec::new();
	let length = env::args().len();

	if length < 3 {
		invalid_length_exception();
	}
	for arg in env::args().skip(2) {	//Need to ignore first two arguments
    numbers.push(u64::from_str(&arg)
			.expect("error parsing argument"));	
	}
	if numbers.len() == 1 {
		println!("{}", numbers[0]);
		process::exit(0);
	}

	let operation = env::args().nth(1);
	let temp = operation.unwrap_or("None".to_string()); //These two lines convert an Option<String> type
	let value = temp.as_str();		 											// 						into a str type
	let result = {
		match value {
			"sum" 	  => sum(&numbers), 
			"product" => product(&numbers),
			"gcd" 	  => gcd_wrapper(&numbers),
			"lcm" 	  => lcm(&numbers),
			_ 			  => {
				println!("Invalid Operation: {}", value);
				println!("Operations available...");
				println!("sum\nproduct\ngcd\nlcm");
				process::exit(0);
			}	
		}
	};
	println!("{:?}", numbers);
	println!("{} is {:?}", value, result);
}

#[test]
fn test_sum() {
	let mut var = Vec::new();
	var.push(10);
	var.push(20);
	var.push(30);
	var.push(400);
	assert_eq!(sum(&var), 460);
	var.pop();
	assert_eq!(sum(&var), 60);
}

