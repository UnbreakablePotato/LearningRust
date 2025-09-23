//use std::io;

//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER

//Variables and Mutability 3.1
fn main() {
	let x = plus_one(5);

	println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32{
	x+1
}
