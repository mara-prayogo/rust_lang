fn main() {
    println!("Hello, world!");
    another_function();
    func_with_parameter(5);
    func_with_2_parameters(7, 8);
}

/* Declaration of the function starts with 'fn' keyword */
fn another_function() {
    println!("Another function!");
}

fn func_with_parameter(x: i32) {
    println!("Value of x: {}", x);
}

fn func_with_2_parameters(x: i32, y: i32) {
    println!("Value of x: {}, y: {}", x, y);
}
