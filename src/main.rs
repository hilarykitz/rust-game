use std::io;
use rust_game::game;

fn main() {
    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            dbg!(error);
        } else {
            match game::parse_instruction(&instruction[..]) {
                Ok(instruction) => {
                    let response = game::get_instruction_response(&instruction);
                    println!("{}\n", response);
                },
                Err(error) => {
                    dbg!(error);
                    ()
                }
            }
        }
    }
}
