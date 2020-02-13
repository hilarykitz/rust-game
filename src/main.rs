use std::io;

mod game;
mod parser;

fn main() {
    let mut scene = game::Scene::new();

    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            dbg!(error);
        } else {
            match game::Instruction::from_str(instruction) {
                Err(error) => println!("{}\n", error),
                Ok(instruction) => println!("{}\n", scene.do_instruction(instruction)),
            }
        }
    }
}
