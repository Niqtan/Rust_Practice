pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    //Doesn't need function body since we want it to be shared, but not
    //specific to some things
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    
        //Default implementation if no content is given for a specific
        //Implementation method

    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Horse_ebooks"),
        content: String::from("mom"),
        reply: false,
        retweet: false,
    }
}


fn main() {
    println!("{}", returns_summarizable().summarize());
}
