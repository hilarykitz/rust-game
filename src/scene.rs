pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(entities: Vec<Entity>) -> Scene {
        Scene { entities }
    }
}

pub enum Entity {
    Apple(Apple),
    Book(Book),
    Wrench(Wrench),
}

pub struct Apple {
    pub consumed: bool,
}

impl Apple {
    pub fn new() -> Apple {
        Apple { consumed: false }
    }

    pub fn describe(&self) -> String {
        if !self.consumed {
            String::from("It's a tempting red apple.")
        } else {
            String::from("It's an apple core.")
        }
    }

    pub fn consume(&mut self) -> Result<String, &str> {
        if !self.consumed {
            self.consumed = true;
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
    pub fn new(title: String, author: String, contents: String) -> Book {
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

pub struct Wrench;

impl Wrench {
    pub fn new() -> Wrench {
        Wrench
    }

    pub fn describe(&self) -> String {
        format!("It's a wrench.")
    }
}
