pub enum Entity {
    Apple(Apple),
    Book(Book),
}

pub struct Apple {
    eaten: bool,
}

impl Apple {
    fn new() -> Apple {
        Apple { eaten: false }
    }

    pub fn describe(&self) -> String {
        if !self.eaten {
            String::from("It's a tempting red apple.")
        } else {
            String::from("It's an apple core.")
        }
    }

    pub fn consume(&mut self) -> Result<String, &str> {
        if !self.eaten {
            self.eaten = true;
            Ok(String::from("It's delicious! All that's left is the core."))
        } else {
            Err("The core doesn't look appetising.")
        }
    }
}

pub struct Book {
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

    pub fn describe(&self) -> String {
        format!(
            "It's a book. The title reads \"{}\" by {}.",
            &self.title, &self.author
        )
    }

    pub fn read(&self) -> Result<String, &str> {
        Ok(format!("The book reads:\n{}", &self.contents))
    }
}

pub struct Scene {
    pub entities: Vec<Entity>,
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
}
