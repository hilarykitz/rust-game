pub mod game {
    pub enum Entity {
        Book,
        Sandwich,
    }

    pub enum Instruction {
        Look,
        Describe(Option<Entity>),
        Consume(Option<Entity>),
    }

    impl Entity {
        fn from(string: &str) -> Option<Entity> {
            match string {
                "book" => Some(Entity::Book),
                "sandwich" => Some(Entity::Sandwich),
                _ => None,
            }
        }
    }

    pub fn parse_instruction(instruction: &str) -> Result<Instruction, &str> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 {
            if tokens[0] == "look" {
                if tokens.len() == 1 {
                    return Ok(Instruction::Look);
                }
                if tokens.len() > 2 && tokens[1] == "at" {
                    return match Entity::from(tokens[2]) {
                        Some(entity) => Ok(Instruction::Describe(Some(entity))),
                        None => Ok(Instruction::Describe(None)),
                    }
                }
            } else if tokens[0] == "eat" {
                if tokens.len() > 1 {
                    return match Entity::from(tokens[1]) {
                        Some(entity) => Ok(Instruction::Consume(Some(entity))),
                        None => Ok(Instruction::Consume(None)),
                    }
                }

                return Ok(Instruction::Consume(None));
            }
        }

        Err("I don't understand")
    }

    pub fn get_instruction_response(instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Look => "You look around and see a book and a sandwich",
            Instruction::Describe(Some(Entity::Book)) => "It's a book",
            Instruction::Describe(Some(Entity::Sandwich)) => "It's a sandwich",
            Instruction::Describe(None) => "You can't see it",
            Instruction::Consume(Some(Entity::Sandwich)) => "The sandwich tastes great and you eat the whole thing",
            Instruction::Consume(Some(Entity::Book)) => "The book tastes like sweeties and you absorb the knowledge within",
            Instruction::Consume(None) => "You don't have anything to eat"
        }
    }
}
