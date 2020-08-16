use std::env;

fn main() {
	// Collect all arguments in one Vector
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
}
