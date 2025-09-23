//ALL COMMENTS SHOULD BE IN ENGLISH AS CERTAIN DANISH LETTERS ARE NOT ALLOWED BY
//THE RUST COMPILER
fn main() {
    let x = 5;

    println!("The value of x is: {x}");

    //x = 6;//kun tilladt hvis x er mutable

    println!("The value of x is: {x}"); //tilgngeld m vi gerne ndre x ved at f.eks.
    // x + 2, men muligvis kun i println?

    const THREE_HOURS_IN_SECONDS:u32 = 60*60*3;

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


}
