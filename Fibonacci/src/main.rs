use std::io;
use std::process::exit;

fn main() {
    // Take in a number n and calculate the nth Fibonacci number
    // Get input
    let mut n = String::new();
    println!("Please enter the numberth Fibonacci number that you would like to calculate:");
    io::stdin()
        .read_line(&mut n)
        .expect("Invalid entry");
    // Convert input to i32
    //let n : i32 = n.trim().parse().expect("Unable to convert to an integer");

    let n : u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => exit(1)
    };

    let mut ans : u128 = 1;
    let mut prev : u128 = 0;
    for i in (1..n){
        ans = next_fibonacci(ans,prev);
        prev = ans-prev;
        println!("Count:{}, Ans:{}, Prev:{}",i,ans,prev);
    }
    println!("{}th Fibonacci number is {}",n,ans);
    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read line");
}

// calculate the next fibonacci number
fn next_fibonacci(np : u128, npp : u128) -> u128{
    np + npp
}