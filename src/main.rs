mod wordle;

use std::io;
use std::io::Write;
use wordle::Wordle;
use std::process::Command;

fn main() {
    let mut wordle = Wordle::new();
    let mut input = String::new();

    loop {
        print!("Start new game (y/n): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_ascii_lowercase();

        let first_char = input.chars().next().unwrap_or_default();

        // Handle anything that isnt yes
        if first_char != 'y' && first_char != 'n' {
            println!("Please answer yes or no");
            input.clear();
            continue;
        } else if first_char == 'n' {
            // End game
            println!("Alrighty");
            break;
        }
        
        // clear here so that if they start a new game
        // it doesnt break
        input.clear();
        wordle.generate_word();
        
        loop {
            // clear_console();
            wordle.print_game();

            if wordle.has_won() {
                println!("\nYou win!\n");
                break;
            }
            
            let mut guess_input = String::new();
            
            println!();
            print!("Guess a word: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut guess_input)
                .expect("Failed to read line");
            
            guess_input = guess_input.trim().to_ascii_lowercase();
            
            if guess_input.len() != 5 {
                // clear_console();
                println!("Word must be 5 letters long");
                println!();
                guess_input.clear();
                continue;
            }

            if !wordle.make_guess(&guess_input) {
                // clear_console();
                
                println!("Word not in list");
                println!();
                guess_input.clear();
                continue;
            }
        }
    }

    // Wait for user input before closing
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input).unwrap();
}

// a little robot birdy told me this works better
fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console on Windows");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear console on Unix-based systems");
    }
}
