use std::env;
use std::fs;

fn main() {
	// Collect all arguments in one Vector
	let args: Vec<String> = env::args().collect();

	let config = parse_config(&args);

	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);

	let contents = fs::read_to_string(config.filename)
		.expect("Something went wrong reading the file");

	println!("With text:\n{}", contents);
}

struct Config {
	query: String,
	filename: String
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filename = args[2].clone();

	return Config { query, filename }
}
