use std::io;

fn main() {
    game();
}

fn game() {
    let mut game_on = true;
    let mut character = String::from("");

    while game_on {
        println!("Welcome to rust_fight!");
        println!("Type [e]xit to quit");

        let mut command = String::from("");

        if character.trim() == "" {
            println!("You have no character, [c]reate to create a character");
        } else {
            println!("[f]ight to fight!");
        }

        io::stdin().read_line(&mut command)
            .expect("Somthing went wrong");
        
        println!("{}", command);

        if command.trim() == "e" || command.trim() == "exit" {
            game_on = false;
        } else if command.trim() == "c" || command.trim() == "create" {
            character = create_character();
        } else if command.trim() == "f" || command.trim() == "fight" {
            fight();
        }
    }
}

fn create_character() -> String {
    println!("Please enter the name of your character");

    let mut name = String::from("");
    io::stdin().read_line(&mut name)
        .expect("Something went wrong");

    name
}

fn fight() {

}