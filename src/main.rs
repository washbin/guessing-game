use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
