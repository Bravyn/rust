use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\nWelcome to the Snarky Guessing Game\n");
    println!("I am thinking of a number between 1 and 100\n
              I will not judge you if you cannot guess it");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    let max_attempts = 7;

    loop {
        println!("\nTake a guess (or type 'quit' to give up)\n");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        //allow player to quit
        if guess.trim().eq_ignore_ascii_case("quit"){
            println!("\nA wise choice. The number was {}. Maybe next time!",
            secret_number);

            break;
        }

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number, try harder");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number){
            Ordering::Less => {
                if attempts == max_attempts {
                    println!("Wow, {} attempts and still too low\n
                    Maybe reconsider your life choices", attempts);

                }
                else {
                    println!("Too small, are you even trying");
                }
            }
            Ordering::Greater => {
                if attempts == max_attempts {
                    println!("{} attempts and you are still overshooting? Impressive\n
                    ...ly bad", attempts);

                }
                else {
                    println!("Too big! Did you miss math class?");
                }
            }

            Ordering::Equal => {
                println!("Congratulations! You guessed it in {} attempts", attempts);
                match attempts {
                    1 => println!("Beginner's luck, I am sure"),
                    2..=4 => println!("Not bad...for a human"),
                    5..=6 => println!("Well, you got there eventually"),
                    _ => println!("It is about time"),
                }
                break;
            }
        }

        if attempts >= max_attempts {
            println!("\nGame over! The number was {}. Better luck next time!", secret_number);
            println!("Or maybe guessing games are not your strong suit");
            break;

            wait_for_exit();
        }
    }
}

fn wait_for_exit(){
    let mut placeholder = String::new();
    io::stdin()
    .read_line(&mut placeholder)
    .expect("Failed to read line");
}