use std::io;

//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER
fn main() {

	//Fibonacci nth number

	let mut x: i128 = 0;

	let mut y: i128 = 1;

	let mut mediator: i128 = 0;

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line...");

	let mut guess: i128 = guess.trim().parse().expect("fuck");

	while guess > 0{
		if guess % 2 == 0{
			mediator = x + y;
			x = mediator;
			println!("The current fibonacci number is {x}");
		}else{
			mediator = x+y;
			y = mediator;
			println!("The current fibonacci number is {y}");
		}

		guess = guess-1;
	}


	
	
}
