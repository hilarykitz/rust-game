use std::io;
use rust_game::game;

fn main() {
    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            println!("Something went wrong: {}", error);
        } else {
            match game::parse_instruction(&instruction[..]) {
                Ok(instruction) => game::do_instruction(&instruction),
                Err(error) => println!("{}", error),
            }
        }
    }
}
