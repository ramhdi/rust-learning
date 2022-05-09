use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    println!("Please input the guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess:u32 = guess.trim().parse().expect("Failed to parse number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }
}
