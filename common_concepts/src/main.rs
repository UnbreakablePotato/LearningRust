//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER

//Variables and Mutability 3.1
fn main() {
    let x = 5;

    println!("The value of x is: {x}");

    //x = 6;//kun tilladt hvis x er mutable

    println!("The value of x is: {x}"); //tilgngeld m vi gerne ndre x ved at f.eks.
    // x + 2, men muligvis kun i println?

    const THREE_HOURS_IN_SECONDS:u16 = 60*60*3;

    println!("There are {THREE_HOURS_IN_SECONDS} in three hours.");

    let x = x+1; //A way to change x without it being initially mutable, called shadowing

    {
        let x = x*2;
        println!("The value of x in the inner scope is; {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    /*let mut spaces = "  ";
    spaces = spaces.len();*/

    //will result in a compile error, as you cannot change type of a variable if it is
    //only mutable, you need to shadow by using the let keyword again to rebind spaces.

    //DATA TYPES 3.2

    let guess: u32 = "42".parse().expect("Not a number");

    //Mathematical Operations in RUST

    let sum = 5 + 10;

    let difference = 95.5 - 4.3; //float 64 signed if unspecified, roughly same speed as f32

    let product = 4*30; //default is i32(signed 32 bit)

    let quotient = 56.7/32.2;
    let truncated = -5/3;//Results in -1

    let remainder = 43 % 5;

    //Boolean type

    let t = true; //type Boolean

    let f: bool = false;// with explicit type annotation

    //Character type

    let c = 'z';
    let z: char = 'Z';
    

    //The Tuple type
    let tup: (i32,f64,u8) = (500,6.4,1);

    let tuup = (500,6.4,1);

    let (x,y,z) = tuup;

    println!("The value of y is: {}, the value of z is {}", y,z);

    //ARRAY TYPE IN RUST

    let _array = [1,2,3,4,5]; //ALLOCATED ON STACK

    let _arr2: [i32,5] =[5,4,3,2,1];

    let first = a[0];

    let seconds = a[1];
}
