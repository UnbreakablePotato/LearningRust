use std::io;

//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER

//Variables and Mutability 3.1
fn main() {
	let a = [1,2,3,4,5];

	println!("Please enter an array index.");

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to readline");

	let index:usize = index.trim().parse()
		.expect("Index entered was not a number");

	let element = a[index];

	println!("The value of the element at index {index} is: {element}");
}
