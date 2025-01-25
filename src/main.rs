use std::io;
use rand::Rng;

fn main() {
    print_main_menu();
    _ = read_input();
    let word = generate_word();
    println!("The word is {} letters long", word.len());
    println!("The word is {}", word); // For testing
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