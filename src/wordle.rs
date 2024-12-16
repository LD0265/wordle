use colored::Colorize;
use rand::Rng;
use std::fs;
use std::path::Path;

pub struct Wordle {
    word: String,
    last_guessed_word: String,
    guessed_words: Vec<String>,
    word_list: Vec<String>,
    round_number: u8
}

impl Wordle {
    pub fn new() -> Wordle {
        let file_path = Path::new("word-list.txt");
        if !file_path.exists() {
            panic!(
                "File not found at {:?}. Ensure word-list.txt is in same dir as exe.",
                file_path
            );
        }

        // Read the word list as one string
        let word_list_as_string = fs::read_to_string(file_path).expect("Failed to read file");

        // Because its a json array we can do this
        let word_list: Vec<String> =
            serde_json::from_str(&word_list_as_string).expect("Failed to parse JSON");

        Self {
            word: "debug".to_string(), // just incase
            last_guessed_word: "".to_string(),
            guessed_words: vec![],
            word_list,
            round_number: 0,
        }
    }

    pub fn has_won(&self) -> bool {
        self.word == self.last_guessed_word
    }

    pub fn make_guess(&mut self, guess: &String) -> bool {
        // We wanna return if it failed or not
        if !self.word_list.contains(&guess) {
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

    pub fn generate_word(&mut self) -> String {
        let mut rng = rand::rng();
        let rand_idx = rng.random_range(0..self.word_list.len());
        self.word_list[rand_idx].to_string()
    }

    // tbh ion rlly understand this
    fn colorize_guess(&mut self, guess: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut word_chars: Vec<char> = self.word.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();

        let mut correct_chars = 0;

        for (i, &g_char) in guess_chars.iter().enumerate() {
            if i < word_chars.len() && g_char == word_chars[i] {
                // Correct position so we highlight green
                result.push((&g_char.to_ascii_uppercase().to_string().green()).to_string()); // Only call to_string() once
                word_chars[i] = ' '; // Mark as used
                correct_chars += 1;
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
