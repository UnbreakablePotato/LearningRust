use std::io;

//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER
fn main() {

	println!("Type the amount of degrees in Celsius you wish to convert");

	let mut celsius = String::new();

	io::stdin()
		.read_line(&mut celsius)
		.expect("Failed to read line");

	let celsius: f64 = celsius.trim().parse().expect("FUCK");

	//println!("{celsius}");

	let k = 9.0/5.0;

	let f = (celsius * k) + 32.0;

	println!("{celsius} degrees celsius in fahrenheit is: {f}");
	
}
