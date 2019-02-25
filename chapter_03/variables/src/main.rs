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

}
