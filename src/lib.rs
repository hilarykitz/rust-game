pub mod game {
    #[derive(Debug,PartialEq)]
    pub enum EntityIdent {
        Book,
        Sandwich,
    }

    impl EntityIdent {
        pub fn from_str(string: &str) -> Option<EntityIdent> {
            match string {
                "book" => Some(EntityIdent::Book),
                "sandwich" => Some(EntityIdent::Sandwich),
                _ => None,
            }
        }
    }

    #[derive(Debug,PartialEq)]
    pub struct Entity {
        ident: EntityIdent,
    }

    impl Entity {
        pub fn from_str(string: &str) -> Option<Entity> {
            match EntityIdent::from_str(string) {
                Some(ident) => Some(Entity { ident }),
                None => None,
            }
        }
    }

    #[derive(Debug,PartialEq)]
    pub enum Instruction {
        Look,
        Describe(Option<EntityIdent>),
        Consume(Option<EntityIdent>),
    }

    pub fn parse_instruction(instruction: &str) -> Result<Instruction, &str> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 {
            if tokens[0] == "look" {
                if tokens.len() == 1 {
                    return Ok(Instruction::Look);
                }
                if tokens.len() == 3 && tokens[1] == "at" {
                    return Ok(Instruction::Describe(EntityIdent::from_str(tokens[2])));
                }
            } else if tokens[0] == "eat" {
                if tokens.len() > 1 {
                    return Ok(Instruction::Consume(EntityIdent::from_str(tokens[1])));
                }
            }
        }

        Err("I don't understand")
    }

    pub fn get_instruction_response(instruction: &Instruction) -> &str {
        match instruction {
            Instruction::Look => "You look around and see a book and a sandwich",
            Instruction::Describe(Some(entity)) => match entity {
                EntityIdent::Book => "It's a book",
                EntityIdent::Sandwich => "It's a sandwich",
            },
            Instruction::Describe(None) => "You can't see it",
            Instruction::Consume(Some(entity)) => match entity {
                EntityIdent::Sandwich => "The sandwich tastes great and you eat the whole thing",
                EntityIdent::Book => "The book tastes like sweeties and you absorb the knowledge within",
            },
            Instruction::Consume(None) => "You can't find that"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::game::*;

    #[test]
    fn it_parses_bad_input() {
        assert_eq!(parse_instruction(""), Err("I don't understand"));
        assert_eq!(parse_instruction("dance"), Err("I don't understand"));
    }

    #[test]
    fn it_parses_look() {
        let instruction = parse_instruction("look").unwrap();
        assert_eq!(instruction, Instruction::Look);
    }

    #[test]
    fn it_parses_look_at() {
        let instruction = parse_instruction("look at");
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = parse_instruction("look at book").unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("book")));

        let instruction = parse_instruction("look at sandwich").unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("sandwich")));

        let instruction = parse_instruction("look at dolphin").unwrap();
        assert_eq!(instruction, Instruction::Describe(None));
    }

    #[test]
    fn it_parses_eat() {
        let instruction = parse_instruction("eat");
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = parse_instruction("eat book").unwrap();
        assert_eq!(instruction, Instruction::Consume(EntityIdent::from_str("book")));

        let instruction = parse_instruction("eat sandwich").unwrap();
        assert_eq!(instruction, Instruction::Consume(EntityIdent::from_str("sandwich")));

        let instruction = parse_instruction("eat dolphin").unwrap();
        assert_eq!(instruction, Instruction::Consume(None));
    }
}
