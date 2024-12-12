mod wordle;

use std::io;
use std::io::Write;
use wordle::Wordle;

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
        
        // a little birdy told me this clears the console
        print!("\x1B[2J\x1B[1;1H");

        loop {
            if wordle.has_won() {
                println!("You win!");
                break;
            }

            wordle.generate_word();
            let mut guess_input = String::new();

            wordle.print_game();
            println!();
            print!("Guess a word: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut guess_input)
                .expect("Failed to read line");
            guess_input = guess_input.trim().to_ascii_lowercase();
            
            if guess_input.len() != 5 {
                print!("\x1B[2J\x1B[1;1H");
                println!("Word must be 5 letters long");
                println!();
                guess_input.clear();
                continue;
            }

            if !wordle.make_guess(&guess_input) {
                print!("\x1B[2J\x1B[1;1H");
                println!("Word not in list");
                println!();
                guess_input.clear();
                continue;
            }
            
            print!("\x1B[2J\x1B[1;1H");
        }
    }

    // Wait for user input before closing
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input).unwrap();
}
