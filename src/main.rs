use std::io;

fn main() {
    game();
}

fn game() {
    let mut game_on = true;

    while game_on {
        println!("Welcome to rust_fight!");
        println!("Type [e]xit to quit");

        let mut command = String::from("");

        io::stdin().read_line(&mut command)
            .expect("Somthing went wrong");
        
        println!("{}", command);

        if command.trim() == "e" || command.trim() == "exit" {
            game_on = false;
        }
    }
}