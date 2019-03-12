/* A function that returns i32 (integer)
 * without return statement as it is an expression */
fn two() -> i32 {
    2
}

fn plus_one(x:i32) -> i32 {
    x + 1
}

fn main() {
    /* Statements and Expressions in Function Bodies  */

    /* Statements are instructions that perform some actions and do not 
     * return any value */
    let x = 5;
    println!("Value of x is {}", x);

    /* Below code is invalid in Rust, since 'let' is used in a statement */
    // let x = (let y = 6);

    /* Expressions are enclosed within '{}' */
    let y = {
        let x = 5;
        x + 1   //returns 6
    };
    println!("Value of y is {}", y);

    /* Functions with return values */
    let two = two();
    println!("Value of two is {}", two);

    println!("Adding 1 to two yields {}", plus_one(two));

}
