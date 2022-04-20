use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, Rust!");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let some_string = Some("Hello Option");
    match some_string{
        Some(v) => println!("Some: {}", v),
        None => println!("Not gonna happen!")
    }

    let learn = LearnRust::RandomGame;

    execute(learn);

    let learn = LearnRust::Option(10);
    execute(learn);
}

fn execute(learn: LearnRust)
{
    match learn {
        LearnRust::RandomGame => random_game(),
        LearnRust::Option(start) => {
            let plusone = plus_one(Some::<i32>(start));
            match plusone {
                None => println!("Nothing there"),
                Some(value) => println!("Plus one equals: {}", value)
            }
        }
    };
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

enum LearnRust {
    RandomGame,
    Option(i32),
}

fn random_game()
{
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

        // Use the "match" pattern. The parse method returns an enumeration of Ok or Err
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