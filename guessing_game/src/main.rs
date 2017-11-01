extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct State {
    // Game state struct
    secret: u32,
    wins: u32,
    tries: u32
}

fn setup (target: &mut State, initial: bool) {
    // Setup a State
    target.secret = rand::thread_rng().gen_range(1, 101);
    target.tries = 0;

    if initial {
        target.wins = 0;
    }
}

fn main() {
    // Create the game state
    let mut game: State = State{secret: 0, wins: 0, tries: 0};
    setup(&mut game, true);

    // Game logic
    println!("Guess the number!");
    loop {
        if game.tries <= 0 {
            println!("Please input your guess.");
        } else {
            println!("Input another guess ({}):", game.tries);
        }

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().to_lowercase().as_ref() {
            "exit" => {
                println!("The game has ended.\nYou won {} times.", game.wins);
                return
            },
            "wins" => {
                println!("You have won {} times.", game.wins);
                continue
            },
            "tries" => {
                println!("You have tried {} times.", game.tries);
                continue
            },
            "cheat" => {
                println!("The secret is {}. Your score has been reset.", game.secret);
                game.wins = 0;
                continue
            }
            &_ => ()
        }

        // Print some informative input, should the input fail
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                game.tries += 1;
                num
            },
            Err(err) => {
                println!("Error: {}", err);
                continue
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&game.secret) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                game.wins += 1;
                println!("You win!\nYou tried {} times and have won {} times.", game.tries, game.wins);
                setup(&mut game, false);
            }
        }
    }

}
