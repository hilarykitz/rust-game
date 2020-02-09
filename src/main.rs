use std::io;
use rust_game::game;

fn main() {
    let mut scene = game::Scene::new();

    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            dbg!(error);
        } else {
            scene.do_instruction(instruction);
        }
    }
}
