use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Rust!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("Super secret number is {}", secret_number);

    println!("Enter your guess:");

    loop {
        // Declare a guess variable as a String, which is about to mutate (mut)
        let mut guess = String::new();

        // Read the value from a terminal and assign it to guess, it will mutate guess
        // Handle the Result with "expect". The "expect" means if failed (for any reason), display the message and crash
        io::stdin()
            .read_line(&mut guess)
            .expect("You have to enter a number");

        println!("Your guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Not a number. Try again!");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Great job!");
                break;
            }
        };
    }
}
