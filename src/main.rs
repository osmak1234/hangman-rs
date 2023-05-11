use crate::{hangman_art::get_art, words::choose_word};
use std::io;
use std::vec::Vec;

mod hangman_art;
mod words;

fn main() {
    let mut guess = String::new();
    let mut right_guess = 0;
    let mut guessed_right = false;
    let mut lost = false;
    let the_word = choose_word();
    let mut hanging_state: i32 = 0;
    let mut guessed_letters: Vec<char> = vec!['_'; the_word.len()];
    let word_string: String = the_word.clone().into_iter().collect();

    let mut already_guessed: Vec<char> = Vec::new();
    let mut guessed_character_vec: Vec<char> = Vec::new();
    let mut guessed_character: char;

    while !lost {
        draw(hanging_state, guessed_letters.clone().into_iter().collect());
        guess = "".to_string();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line, try again");
        guessed_character_vec = guess.clone().chars().collect();
        guessed_character = guessed_character_vec[0];

        already_guessed.push(guessed_character);
        for i in 0..the_word.len() {
            if guessed_character == the_word[i] {
                guessed_letters[i] = the_word[i];
                guessed_right = true;
                right_guess += 1;
            }
        }

        if !guessed_right {
            hanging_state += 1;
        } else {
            guessed_right = false;
        }

        if word_string.len() == right_guess {
            draw_win(guessed_letters.clone().into_iter().collect());
            lost = true;
        }

        if hanging_state == 6 {
            draw(hanging_state, "".to_string());
            println!("The word was {}", word_string);
            lost = true;
        }
    }
}

fn draw(state: i32, text: String, already_guessed: Vec<char>) {
    std::process::Command::new("clear").status().unwrap();
    if state != 6 {
        println!("{}", text);
    }
    println!("{}", get_art(state as usize));
    if state != 7 && state != 6 {
        println!(
            "You already_guessed: {}",
            already_guessed.into_iter().collect()
        );
        println!("Enter just one character to guess")
    }
}

fn draw_win(text: String) {
    std::process::Command::new("clear").status().unwrap();
    println!("{}", text);
    draw(7, text);
    println!(
        "

db    db  .d88b.  db    db      db   d8b   db  .d88b.  d8b   db
`8b  d8' .8P  Y8. 88    88      88   I8I   88 .8P  Y8. 888o  88
 `8bd8'  88    88 88    88      88   I8I   88 88    88 88V8o 88
   88    88    88 88    88      Y8   I8I   88 88    88 88 V8o88
   88    `8b  d8' 88b  d88      `8b d8'8b d8' `8b  d8' 88  V888
   YP     `Y88P'  ~Y8888P'       `8b8' `8d8'   `Y88P'  VP   V8P

                                                                "
    )
}
