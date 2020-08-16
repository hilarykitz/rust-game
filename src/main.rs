use std::convert::TryFrom;
use std::io;

mod parser;
mod scene;

use parser::{EntityIdent, Instruction};
use scene::{Entity, Scene};

fn find_entity(scene: &mut Scene, ident: EntityIdent) -> Option<&mut Entity> {
    scene
        .entities
        .iter_mut()
        .find(|entity| match (&ident, entity) {
            (EntityIdent::Apple, Entity::Apple(_)) => true,
            (EntityIdent::Apple, _) => false,
            (EntityIdent::Book, Entity::Book(_)) => true,
            (EntityIdent::Book, _) => false,
            (EntityIdent::Wrench, Entity::Wrench(_)) => true,
            (EntityIdent::Wrench, _) => false,
        })
}

fn do_instruction(scene: &mut Scene, instruction: Instruction) -> String {
    match instruction {
        Instruction::Exit => panic!("Can't do exit instruction on scene"),
        Instruction::Look => String::from("You look around and see an apple and a book."),
        Instruction::Describe(ident, token) => match ident {
            None => format!("You've never heard of a {}.", token),
            Some(ident) => match find_entity(scene, ident) {
                None => format!("You look around but you can't find a {} here.", token),
                Some(entity) => match entity {
                    Entity::Apple(entity) => entity.describe(),
                    Entity::Book(entity) => entity.describe(),
                    Entity::Wrench(entity) => entity.describe(),
                },
            },
        },
        Instruction::Consume(ident, token) => match ident {
            None => format!("You've never heard of a {}.", token),
            Some(ident) => match find_entity(scene, ident) {
                None => format!("You look around but you can't find a {} here.", token),
                Some(entity) => {
                    let result = match entity {
                        Entity::Apple(apple) => apple.consume(),
                        Entity::Book(_) => Err("It's not food."),
                        Entity::Wrench(_) => Err("You bite down hard. Ouch!"),
                    };
                    match result {
                        Err(error) => format!("{} You decide not to eat it.", error),
                        Ok(response) => response,
                    }
                }
            },
        },
        Instruction::Read(ident, token) => match ident {
            None => format!("You've never heard of a {}.", token),
            Some(ident) => match find_entity(scene, ident) {
                None => format!("You look around but can't find a {} here.", token),
                Some(entity) => {
                    let result = match entity {
                        Entity::Book(book) => book.read(),
                        Entity::Apple(_) => Err("There's nothing to read."),
                        Entity::Wrench(_) => Err("There's nothing to read."),
                    };
                    match result {
                        Err(error) => format!("{} You leave it alone.", error),
                        Ok(response) => response,
                    }
                }
            },
        },
    }
}

fn main() {
    let mut scene = scene::Scene::new(vec![
        Entity::Apple(scene::Apple::new()),
        Entity::Book(scene::Book::new(
            String::from("The Lusty Argonian Maid"),
            String::from("Crassius Curio"),
            String::from("[contents here]"),
        )),
    ]);

    loop {
        let mut instruction = String::new();

        if let Err(error) = io::stdin().read_line(&mut instruction) {
            dbg!(error);
        } else {
            match parser::Instruction::try_from(instruction) {
                Err(error) => println!("{}\n", error),
                Ok(instruction) => {
                    if instruction == parser::Instruction::Exit {
                        std::process::exit(0);
                    }
                    println!("{}\n", do_instruction(&mut scene, instruction));
                }
            }
        }
    }
}
