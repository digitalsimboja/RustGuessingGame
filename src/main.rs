use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

  
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less"),
            Ordering::Greater => println!("Your guess is greater"),
            Ordering::Equal => {
                println!("Congratulations!!! You win.");
                println!("The secret number is: {}", secret_number);
                break;
            },
        }
        println!("You guessed: {}", guess);


    }

   
}
