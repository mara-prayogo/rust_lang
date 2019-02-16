/* prelude  */

/* rand crate as an external dependency*/
extern crate rand;

/* using standard library 'std' */
use std::io;
use std::cmp::Ordering;

/* using Rng trait that defines methods that random number generator
 * implement. Chapter 10 covers traits in detail. */
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    /* thread_ng() will give a random number generator local to the current
     * thread and seeded by the OS. */
    /* gen_range is defined in rand::Rng trait. generates random number
     * between 2 numbers provided as arguments. First number is inclusive
     * and the second one is exclusive i.e. [1st, 2nd). */

    let secret_number = rand::thread_rng().gen_range(1,101);
    /*  println!("Secret number is: {}", secret_number); */

    loop {
        println!("Please input your guess.");

        /* 'let' keyword is used to indicate a variable. */
        /* By default, variables are immutable in Rust. */    
        /* Use 'mut' keyword to make a variable mutable. */
        /* String is the string type provided by std */
        /* '::' indicates an associated function. Similar to static methods, */
        /* Which means, the associated function is implemented on a type rather
         * than the instance of the type. */
        /* 'new' function creates a new, empty string. */

        let mut guess = String::new();

        /* stdin function returns std::io::Stdin, which is a type that represents
         * a handle to the standard input for the terminal. */
        /* read_line function of Stdin is used to read input from the user.
         * It takes the input and place that into a string hence it takes string
         * as an argument. the string argument needs to be mutable so that the
         * method can change the string content. */
        /* &mut guess: & indicates that argument is a referenece. References are
         * immutable by default. Hence we need to write '&mut guess' instead of
         * '&guess' to make it immutable. */
        /* read_line also returns a value of type io::Result. Result types are 
         * enumerations having fixed set of values called enum's variants. 
         * For Result, the variants are Ok and Err. Ok=operation successful.
         * Err=operation failed. Err contains info about the failure. 
         * Result types (Err and Ok), have methods defined on them. 'expect' is 
         * one of them. If Result==Err, 'expect' will cause the program to crash. 
         * If Result==Ok, expect will return Ok's value so it can be used further.
         * In this case, the value is number of bytes in what the user entered 
         * into the standard input. If 'expect' is not used, the compiler will
         * print a warning, but the program will compile. */

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /* Convert the input string to the corresponding 32 bit number. 
         * Otherwise the compiler will throw an error. */
        /* Here 'guess' (u32) shadows the previous 'guess' (String).*/
        /* Here the 'parse' method returns a Result type. We use 'expect' 
         * to handle errors if any. */

        /* let guess: u32 = guess.trim().parse()
            .expect("Please type a number!"); */

        /* Using 'match' to move from crashing on an error to handling the
         * error. */
        let guess: u32 = match guess.trim().parse() {
            /* If Ok, 'parse' returns the number. It won't check
             * the arms below. */
            Ok(num) => num,
            /* '_' is catchall value. Meaning we want to match all 
            * the Err values, no matter what info they contain. */
            Err(_) => continue,
        };

        /* println! prints the string saved from the user's input. {} are
         * placeholders. Can use multiple {}. Similar to '%d' in C. */
        println!("You guessed: {}", guess); 

        /* Ordering is an enum like Result. Variants of Ordering are 
         * 1. Less, 2. Greater, 3. Equal. */
        /* 'cmp' method compares 2 values and can be called on anything that
         * can be compared. It returns variant of Ordering enum. */
        /* 'match' here is used to decide what to do next with the Ordering
         * variant returned from 'cmp'. */
        /* 'match' is like a 'switch' statement but it's an exhaustive
         * checking mechanism. It doesn't let any possible value of the 
         * expression slip by. */

        match guess.cmp(&secret_number) {
            /* Below statements are called arms*/
            /* arm: val => expression */
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
