use crate::game::{EntityIdent, Instruction};

const PARSE_ERROR: Result<Instruction, &str> = Err("I don't understand");

impl EntityIdent {
    fn from_str(string: &str) -> Option<EntityIdent> {
        match string {
            "apple" => Some(EntityIdent::Apple),
            "book" => Some(EntityIdent::Book),
            _ => None,
        }
    }
}

impl Instruction {
    pub fn from_str(instruction: String) -> Result<Instruction, &'static str> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 {
            match tokens[0] {
                "look" => parse_look(&tokens[1..]),
                "eat" => parse_eat(&tokens[1..]),
                "read" => parse_read(&tokens[1..]),
                _ => PARSE_ERROR
            }
        } else {
            PARSE_ERROR
        }
    }

}

fn parse_look(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        0 => {
            Ok(Instruction::Look)
        },
        2 if tokens[0] == "at" => {
            Ok(Instruction::Describe(EntityIdent::from_str(tokens[1])))
        },
        3 if tokens[0] == "at" && tokens[1] == "the" => {
            Ok(Instruction::Describe(EntityIdent::from_str(tokens[2])))
        },
        _ => PARSE_ERROR
    }
}

fn parse_eat(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => {
            Ok(Instruction::Consume(EntityIdent::from_str(tokens[0])))
        },
        2 if tokens[0] == "the" => {
            Ok(Instruction::Consume(EntityIdent::from_str(tokens[1])))
        },
        _ => PARSE_ERROR
    }
}

fn parse_read(tokens: &[&str]) -> Result<Instruction, &'static str> {
    match tokens.len() {
        1 => {
            Ok(Instruction::Read(EntityIdent::from_str(tokens[0])))
        },
        2 if tokens[0] == "the" => {
            Ok(Instruction::Read(EntityIdent::from_str(tokens[1])))
        },
        _ => PARSE_ERROR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_bad_input() {
        assert_eq!(Instruction::from_str(String::from("")), Err("I don't understand"));
        assert_eq!(Instruction::from_str(String::from("dance")), Err("I don't understand"));
    }

    #[test]
    fn it_parses_look() {
        let instruction = Instruction::from_str(String::from("look")).unwrap();
        assert_eq!(instruction, Instruction::Look);
    }

    #[test]
    fn it_parses_look_at() {
        let instruction = Instruction::from_str(String::from("look at"));
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = Instruction::from_str(String::from("look at book")).unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("book")));

        let instruction = Instruction::from_str(String::from("look at dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Describe(None));

        let instruction = Instruction::from_str(String::from("look at the book")).unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("book")));

        let instruction = Instruction::from_str(String::from("look at the dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Describe(None));
    }

    #[test]
    fn it_parses_eat() {
        let instruction = Instruction::from_str(String::from("eat"));
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = Instruction::from_str(String::from("eat book")).unwrap();
        assert_eq!(instruction, Instruction::Consume(EntityIdent::from_str("book")));

        let instruction = Instruction::from_str(String::from("eat dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Consume(None));
    }

    #[test]
    fn it_parses_read() {
        let instruction = Instruction::from_str(String::from("read"));
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = Instruction::from_str(String::from("read book")).unwrap();
        assert_eq!(instruction, Instruction::Read(EntityIdent::from_str("book")));

        let instruction = Instruction::from_str(String::from("read dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Read(None));

        let instruction = Instruction::from_str(String::from("read the book")).unwrap();
        assert_eq!(instruction, Instruction::Read(EntityIdent::from_str("book")));

        let instruction = Instruction::from_str(String::from("read the dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Read(None));
    }
}
