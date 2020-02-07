pub mod game {
    # [derive(Debug)]
    pub enum Entity {
        Book,
    }

    # [derive(Debug)]
    pub enum Instruction {
        Look,
        Eat(Entity),
    }

    pub fn parse_instruction(instruction: &str) -> Result<Instruction, &str> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 {
            if tokens[0] == "look" {
                if tokens.len() > 1 {
                    return Err("I can't find that")
                }

                return Ok(Instruction::Look)
            } else if tokens[0] == "eat" {
                if tokens.len() > 1 {
                    if tokens[1] == "book" {
                        return Ok(Instruction::Eat(Entity::Book));
                    }
                    return Err("I can't find that")
                }

                return Err("You don't have anything to eat");
            }
        }

        Err("I don't understand")
    }
}
