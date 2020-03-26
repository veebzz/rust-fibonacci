extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;
use std::mem::replace;

fn main() {
    println!("Find the nth Fibonacci number where n is ....");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to get number");

    let answer: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("oh no.., {}", n),
    };

    if answer == 0 {
        panic!("NONONONONONONOOOOOOOOOOOO NO ZERO......NO!...STOP IT");
    }

    println!("The result is {}", fib(answer));
}

fn fib(n: u32) -> BigUint {
    let mut n1: BigUint = Zero::zero();
    let mut n2: BigUint = One::one();
    for _ in 1..n {
        let n3 = n1 + &n2;
        n1 = replace(&mut n2, n3);
    }
    n1
}
