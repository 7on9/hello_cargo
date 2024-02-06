use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub(crate) fn guess_the_number() {
  println!("********** Guess the number **********");
  let random_number = rand::thread_rng().gen_range(1..=100);
  
  loop {
      println!("Please input your guess. (1-100), or 'q' to quit.");
      let mut guess = String::new();
      
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

      if guess.trim() == "q" {
        println!("Quitting...");
        break;
      }
      println!("You guessed: {guess}");

      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      };

      match guess.cmp(&random_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
              println!("You win!");
              break;
          }
      }
  }
  return;
}