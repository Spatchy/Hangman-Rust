use std::io;
use rand::Rng;

fn main() {
    print_main_menu();
    _ = read_input();

    let mut playing: bool = true;
    let mut has_won: bool = false;
    let mut has_lost: bool = false;

    let word = generate_word();
    println!("My secret word is {} letters long", word.len());

    let mut correct_letters: Vec::<char> = Vec::new();
    let mut wrong_letters: Vec::<char> = Vec::new();

    
    while playing {
        show_board(&word, &correct_letters, &wrong_letters);
        (correct_letters, wrong_letters) = process_guess(&word, &correct_letters, &wrong_letters);
        has_won = check_has_won(&word, &correct_letters);
        has_lost = check_has_lost(&wrong_letters);
        playing = !has_won || !has_lost;
    }
}

fn print_main_menu() {
    println!(
        "
        Welcome traveller to The Hangman's Gallows.
        If you wish to free yourself, you must attone
        for your crimes by guessing my secret word.
        If you guess a wrong letter 6 times, you'll be strung up
        and your corpse will become a feast for the crows.

        [PRESS ENTER TO PLAY]
        "
    );
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn read_file() -> String {
    let bytes = include_bytes!("./res/words.txt");
    let contents = String::from_utf8_lossy(bytes).to_string();

    contents
}

fn generate_word() -> String {
    let file = read_file();
    let lines = file
        .split("\n")
        .collect::<Vec<&str>>();
    let num = rand::thread_rng().gen_range(0..50);
    let word = lines[num];

    word.to_string()
}

fn show_board(word: &String, correct_letters: &Vec<char>, wrong_letters: &Vec<char>) {
    let mut guessed_string: String = "You have guessed: ".to_string();

    for letter in correct_letters {
        guessed_string.push(*letter);
        guessed_string.push_str(" ");
    };

    for letter in wrong_letters {
        guessed_string.push(*letter);
        guessed_string.push_str(" ");
    };

    let mut word_readout = String::new();

    for letter in word.chars() {
        if correct_letters.contains(&letter) {
            word_readout.push(letter);
        } else {
            word_readout.push_str("_ ");
        }
    };

    println!("{}", guessed_string);
    println!("{}", word_readout);
}

fn process_guess(word: &String, correct_letters: &Vec<char>, wrong_letters: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut new_correct_letters = correct_letters.to_vec();
    let mut new_wrong_letters = wrong_letters.to_vec();

    println!("Pick a letter, may the luck of the moon be with you: ");
    let guess = read_input();
    let guess_char = guess.chars().nth(0).unwrap();

    if word.contains(&guess_char.to_string()) {
        new_correct_letters.push(guess_char);
    } else {
        new_wrong_letters.push(guess_char);
    }

    (new_correct_letters, new_wrong_letters)
}

fn check_has_won(word: String, correct_letters: &Vec<char>) -> (bool) {
    let mut still_to_find: bool = false;

    for letter in word.Chars {
        if !correct_letters.contains(letter) {
            still_to_find = true;
            break;
        }
    }

    return !still_to_find;
}

fn check_has_lost(wrong_letters: &Vec<char>) -> (bool) {
    return wrong_letters.Length >= 7;
}