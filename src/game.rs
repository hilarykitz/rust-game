#[derive(Debug,PartialEq)]
pub enum EntityIdent {
    NullEntity(String),
    Apple,
    Book,
}

#[derive(Debug,PartialEq)]
pub enum Instruction {
    Look,
    Describe(EntityIdent),
    Consume(EntityIdent),
    Read(EntityIdent),
}

trait Entity {
    fn describe(&self) -> String;

    fn consume(&self) -> Result<String, &str> {
        Err("It's not food.")
    }

    fn read(&self) -> Result<String, &str> {
        Err("There's nothing to read.")
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
            String::from("It's a tempting red apple.")
        } else {
            String::from("It's an apple core.")
        }
    }

    fn consume(&self) -> Result<String, &str> {
        if !self.eaten {
            Ok(String::from("It's delicious! All that's left is the core."))
        } else {
            Err("The core doesn't look appetising.")
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
        format!("It's a book. The title reads \"{}\" by {}.", &self.title, &self.author)
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

    fn find_entity(&self, ident: EntityIdent) -> Result<&Box<dyn Entity>, String> {
        match ident {
            EntityIdent::NullEntity(ident) => Err(ident),
            EntityIdent::Apple => Ok(&self.entities[0]),
            EntityIdent::Book => Ok(&self.entities[1]),
        }
    }

    pub fn do_instruction(&self, instruction: Instruction) -> String {
        match instruction {
            Instruction::Look => String::from("You look around and see an apple and a book."),
            Instruction::Describe(ident) => {
                match self.find_entity(ident) {
                    Ok(entity) => entity.describe(),
                    Err(ident) => format!("You can't find a {}", ident),
                }
            },
            Instruction::Consume(ident) => {
                match self.find_entity(ident) {
                    Ok(entity) => {
                        let result = entity.consume();
                        match result {
                            Ok(response) => response,
                            Err(error) => format!("{}, so you decide not to eat it.", error),
                        }
                    },
                    Err(ident) => format!("You can't find a {}.", ident),
                }
            },
            Instruction::Read(ident) => {
                match self.find_entity(ident) {
                    Ok(entity) => {
                        let result = entity.read();
                        match result {
                            Ok(response) => response,
                            Err(error) => format!("{}", error),
                        }
                    },
                    Err(ident) => format!("You can't find a {}.", ident),
                }
            },
        }
    }
}
