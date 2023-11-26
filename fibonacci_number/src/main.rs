use num_traits::{Zero, One};
use num_bigint::BigUint;
use std::{io};

fn main() {
    println!("Input the fibonacci term that you want to find: ");

    let fibonacci_term: usize = loop {
        let mut fibonacci_term = String::new();

        io::stdin().read_line(&mut fibonacci_term)
            .expect("Failed to read line");

        match fibonacci_term.trim().parse() {
            Ok(number) => break number,
            _ => {
                if fibonacci_term.trim().parse::<f64>().is_ok() {
                    println!("Error: It must be a whole number.");
                } else {
                    println!("Error: Please input a valid number.");
                }
                continue;
            }
        };
    };

    let fibonacci_number: BigUint = find_fibonacci_number(fibonacci_term);
    let index = fibonacci_term + 1;
    let suffix = match index {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    };

    let result: String = format!("The {index}{suffix} fibonacci number is: {fibonacci_number}");

    println!("{}", result);
}

fn find_fibonacci_number(n: usize) -> BigUint {
    if n.is_zero() {
        return Zero::zero();
    }

    if n.is_one() {
        return One::one();
    }

    let mut previous: BigUint = Zero::zero();
    let mut current: BigUint = One::one();

    for _ in 2..n + 1 {
        let sum = previous + &current;
        previous = current;
        current = sum;
    }

    return current;
}
