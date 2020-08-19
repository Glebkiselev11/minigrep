use std::env;
use std::fs;

fn main() {
	// Collect all arguments in one Vector
	let args: Vec<String> = env::args().collect();

	let (query, filename) = parse_config(&args);

	println!("Searching for {}", query);
	println!("In file {}", filename);

	let contents = fs::read_to_string(filename)
		.expect("Something went wrong reading the file");

	println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
	let query = &args[1];
	let filename = &args[2];

	return (query, filename)
}
