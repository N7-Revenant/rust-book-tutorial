use std::io;

const FIB_1: u128 = 0;
const FIB_2: u128 = 1;

fn generate_fibonacci_number(position: u128) -> u128 {
    if position == 1 {
        FIB_1
    }
    else if position==2 {
        FIB_2
    }
    else {
        let mut fib1: u128 = FIB_1;
        let mut fib2: u128 = FIB_2;
        let mut sum: u128 = 0;
        for _ in 2..position {
            sum = fib1 + fib2;
            fib1 = fib2;
            fib2 = sum;
        }
        sum
    }
}

fn main() {
    println!("Fibonacci sequence number generator started...");
    println!("It can generate fibonacci sequence numbers on positions from 1 to 187");

    loop {
        println!("Please input position of desired sequence element:");

        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        let position: u128 = match position.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Position can't be lower then 1!");
                    continue
                }
                else if num > 187 {
                    println!("Position can't be greater then 187!");
                    continue
                }
                else {
                    num
                }
            }
            Err(_) => {
                println!("Incorrect input!");
                continue
            }
        };

        let fibonacci_num: u128 = generate_fibonacci_number(position);
        println!("Fibonacci sequence number on position '{position}' is: '{fibonacci_num}'");
        break
    }
}
