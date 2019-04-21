fn main() {
    /* Using 'loop' construct */
    /* Runs forever until explicitly told to be stopped */

    let mut loop_var = 0;
    loop {
        if loop_var == 5 {
            break;
        }
        println!("again!");
        loop_var += 1;
    }

    /* Conditional loops using 'while' */
    /* Countdown code */
    loop_var = 3;

    while loop_var != 0 {
        println!("{}!", loop_var);

        loop_var -= 1;
    }

    /* Looping through a collection using 'for' */
    let array = [10, 20, 30, 40, 50];

    let mut index = 0;

    /* How it is done using while loop */
    /* But this approach is error prone; we could cause the program to panic
     * if the index length is incorrect. It's also slow, because the compiler
     * adds runtime code to perform the conditional check on every element on
     * every iteration through the loop.*/
    while index < 5 {
        println!("value at index: {} is: {}", index, array[index]);

        index += 1;
    }

    /* More concise alternative is to use 'for' loop and execute code for
     * each item in the collection */
    for element in array.iter() {
        println!("The value is: {}", element);
    }

    /* Countdown code using 'for' loop */
    /* Note that (1..4). It is a Range type provided by standard library
     * (a..b) -> [a,b). i.e. a is inclusive and b is exclusive
     * 'rev' is used to reversed the Range */
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
