extern crate rand;

use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::cmp::max;
use rand::Rng;

#[derive(Clone)]
struct GameInput(u32, String);

struct State {
    // Game state struct

    secret: u32,
    wins: u32,
    tries: u32,
    tries_total: u32,
    ended: bool,

    command_log: HashMap<u32, GameInput>,
    log_index: u32
}


fn setup (target: &mut State, initial: bool) {
    // Setup a State

    target.secret = rand::thread_rng().gen_range(1, 101);
    target.tries = 0;
    target.ended = false;

    if initial {
        target.wins = 0;
        target.tries_total = 0;
    }
}

fn format_log(state: &State) -> String {
    // Turn a command log into something readable
    let mut unsorted: Vec<(&u32, &GameInput)> = Vec::new();
    for i in state.command_log.iter() {
        unsorted.push(i);
    }
    unsorted.sort_by_key(|k| k.0);
    if state.log_index < 4 {
        return unsorted.iter().fold("tries\t| command\n".to_string(),
                |output, input| format!("{}{}\t| {}\n", output, (input.1).0, (input.1).1));
    } else {
        return unsorted[max(1, state.log_index-4) as usize .. state.log_index as usize].iter().fold("tries\t| command\n".to_string(),
                |output, input| format!("{}{}\t| {}\n", output, (input.1).0, (input.1).1));
    }
}

fn parse_input(game: &mut State) -> Result<u32, String> {
    // Parse input and perform game logic

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    guess = guess.trim().to_lowercase();

    game.command_log.insert(game.log_index, GameInput(game.tries, guess.clone()));
    game.log_index += 1;

    // Attempt to parse a u32 from the text
    let parsed = guess.parse::<u32>();
    match parsed {
        Ok(num) => {
            return Ok(num)
        },
        Err(_) => ()
    }

    // Check if it's a valid command
    match guess.as_ref() {
        "exit" => {
            game.ended = true;
            return Err(format!("The game has ended.\nYou won {} times using {} tries.", game.wins, game.tries_total));
        },
        "wins" => {
            return Err(format!("You have won {} times.", game.wins));
        },
        "tries" => {
            return Err(format!("You have tried {} times for this number and {} times in total.", game.tries, game.tries_total));
        },
        "cheat" => {
            game.wins = 0;
            return Err(format!("The secret is {}. Your score has been reset.", game.secret));
        },
        "history" => {
            return Err(format_log(game));
        }

        // Unknown command
        &_ => return Err(format!("In parsing u32, unknown command or number: {}", guess))
    }
}

fn take_guess(mut game: &mut State, guess: u32) {
    // Take a guess and update state accordingly

    game.tries += 1;
    game.tries_total += 1;

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

fn main() {
    // Create the game state
    let mut game: State = State{secret: 0, wins: 0, tries: 0, tries_total: 0,
                    ended: false, command_log: HashMap::new(), log_index: 0};
    setup(&mut game, true);

    println!("Guess the number!");
    while !game.ended {
        if game.tries <= 0 {
            println!("Please input your guess.");
        } else {
            println!("Input another guess ({}):", game.tries);
        }

        // Grab guess/execute command
        let guess: u32;
        let input = parse_input(&mut game);
        match input {
            Ok(num) => guess = num,
            Err(e) => {
                eprintln!("{}", e);
                continue
            }
        }

        // Take guess
        println!("You guessed: {}", guess);
        take_guess(&mut game, guess);
    }

    println!("Your history follows\n{}\n{}", format_log(&game), format_log(&game));
}
