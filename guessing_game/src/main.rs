extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);

	println!("Please input your guess.");

	loop {

		let mut guess = String::new();
						io::stdin().read_line(&mut guess)
				.ok()
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Enter a number you moron!!!!!");
				continue;
			}
		};


		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
				Ordering::Less => println!("Too small. What's wrong wth you you idiot!!"),
				Ordering::Greater => println!("Too big, you are an ass!!!"),
				Ordering::Equal => {
				println!("Maddie, You win!!!!!! You are not as dumb as Julia and Waverly.");
				break;
			}
			}
	}
}

