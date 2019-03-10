fn main() {

    /* There are 2 kinds of data types */
    /* 1. Scalar Types 2. Compound Types */

    /* 1. Scalar Types */
    /* There are 4 types of scalar types */

    /* i. Integer type */
    /* Variants:
     * i8, u8
     * i16, u16
     * i32 (default), u32
     * i64, u64
     * isize, usize */
    let integer: u32 = 250;
    println!("The integer value is {}", integer);

    /* ii. Floating type */
    /* Variants:
     * f32, f64 (default) */
    let float: f64 = 3.5;
    println!("The float value is {}", float); 

    /* iii. Boolean type */
    /* true or false */
    let true_value = true;
    let false_value: bool = false; // with explicit type annotation
    println!("Does the Earth rotate around the Sun?: {}", true_value);
    println!("Is the Earth flat?: {}", false_value);

    /* iv. Character  */
    let c = 'c';
    let z = '\u{2602}';
    println!("Characters: {} {}", c, z);

    /* 2. Compound Types  */

    /* i. Tuples */
    /* A tuple is a general way of grouping together some number of other 
     * values with a variety of types (heterogeneous elements) 
     * into one compound type */
    let tup: (i32, f64, u8) = (1992, 6.0, 2);

    /* To get the individual values out of a tuple, we can use pattern 
     * matching to "destructure" a tuple value, like this: */
    let (y, m, d) = tup;
    println!("DOB is: {}-{}-{}", y, m, d);
   
    /* Another way to access elements of a tuple */
    let year = tup.0;
    let month = tup.1;
    let date = tup.2;
    println!("DOB is: {}-{}-{}", year, month, date);

    /* ii. Arrays */
    /* Arrays are a way to group homogeneous elements. i.e. Each element
     * in an array is of same type. 
     * Arrays in Rust have fixed length. Once declared, they cannot grow
     * or shrink in size. */
    let the_array = [1, 2, 3, 4, 5];

    /* Accessing the array elements */
    let zeroth = the_array[0];
    let first = the_array[1];
    println!("Array elements are {} {}", zeroth, first);

    /* Accessing an array element outside of the bounds result in a 'panic', 
     * which is a runtime error (and not compile time) */

    /* Below code creates a panic like this: 
     * thread 'main' panicked at 'index out of bounds: the len is 5 but the 
     * index is 10', src/main.rs:71:46 note: Run with `RUST_BACKTRACE=1` 
     * for a backtrace.*/
    //let index = 10;
    //println!("Array element at index is {}", the_array[index]);
}

