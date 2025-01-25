use std::io;

fn main() {
    print_main_menu();
    _ = read_input();
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