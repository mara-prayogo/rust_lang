use std::io;

fn nth_fibonacci(n:i32) -> i32 {
    let mut fibonacci_sum = 1;
    if n <= 2 { 
        return n-1;
    } else {
        for element in 2..n-2 {
            fibonacci_sum += element;
        }
    }
    return fibonacci_sum;
}

fn main() {
    loop {
        println!("Please enter n: ");

        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n:i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("nth fibonacci: {}", nth_fibonacci(n));

        println!("Keep going? (y/n)[n]:");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans)
            .expect("Failed to read line");

        match ans.as_str().trim() {
            "y" | "Y" => {
                continue;
            },
            _ => {
                break;
            }
        };
    }
}
