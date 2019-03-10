fn main() {
    
    /* Variables */

    /* let x = 5; */

    /* 'mut' makes a variable mutable. All the variables are immutable
     * by default */
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is {}", x);

    /* Constants */

    /* Constants are a bit similar to the immutable variables with some major 
     * differences. 
     * 1. Constants must be annotated with a type. Using 'let' would not work. 
     * 2. Constants are always immutable, not just immutable by default. 
     * 3. Constants can be set only to a constant expression, not to the result
     * of a function call or any other value that could be computed at runtime.
     * */

    const SPEED_OF_LIGHT: u32 = 300_000; // Unit in km/s
    println!("Speed of light is {} km/s.", SPEED_OF_LIGHT);

    /* Shadowing */
    
    /* Shadowing is declaring a new variable with the same name as the
     * previous one, and the new variable shadows the previous one. 
     * Whenever the variable is used, it's the new variable's value which
     * would be reflected. 
     * Use 'let' again with the same variable name to shadow a variable. */
    
    let y = 5;

    let y = y + 1; // y = 5 + 1 = 6;

    let y = y * 2; // y = 6 * 2 = 12;

    println!("The value of y is {}", y);

    /* Shadowing creates new variable using 'let', which is different from using
     * 'mut'. This allows us to change the type of the variable if we are
     * shadowing a variable. */

    let z = 5; // integer
    println!("The value of z is {}", z);

    let z = "string literal"; // string literal
    println!("The value of z is {}", z);

    /* If we had used 
     *      let mut z = 5;
     *      z = "string literal";
     * This would have been a compiler error. */
    
}
