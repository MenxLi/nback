use rand::prelude::*;
use std::fmt;

pub struct GameConfig {
    pub n: usize,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig { n: 1 }
    }
}

pub struct Game<'a> {
    data: Vec<String>,
    pub n_correct: usize,
    pub n_guesses: usize,
    pub config: &'a GameConfig,
}

impl<'a> Game<'a> {
    pub fn new(config: &'a GameConfig) -> Self {
        Self {
            data: Vec::new(),
            n_correct: 0,
            n_guesses: 0,
            config: &config,
        }
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn get_next(&mut self) -> &String {
        let mut rng = rand::rng();
        let s = rng.sample(rand::distr::Alphanumeric).to_string();
        self.data.push(s);
        self.data.get(self.data.len() - 1).unwrap()
    }

    pub fn should_guess(&self) -> bool {
        if self.data.len() < self.config.n + 1 {
            false
        } else {
            true
        }
    }

    pub fn guess(&mut self, s: &String) -> Result<bool, String> {
        if !self.should_guess() {
            return Err("Not enough data to guess".to_string());
        }
        let aim = &self.data[self.data.len() - 1 - self.config.n];
        self.n_guesses += 1;
        if s == aim {
            self.n_correct += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // https://stackoverflow.com/a/62101709
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl<'a> fmt::Display for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut concat = String::new();
        self.data().iter().for_each(|s| {
            concat.push_str(&s);
            concat.push(' ');
        });
        write!(f, "[ {}]", concat)
    }
}
