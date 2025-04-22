mod core;

use core::{Game, GameConfig};
use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    String::from(buf.trim())
}

fn main() {
    let mut config = GameConfig::new();
    let mut last_attempt_failed = false;

    Game::clear_screen();
    println!("Welcome to nback game!");
    let inp = get_user_input("Input the number of steps back (default is 1): ");
    if inp != "" {
        config.n = inp.parse::<usize>().expect("Invalid number");
    }
    let n = config.n;
    let mut g = Game::new(&config);

    loop {
        Game::clear_screen();
        let val = g.get_next();
        println!("The current value is: {val}");

        let n_correct = g.n_correct;
        let n_guesses = g.n_guesses;
        println!(
            "Accuracy: {}/{}{}",
            n_correct,
            n_guesses,
            if last_attempt_failed {
                " (last guess failed)"
            } else {
                ""
            }
        );
        println!("----------------------");
        println!("Please input your guess for {n} steps back, or type 'exit' to quit.");
        let ans = get_user_input(if g.should_guess() {
            "Your guess: "
        } else {
            "Press enter to continue... "
        });
        if ans == "exit" {
            println!("Complete sequence: {g}");
            println!("Bye!");
            break;
        }
        if !g.should_guess() {
            continue;
        }

        last_attempt_failed = !g.guess(&ans).expect("Invalid guess");
    }
}
