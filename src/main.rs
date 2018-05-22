extern crate rand;

use rand::Rng;
use std::io;
use std::io::Write;
use std::str::FromStr;

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(1,101)
}

fn get_user_input() -> Option<i32> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match i32::from_str(&input.trim()) {
                Ok(number) => Some(number),
                Err(_) => {
                    println!("Please enter a valid number.");
                    None
                }
            }
        },
        _ => {
            println!("Error when getting input value.");
            None
        }
    }
}

fn play() -> bool {
    let mut tries = 10;

    println!("Generate random number...");
    let random_number = generate_random_number();
    println!("Number generated. Try to find out !");

    while tries > 0 {
        print!("Enter a number : ");
        io::stdout().flush();

        match get_user_input() {
            Some(number) => {
                if number < random_number {
                    println!("C'est plus grand !")
                } else if number > random_number {
                    println!("C'est plus petit !")
                } else {
                    return true;
                }
            }
            None => {}
        }
        tries -= 1;
    }
    false
}

fn main() {
    if play() == true {
        println!("Vous avez gagné !");
    } else {
        println!("Vous avez perdu...");
    }
}
