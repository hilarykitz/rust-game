use std::convert::TryFrom;

const PARSE_ERROR: Result<Instruction, &str> = Err("You look confused.");

type EntityToken = String;

#[derive(Debug, PartialEq)]
pub enum EntityIdent {
    Apple,
    AppleCore,
    Book,
    Wrench,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Exit,
    Look,
    Describe(Option<EntityIdent>, EntityToken),
    Consume(Option<EntityIdent>, EntityToken),
    Read(Option<EntityIdent>, EntityToken),
}

impl TryFrom<&str> for EntityIdent {
    type Error = ();

    fn try_from(string: &str) -> Result<EntityIdent, Self::Error> {
        match string {
            "apple" => Ok(EntityIdent::Apple),
            "core" => Ok(EntityIdent::AppleCore),
            "book" => Ok(EntityIdent::Book),
            "wrench" => Ok(EntityIdent::Wrench),
            _ => Err(()),
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
        2 if tokens[0] == "at" => Ok(Instruction::Describe(
            EntityIdent::try_from(tokens[1]).ok(),
            String::from(tokens[1]),
        )),
        3 if tokens[0] == "at" && tokens[1] == "the" => Ok(Instruction::Describe(
            EntityIdent::try_from(tokens[2]).ok(),
            String::from(tokens[2]),
        )),
        _ => PARSE_ERROR,
    }
}

fn parse_eat(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => Ok(Instruction::Consume(
            EntityIdent::try_from(tokens[0]).ok(),
            String::from(tokens[0]),
        )),
        2 if tokens[0] == "the" => Ok(Instruction::Consume(
            EntityIdent::try_from(tokens[1]).ok(),
            String::from(tokens[1]),
        )),
        _ => PARSE_ERROR,
    }
}

fn parse_read(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => Ok(Instruction::Read(
            EntityIdent::try_from(tokens[0]).ok(),
            String::from(tokens[0]),
        )),
        2 if tokens[0] == "the" => Ok(Instruction::Read(
            EntityIdent::try_from(tokens[1]).ok(),
            String::from(tokens[1]),
        )),
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
            Instruction::Describe(Some(EntityIdent::Book), String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("look at dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(None, String::from("dolphin"))
        );

        let instruction = Instruction::try_from(String::from("look at the book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(Some(EntityIdent::Book), String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("look at the dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Describe(None, String::from("dolphin"))
        );
    }

    #[test]
    fn it_parses_eat() {
        let instruction = Instruction::try_from(String::from("eat"));
        assert_eq!(instruction, PARSE_ERROR);

        let instruction = Instruction::try_from(String::from("eat book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Consume(Some(EntityIdent::Book), String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("eat dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Consume(None, String::from("dolphin"))
        );
    }

    #[test]
    fn it_parses_read() {
        let instruction = Instruction::try_from(String::from("read"));
        assert_eq!(instruction, PARSE_ERROR);

        let instruction = Instruction::try_from(String::from("read book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(Some(EntityIdent::Book), String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("read dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(None, String::from("dolphin"))
        );

        let instruction = Instruction::try_from(String::from("read the book")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(Some(EntityIdent::Book), String::from("book"))
        );

        let instruction = Instruction::try_from(String::from("read the dolphin")).unwrap();
        assert_eq!(
            instruction,
            Instruction::Read(None, String::from("dolphin"))
        );
    }
}
