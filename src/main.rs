use std::io;

struct Character {
    name: String,
    health: i32,
    alive: bool,
}

fn main() {
    game();
}

fn game() {
    let mut game_on = true;
    let mut character = Character {
        name: String::from(""),
        health: 100,
        alive: false,
    };
    let mut characters_created = 0;

    while game_on {
        println!("{}", characters_created);
        println!("Welcome to rust_fight!");
        println!("Type [e]xit to quit");

        let mut command = String::from("");

        if character.alive == false {
            println!("You have no character, [c]reate to create a character");
        } else {
            println!("[f]ight to fight!");
        }

        io::stdin()
            .read_line(&mut command)
            .expect("Somthing went wrong");
        println!("{}", command);

        if command.trim() == "e" || command.trim() == "exit" {
            game_on = false;
        } else if command.trim() == "c" || command.trim() == "create" {
            character = create_character(&mut characters_created);
        } else if command.trim() == "f" || command.trim() == "fight" {
            fight(&character);
        }
    }
}

fn create_character(characters_created: &mut i32) -> Character {
    println!("Please enter the name of your character");

    let mut name = String::from("");
    io::stdin()
        .read_line(&mut name)
        .expect("Something went wrong");

    let character = Character {
        name: name,
        health: 100,
        alive: true,
    };

    *characters_created += 1;

    character
}

fn fight(player: &Character) {
    let mut fight_on = true;

    println!("Welcome to the areana {}!", player.name);

    while fight_on {}
}
