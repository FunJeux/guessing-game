extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("'Guess The Number' Game!");
    
    println!("In this game you'll be guessing the secret number between two excluded values. To start choose the lower and higher limits");

    let mut lower_limit = String::new();
    
    println!("Choose the lower limit (number > 0): ");

    io::stdin().read_line(&mut lower_limit)
    .expect("Failed to read value");

    let lower_limit: u32 = match lower_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    let mut higher_limit = String::new();
    
    println!("Choose the higher limit (number > lower_limit + 1): ");

    io::stdin().read_line(&mut higher_limit)
    .expect("Failed to read value");

    let higher_limit: u32 = match higher_limit.trim().parse() {
        Ok(num) => num,
        Err(_) => 101,
    };

    let secret_number = rand::thread_rng().gen_range(lower_limit..higher_limit);

    let mut moves_made = 0;

    loop {
        println!("Input guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read input!!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed the number: {}", guess);

        moves_made += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less  => println!("Too small!!"),
            Ordering::Greater  => println!("Too big!!"),
            Ordering::Equal  => {
                let total_moves = higher_limit - lower_limit;
                let total_moves_float = total_moves as f64;
                let moves_made_float = moves_made as f64;
                let efficiency: f64 = ((total_moves_float - moves_made_float) / total_moves_float) * 100.00;
                println!("YOU HAVE WON!! \n Guesses Made: {}\n Possible Guesses: {}\n Efficiency: {}%" , moves_made, total_moves, efficiency);
                break;
            },
        }
    }
}