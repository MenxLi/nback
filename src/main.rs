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
    let inp = get_user_input(
        format!("Input the number of steps back (default is {}): ", config.n).as_str(),
    );
    if inp != "" {
        config.n = inp.parse::<usize>().expect("Invalid number");
    }
    let n = config.n;
    let mut g = Game::new(&config);

    loop {
        Game::clear_screen();
        let val = g.get_next();
        match val {
            Some(v) => {
                println!("The current value is: {v}");
            }
            None => {
                println!("Game over! You have completed all rounds.");
                println!("Your score is: {}/{}.", g.n_correct, g.config.max_rounds);
                println!("The complete sequence is: {g}");
                println!("Bye!");
                break;
            }
        }

        let n_correct = g.n_correct;
        println!(
            "Accuracy: {}/{}/{} [correct/round/total]{}",
            n_correct,
            g.curr_round,
            g.config.max_rounds,
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
