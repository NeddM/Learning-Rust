use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please enter a number to guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u8 = guess.trim().parse().expect("Please type a number");

    println!("You guessed {}", guess);

    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
