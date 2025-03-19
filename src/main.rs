use std::io;
use std::io::Write;
use rand::Rng;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;
use termion::clear;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Welcome to Hangman");
    sleep(Duration::from_secs(2));
    // word bank for words to guess
    // Load words from file

        // Fallback to default list if error
        let words = vec![
            "apple", "banana", "grape", "orange", "peach", "cherry", "kiwi", "melon", "plum", "pear",
            "avocado", "mango", "pineapple", "watermelon", "blueberry", "strawberry", "apricot", "blackberry",
            "cantaloupe", "fig", "papaya", "lemon", "lime", "nectarine", "tangerine", "apricot", "applause",
            "game", "laptop", "keyboard", "mouse", "monitor", "printer", "screen", "tablet", "smartphone", "charger",
            "camera", "lens", "memory", "storage", "router", "modem", "cable", "microwave", "refrigerator",
            "stove", "oven", "toaster", "dishwasher", "sink", "faucet", "window", "door", "wall", "floor", "ceiling",
            "bed", "sofa", "chair", "table", "lamp", "couch", "bookshelf", "dresser", "mirror", "painting",
            "piano", "guitar", "drums", "flute", "violin", "clarinet", "trumpet", "tuba", "saxophone", "accordion",
            "actor", "director", "writer", "producer", "script", "stage", "theater", "cinema", "movie", "show",
            "star", "award", "scene", "plot", "dialogue", "audience", "rehearsal", "director", "film", "studio",
            "scientist", "chemist", "biologist", "astronomer", "engineer", "mathematician", "physicist", "doctor",
            "nurse", "teacher", "student", "principal", "classroom", "school", "university", "research", "experiment",
            "theory", "hypothesis", "biology", "chemistry", "physics", "math", "geometry", "algebra", "calculus"
        ];

    loop {
        // Randomly sets the word to guess
        let secret_word = select_random_word(&words);
        // println!("{}", secret_word);

        // set up game state
        let mut guessed_letters: HashSet<char> = HashSet::new();
        let mut incorrect_guesses = 0;
        const MAX_INCORRECT: u8 = 6;

        // Main game loop
        loop {
            // Clears screen
            clear_terminal();

            // Display current state of the Game
            display_hangman(incorrect_guesses);
            display_word_progress(&secret_word, &guessed_letters);
            display_letters_guessed(&guessed_letters);

            // Check if player has won
            if is_word_guessed(&secret_word, &guessed_letters) {
                println!("\nCongratulations! You've guessed the word.");
                break;
            }

            //checks if player has lost
            if incorrect_guesses >= MAX_INCORRECT {
                println!("\nGame Over! The word was {}", secret_word);
                break;
            }
            // gets the player guess
            let guess = get_player_guess(&guessed_letters);
            guessed_letters.insert(guess);

            // checks if players guess is correct
            if !secret_word.contains(guess) {
                incorrect_guesses += 1;
            }

        }
        if !play_again() {
            println!("\nThanks for playing!");
            break;
        }
    }
}

fn select_random_word(words: &Vec<&str>) -> String {
    let random_index = rand::rng().random_range(0..words.len());
    words[random_index].to_string()
}

fn display_hangman(incorrect_guesses: u8) {
match incorrect_guesses {
    0 => {
        println!("
       +------+
       |      |
       |      O
       |
       |
  0    |
 /|\\  ============
 / \\  |          |");
    },
    1 => {
        println!("
       +------+
       |      |
       |      0
       |
       |
       |
      ============
      |          |")
    },

    2 => {
        println!("
       +------+
       |      |
       |      0
       |      |
       |
       |
      ============
      |          |")
    },

    3 => {
        println!("
       +------+
       |      |
       |      0
       |     /|
       |
       |
      ============
      |          |")
    },
    4 => {
        println!("
       +------+
       |      |
       |      0
       |     /|\\
       |
       |
      ============
      |          |")
    },
    5 => {
        println!("

       +------+
       |      |
       |      0
       |     /|\\
       |     /
       |
      ============
      |          |")
    },
    _ => {
        println!("

       +------+
       |      |
       |      0
       |     /|\\
       |     / \\
       |
      ============
      |          |")
    }

}
}
fn display_word_progress(word: &str, guessed_letters: &HashSet<char>) {
    print!("Word: ");
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            print!("{} ", c);
        } else {
            print!("_ ")
        }
    }
   println!();
}

fn display_letters_guessed(guessed_letters: &HashSet<char>) {
println!("Guessed letters: {:?}", guessed_letters);
}

fn is_word_guessed(word: &str, guessed_letters: &HashSet<char>) -> bool {
    word.chars().all(|c| guessed_letters.contains(&c))
}

fn get_player_guess(guessed_letters: &HashSet<char>) -> char {
    loop {
        println!("\nEnter your guess (a-z): ");
        io::stdout().flush().unwrap();

        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Some(c) = input.trim().chars().next() {
            if c.is_ascii_lowercase() {
                if guessed_letters.contains(&c) {
                    println!("You have already guessed this letter");
                } else {
                    return c;
                }
            } else {
                println!("Please enter a lowercase letter (a-z).");
            }
        } else {
            println!("Please enter a single letter.");
        }
    }
}

fn clear_terminal() {
    print!("{}", clear::All);
    io::stdout().flush().unwrap();
}

fn play_again() -> bool {
    println!("Play again (y/n): ");
    loop {
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if let Some(ch) = again.trim().chars().next() {
            if ch == 'y' {
                return true;
            }
            if ch == 'n' {
                return false;
            } else {
                println!("Invalid Input");
            }
        }
    }
}


fn load_words_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {

    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    Ok(words)
}