use std::convert::TryFrom;

const PARSE_ERROR: Result<Instruction, &str> = Err("I don't understand");

type EntityToken = String;

#[derive(PartialEq)]
pub enum EntityIdent {
    NullEntity,
    Apple,
    Book,
}

#[derive(PartialEq)]
pub enum Instruction {
    Exit,
    Look,
    Describe(EntityIdent, EntityToken),
    Consume(EntityIdent, EntityToken),
    Read(EntityIdent, EntityToken),
}

impl From<&str> for EntityIdent {
    fn from(string: &str) -> EntityIdent {
        match string {
            "apple" => EntityIdent::Apple,
            "book" => EntityIdent::Book,
            _ => EntityIdent::NullEntity,
        }
    }
}

impl TryFrom<String> for Instruction {
    type Error = &'static str;

    fn try_from(instruction: String) -> Result<Instruction, Self::Error> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 {
            match tokens[0] {
                "exit" => Ok(Instruction::Exit),
                "look" => parse_look(&tokens[1..]),
                "eat" => parse_eat(&tokens[1..]),
                "read" => parse_read(&tokens[1..]),
                _ => PARSE_ERROR,
            }
        } else {
            PARSE_ERROR
        }
    }
}

fn parse_look(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        0 => Ok(Instruction::Look),
        2 if tokens[0] == "at" => Ok(Instruction::Describe(EntityIdent::from(tokens[1]), String::from(tokens[1]))),
        3 if tokens[0] == "at" && tokens[1] == "the" => {
            Ok(Instruction::Describe(EntityIdent::from(tokens[2]), String::from(tokens[2])))
        }
        _ => PARSE_ERROR,
    }
}

fn parse_eat(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => Ok(Instruction::Consume(EntityIdent::from(tokens[0]), String::from(tokens[0]))),
        2 if tokens[0] == "the" => Ok(Instruction::Consume(EntityIdent::from(tokens[1]), String::from(tokens[1]))),
        _ => PARSE_ERROR,
    }
}

fn parse_read(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => Ok(Instruction::Read(EntityIdent::from(tokens[0]), String::from(tokens[0]))),
        2 if tokens[0] == "the" => Ok(Instruction::Read(EntityIdent::from(tokens[1]), String::from(tokens[1]))),
        _ => PARSE_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_bad_input() {
        assert_eq!(Instruction::try_from(String::from("")), PARSE_ERROR);
        assert_eq!(Instruction::try_from(String::from("dance")), PARSE_ERROR);
    }

    #[test]
    fn it_parses_exit() {
        let instruction = Instruction::try_from(String::from("exit")).unwrap();
        assert_eq!(instruction, Instruction::Exit);
    }

    #[test]
    fn it_parses_look() {
        let instruction = Instruction::try_from(String::from("look")).unwrap();
        assert_eq!(instruction, Instruction::Look);
    }

    #[test]
    fn it_parses_look_at() {
        let instruction = Instruction::try_from(String::from("look at"));
        assert_eq!(instruction, PARSE_ERROR);

        let instruction = Instruction::try_from(String::from("look at book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(EntityIdent::Book, String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("look at dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(EntityIdent::NullEntity, String::from("dolphin"))
        );

        let instruction = Instruction::try_from(String::from("look at the book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(EntityIdent::Book, String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("look at the dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(EntityIdent::NullEntity, String::from("dolphin"))
        );
    }

    #[test]
    fn it_parses_eat() {
        let instruction = Instruction::try_from(String::from("eat"));
        assert_eq!(instruction, PARSE_ERROR);

        let instruction = Instruction::try_from(String::from("eat book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Consume(EntityIdent::Book, String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("eat dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Consume(EntityIdent::NullEntity, String::from("dolphin"))
        );
    }

    #[test]
    fn it_parses_read() {
        let instruction = Instruction::try_from(String::from("read"));
        assert_eq!(instruction, PARSE_ERROR);

        let instruction = Instruction::try_from(String::from("read book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(EntityIdent::Book, String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("read dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(EntityIdent::NullEntity, String::from("dolphin"))
        );

        let instruction = Instruction::try_from(String::from("read the book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(EntityIdent::Book, String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("read the dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(EntityIdent::NullEntity, String::from("dolphin"))
        );
    }
}
