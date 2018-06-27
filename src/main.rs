fn invalid_length_exception() {
	println!("usage target/debug/calc <operation> <num1> <num2> ... <numN>");
	println!("example: target/debug/calc sum 4 6 10 12");
	std::process::exit(0);
}

fn main() {

	let length = std::env::args().len();
	if length < 3
	{
		invalid_length_exception();
	}
	for arg in std::env::args()
	{
    	println!("{}", arg);	
	}
	
}
