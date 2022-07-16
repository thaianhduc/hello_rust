use std::{io, cmp::Ordering};
use rand::Rng;

//pub mod ty

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

    let mut array: Vec<i32> = Vec::new();
    array.push(1);
    array.push(4);

    let second: i32 = array[1];
    println!("Array at 1 index {}", second);
    let second_again: i32 = array[1];
    println!("Array at 1 index again {}", second_again);
    println!("Array at 1 index work? {}", second);

    let vec_macro = vec![String::from("Hello"), String::from("Reference type")];
    let hell = &vec_macro[0];
    println!("Hello hell cool: {}", hell);


    println!("Playing around with generics");
    let numbers = [2, 3, 1, 8];
    let largest = largest(&numbers);

    println!("Largest number {}", largest);
}

/// Find the largest value in a list
fn largest<T : PartialOrd>(list: &[T]) -> &T {
    let mut index: usize = 0;
    let mut largest_value_index: usize = 0;

    while index < list.len() - 1 {
        if list[index+1] > list[index]  {
            largest_value_index = index + 1;
        }
        index = index + 1;
    }
    

    &list[largest_value_index]
}

fn execute(learn: LearnRust){
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