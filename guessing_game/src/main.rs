use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let secret_number: u32 = rand::rng().random_range(1..=100);

  println!("Guess the number!");
  println!("Please input your guess.");

  loop {
    let mut guess = String::new();

    io::stdin()
      // & since read_line does not take ownership of guess, just borrows it to mutate it
      .read_line(&mut guess)
      // This blows up if the actual stdin cannot read the line, this does not have
      // to do with the guess variable
      .expect("Failed to read line");

    // parse() tries to parse it into the type of the left side, so u32
    let guess: u32 = match guess.trim().parse() {
      // num is just a temp variable that lives within the scope of the match arm
      Ok(num) => num,
      // _ means we don't care what the actual error was
      Err(_) => {
        println!("Please input a number!");
        continue;
      }
    };

    // &secret_number since we are just borrowing it, we don't want cmp to own it
    // (cmp only takes in a &, we couldn't even give it ownership if we wanted to)
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
