use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", num);
    loop {
        println!("Guess the Number\nPlease enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");



        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            },
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!")
        }
    }
}
