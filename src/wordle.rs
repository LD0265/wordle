use colored::Colorize;
use rand::Rng;
use std::fs;
use std::path::Path;

pub struct Wordle {
    word: String,
    last_guessed_word: String,
    guessed_words: Vec<String>,
    guessable_word_list: Vec<String>,
    chooseable_word_list: Vec<String>,
    round_number: u8
}

impl Wordle {
    pub fn new() -> Wordle {
        let guessable_file_path = Path::new("guessable-words.txt");
        let chooseable_file_path = Path::new("chooseable-words.txt");

        if !guessable_file_path.exists() || !chooseable_file_path.exists() {
            panic!("Required files not found. Ensure 'guessable-words.txt' and 'chooseable-words.txt' are in the same directory as the executable.");
        }

        let guessable_file_content =
            fs::read_to_string(guessable_file_path).expect("Failed to read 'guessable-words.txt'");
        let chooseable_file_content =
            fs::read_to_string(chooseable_file_path).expect("Failed to read 'chooseable-words.txt'");

        let guessable_word_list: Vec<String> = guessable_file_content
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        let chooseable_word_list: Vec<String> = chooseable_file_content
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Self {
            word: "debug".to_string(), // just incase
            last_guessed_word: "".to_string(),
            guessed_words: vec![],
            guessable_word_list,
            chooseable_word_list,
            round_number: 0
        }
    }

    pub fn has_won(&self) -> bool {
        self.word == self.last_guessed_word
    }
    
    pub fn has_lost(&self) -> bool {
        self.round_number == 6
    }
    
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn make_guess(&mut self, guess: &String) -> bool {
        // We wanna return if it failed or not
        if !self.guessable_word_list.contains(&guess) {
            return false;
        }

        self.guessed_words.push(guess.clone());
        self.last_guessed_word = guess.clone();
        self.round_number += 1;

        // true means it worked
        true
    }

    pub fn print_game(&mut self) {
        // Print colorized word with chars split by spaces
        for i in 0..self.guessed_words.len() {
            let word = self.guessed_words[i].clone();
            let chars: Vec<String> = self.colorize_guess(word);

            println!("{}", chars.join(" "));
        }

        // Depending on what round we on, print underscores
        let n = 6 - self.round_number;
        for _ in 0..n {
            println!("_ _ _ _ _");
        }
    }

    pub fn generate_word(&mut self) {
        let mut rng = rand::rng();
        let rand_idx = rng.random_range(0..self.chooseable_word_list.len());
        
        self.word = self.chooseable_word_list[rand_idx].to_string()
    }
    
    pub fn reset_game(&mut self) {
        self.guessed_words.clear();
        self.last_guessed_word = "".to_string();
        self.round_number = 0;
    }

    // tbh ion rlly understand this
    fn colorize_guess(&mut self, guess: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut word_chars: Vec<char> = self.word.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();
        
        for (i, &g_char) in guess_chars.iter().enumerate() {
            if i < word_chars.len() && g_char == word_chars[i] {
                // Correct position so we highlight green
                result.push((&g_char.to_ascii_uppercase().to_string().green()).to_string()); // Only call to_string() once
                word_chars[i] = ' '; // Mark as used
            } else if word_chars.contains(&g_char) {
                // Incorrect position so highlight yellow
                result.push((&g_char.to_ascii_uppercase().to_string().yellow()).to_string()); // Only call to_string() once
                let index = word_chars.iter().position(|&c| c == g_char).unwrap();
                word_chars[index] = ' '; // Mark as used
            } else {
                // Not in word so highlight gray
                result.push((&g_char.to_ascii_uppercase().to_string().bright_black()).to_string());
                // Only call to_string() once
            }
        }

        result
    }
}
