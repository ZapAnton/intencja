use std::{io, io::Write};

#[derive(Debug)]
struct Player {
    name: String,
}

fn get_user_input_number() -> Result<u32, io::Error> {
    let user_input = get_user_input_string().expect("Error getting user input!");

    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse input. You must enter a number!");

    Ok(user_input)
}

fn get_user_input_string() -> Result<String, io::Error> {
    print!("\n> ");

    io::stdout().flush().unwrap();

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)?;

    let user_input = user_input.trim().to_string();

    Ok(user_input)
}

fn show_welcome_message() {
    println!("Welcome to Intencja Game!");

    println!(
        "Press 1 to start a new game.\n\
         Press 2 to enter game settings.\n\
         Press 3 to read game tutorial\n\
         Press 4 to exit game."
    );
}

fn start_new_game() {
    println!("Starting new game!\n");

    println!(
        "You find yourself in a strage, unfamiliar place. \n\
         You don't remember how you got here. You don't\n\
         even remember who you are. You are trying hard to\n\
         recall something, anything and after a moment a name\n\
         starts emerging in your mind.\n\
         [ENTER YOU NAME]"
    );

    let player_name = get_user_input_string().unwrap();

    let player = Player { name: player_name };

    println!(
        "\n{}. Is this your real name? Is it even a name?\n\
         This name feels strange - you feel no connection\n\
         with it, as if it does not really belongs to you.\n\
         But for now you let it be.",
        player.name
    );
}

fn show_settings() {
    println!("Game settings:");
}

fn show_tutorial() {
    println!("How to play Intencja:");
}

fn exit_game() {
    println!("Exiting game!");
}

fn process_option(chosen_option: u32) {
    match chosen_option {
        1 => start_new_game(),
        2 => show_settings(),
        3 => show_tutorial(),
        4 => exit_game(),
        _ => println!("Wrong option"),
    }
}

fn main() {
    show_welcome_message();

    let chosen_option = get_user_input_number().expect("Error getting user input!");

    process_option(chosen_option);
}
