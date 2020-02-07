use std::io;
use rust_game::game;

fn main() {
    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            println!("Something went wrong: {}", error);
        } else {
            match game::parse_instruction(&instruction[..]) {
                Ok(game::Instruction::Look) => println!("You look around and see a book"),
                Ok(game::Instruction::Eat(game::Entity::Book)) =>
                    println!("The book tastes like sweeties and you absorb the knowledge within"),
                Err(error) => println!("{}", error),
            }
        }
    }
}
