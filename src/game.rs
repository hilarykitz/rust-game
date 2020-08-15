#[derive(Debug, PartialEq)]
pub enum EntityIdent {
    NullEntity(String),
    Apple,
    Book,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Exit,
    Look,
    Describe(EntityIdent),
    Consume(EntityIdent),
    Read(EntityIdent),
}

enum Entity {
    Apple(Apple),
    Book(Book),
}

struct Apple {
    eaten: bool,
}

impl Apple {
    fn new() -> Apple {
        Apple { eaten: false }
    }

    fn describe(&self) -> String {
        if !self.eaten {
            String::from("It's a tempting red apple.")
        } else {
            String::from("It's an apple core.")
        }
    }

    fn consume(&mut self) -> Result<String, &str> {
        if !self.eaten {
            self.eaten = true;
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

    fn describe(&self) -> String {
        format!(
            "It's a book. The title reads \"{}\" by {}.",
            &self.title, &self.author
        )
    }

    fn read(&self) -> Result<String, &str> {
        Ok(format!("The book reads:\n{}", &self.contents))
    }
}

pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            entities: vec![
                Entity::Apple(Apple::new()),
                Entity::Book(Book::new(
                    String::from("The Lusty Argonian Maid"),
                    String::from("Crassius Curio"),
                    String::from("[contents here]"),
                )),
            ],
        }
    }

    fn find_entity(&mut self, ident: EntityIdent) -> Result<&mut Entity, String> {
        match ident {
            EntityIdent::NullEntity(ident) => Err(ident),
            EntityIdent::Apple => Ok(&mut self.entities[0]),
            EntityIdent::Book => Ok(&mut self.entities[1]),
        }
    }

    pub fn do_instruction(&mut self, instruction: Instruction) -> String {
        match instruction {
            Instruction::Exit => panic!("Can't perform exit instuction on scene"),
            Instruction::Look => String::from("You look around and see an apple and a book."),
            Instruction::Describe(ident) => match self.find_entity(ident) {
                Ok(entity) => match entity {
                    Entity::Apple(apple) => apple.describe(),
                    Entity::Book(book) => book.describe(),
                },
                Err(ident) => format!("You can't find a {}", ident),
            },
            Instruction::Consume(ident) => match self.find_entity(ident) {
                Ok(entity) => {
                    let result = match entity {
                        Entity::Apple(apple) => apple.consume(),
                        Entity::Book(_) => Err("It's not food."),
                    };
                    match result {
                        Ok(response) => response,
                        Err(error) => format!("{} You decide not to eat it.", error),
                    }
                }
                Err(ident) => format!("You can't find a {}.", ident),
            },
            Instruction::Read(ident) => match self.find_entity(ident) {
                Ok(entity) => {
                    let result = match entity {
                        Entity::Book(book) => book.read(),
                        Entity::Apple(_) => Err("There's nothing to read."),
                    };
                    match result {
                        Ok(response) => response,
                        Err(error) => format!("{}", error),
                    }
                }
                Err(ident) => format!("You can't find a {}.", ident),
            },
        }
    }
}
