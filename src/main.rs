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

fn recall_name_scene() {
    println!(
        "You find yourself in a strage, unfamiliar place.\n\
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

fn examine_red_door() {

}

fn examine_yellow_door() {

}

fn examine_brown_door() {

}

fn explore_room_scene() {
    println!(
        "\nYou look around. It appears that you are in some\n\
         kind of room. There is nothing in it - four bare \n\
         stone walls, stone floor. You look up and to your\n\
         surprise there is no ceiling - only perpetual darkness\n\
         of a night sky and a brigth moon, granting you it's light.\n\n\

         As you look around one more time you notice that some walls\n\
         now have doors on them (although you are quite sure that\n\
         there was nothing on them a second ago). You study the doors.
         "
    );

    loop {
        println!(
            "There are three door leading from the room:\n\
             an old wooden red door covered in moss, a glass door\n\
             with yellow wood frame and a brown door.\n\n\
             
             1. Examine the red old door.\n\
             2. Examine the yellow glass door.\n\
             3. Examine the brown door.\n\
             "
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
