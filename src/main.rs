mod data_access;

use std::{io, io::Write};

use data_access::get_text_data;

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

fn recall_name_scene() {
    print!(
        "{}",
        get_text_data("game", "recall_name_intro").expect("Error reading text data!")
    );

    let player_name = get_user_input_string().unwrap();

    println!(
        "{}. {}",
        player_name,
        get_text_data("game", "recall_name_body").expect("Error reading text data!")
    );
}

fn red_door_scene() {}

fn examine_red_door() {
    print!(
        "{}",
        get_text_data("game", "examine_red_door_intro").expect("Error reading text data!")
    );

    print!(
        "{}",
        get_text_data("game", "examine_red_door_body").expect("Error reading text data!")
    );

    let user_input = get_user_input_number().unwrap();

    match user_input {
        1 => red_door_scene(),
        _ => {
            println!("You go way from the red door and continue to explore the room");
            return;
        }
    }
}

fn examine_yellow_door() {}

fn examine_brown_door() {}

fn explore_room_scene() {
    print!(
        "{}",
        get_text_data("game", "explore_room_intro").expect("Error reading text data!")
    );

    loop {
        print!(
            "{}",
            get_text_data("game", "explore_room_body").expect("Error reading text data!")
        );

        let user_input = get_user_input_number().expect("Error getting user input!");

        match user_input {
            1 => examine_red_door(),
            2 => examine_yellow_door(),

            3 => examine_brown_door(),

            _ => continue,
        }
    }
}

fn start_new_game() {
    println!("Starting new game!\n");

    recall_name_scene();

    explore_room_scene();
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

fn main_menu_scene() {
    print!(
        "{}",
        get_text_data("game_menu", "main_menu").expect("Error reading data file!")
    );

    let user_input = get_user_input_number().expect("Error getting user input!");

    match user_input {
        1 => start_new_game(),
        2 => show_settings(),
        3 => show_tutorial(),
        4 => exit_game(),
        _ => println!("Wrong option"),
    }
}

fn main() {
    main_menu_scene();
}
