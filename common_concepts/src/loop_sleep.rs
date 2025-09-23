//use std::io;
use std::thread::sleep;
use std::{thread, time};

//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER

//Variables and Mutability 3.1
fn main() {

    let one_sec = time::Duration::from_millis(1000);
    loop {
        println!("AGAIN!");
        sleep(one_sec);
    }
}
