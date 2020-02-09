mod parser {
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
    pub enum Instruction {
        Look,
        Describe(Option<EntityIdent>),
        Consume(Option<EntityIdent>),
    }

    pub fn parse_instruction(instruction: String) -> Result<Instruction, &'static str> {
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
}

pub mod game {
    use super::parser;

    trait Entity {
        fn describe(&self) -> String;

        fn consume(&self) -> Result<String, &str> {
            Err("It's not food")
        }

        fn read(&self) -> Result<String, &str> {
            Err("There's nothing to read")
        }
    }

    struct Apple {
        eaten: bool
    }

    impl Apple {
        fn new() -> Apple {
            Apple {
                eaten: false
            }
        }
    }

    impl Entity for Apple {
        fn describe(&self) -> String {
            if !self.eaten {
                String::from("It's a perfect red apple")
            } else {
                String::from("It's an apple core")
            }
        }

        fn consume(&self) -> Result<String, &str> {
            if !self.eaten {
                Ok(String::from("It's delicious! All that's left is the core"))
            } else {
                Err("The core doesn't look appetising")
            }
        }
    }

    struct Book {
        title: String,
        author: String,
        contents: String,
    }

    impl Book {
        fn new(title: String, author: String, contents: String) -> Book {
            Book {
                title,
                author,
                contents,
            }
        }
    }

    impl Entity for Book {
        fn describe(&self) -> String {
            format!("It's a book. The title reads \"{}\" by {}", &self.title, &self.author)
        }

        fn read(&self) -> Result<String, &str> {
            Ok(format!("The book reads:\n{}", &self.contents))
        }
    }

    pub struct Scene {
        entities: Vec<Box<dyn Entity>>
    }

    impl Scene {
        pub fn new() -> Scene {
            Scene {
                entities: vec![
                    Box::new(Apple::new()),
                    Box::new(Book::new(String::from("The Lusty Argonian Maid"), String::from("Crassius Curio"), String::from("[contents here]"))),
                ]
            }
        }

        pub fn do_instruction(&mut self, instruction: String) {
            match parser::parse_instruction(instruction) {
                Ok(instruction) => {
                    let response = match instruction {
                        parser::Instruction::Look => "You look around and see a book and a sandwich",
                        parser::Instruction::Describe(Some(entity)) => match entity {
                            parser::EntityIdent::Book => "It's a book",
                            parser::EntityIdent::Sandwich => "It's a sandwich",
                        },
                        parser::Instruction::Describe(None) => "You can't see it",
                        parser::Instruction::Consume(Some(entity)) => match entity {
                            parser::EntityIdent::Sandwich => "The sandwich tastes great and you eat the whole thing",
                            parser::EntityIdent::Book => "The book tastes like sweeties and you absorb the knowledge within",
                        },
                        parser::Instruction::Consume(None) => "You can't find that"
                    };
                    println!("{}\n", response);
                },
                Err(error) => {
                    println!("{}\n", error);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parser::*;

    #[test]
    fn it_parses_bad_input() {
        assert_eq!(parse_instruction(String::from("")), Err("I don't understand"));
        assert_eq!(parse_instruction(String::from("dance")), Err("I don't understand"));
    }

    #[test]
    fn it_parses_look() {
        let instruction = parse_instruction(String::from("look")).unwrap();
        assert_eq!(instruction, Instruction::Look);
    }

    #[test]
    fn it_parses_look_at() {
        let instruction = parse_instruction(String::from("look at"));
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = parse_instruction(String::from("look at book")).unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("book")));

        let instruction = parse_instruction(String::from("look at sandwich")).unwrap();
        assert_eq!(instruction, Instruction::Describe(EntityIdent::from_str("sandwich")));

        let instruction = parse_instruction(String::from("look at dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Describe(None));
    }

    #[test]
    fn it_parses_eat() {
        let instruction = parse_instruction(String::from("eat"));
        assert_eq!(instruction, Err("I don't understand"));

        let instruction = parse_instruction(String::from("eat book")).unwrap();
        assert_eq!(instruction, Instruction::Consume(EntityIdent::from_str("book")));

        let instruction = parse_instruction(String::from("eat sandwich")).unwrap();
        assert_eq!(instruction, Instruction::Consume(EntityIdent::from_str("sandwich")));

        let instruction = parse_instruction(String::from("eat dolphin")).unwrap();
        assert_eq!(instruction, Instruction::Consume(None));
    }
}
