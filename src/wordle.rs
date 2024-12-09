use std::fs;
use colored::Colorize;
use rand::Rng;

struct Wordle {
    word: String,
    seen_words: Vec<String>,
    word_list: Vec<String>,
    round_number: u8
}

impl Wordle {
    pub fn new() -> Wordle {
        let word_list_as_string = fs::read_to_string("wordle.txt").expect("Failed to read file");
        let word_list: Vec<String> = serde_json::from_str(&word_list_as_string).expect("Failed to parse JSON");
        
        Self {
            word: "".to_string(),
            seen_words: vec![],
            word_list,
            round_number: 1
        }
    }
    
    pub fn print_game(&self) {
        
    }
    
    fn generate_word(&mut self) -> String {
        let mut rng = rand::rng();
        let rand_idx = rng.random_range(0..self.word_list.len());
        self.word_list[rand_idx].to_string()
    }
}