use rand::prelude::*;
use std::fmt;

pub struct GameConfig {
    pub n: usize,
    pub charset: Vec<char>,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            n: 1,
            charset: ('A'..='Z').chain('a'..='z').collect(),
        }
    }
}

pub struct Game<'a> {
    pub n_correct: usize,
    pub n_guesses: usize,
    pub config: &'a GameConfig,

    data: Vec<String>,
    rng: rand::rngs::ThreadRng,
}

impl<'a> Game<'a> {
    pub fn new(config: &'a GameConfig) -> Self {
        Self {
            n_correct: 0,
            n_guesses: 0,
            config: &config,
            data: Vec::new(),
            rng: rand::rng(),
        }
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn get_next(&mut self) -> &String {
        let rng = &mut self.rng;
        let s = self.config.charset[rng.random_range(0..self.config.charset.len())];
        self.data.push(s.into());
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
