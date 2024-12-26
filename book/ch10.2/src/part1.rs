pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    
}

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

//Trait bounds

pub fn notify(item: &impl Summary) {
    println!("Breaking News: {}", item.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

//Using multiple traits
pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
    ...
}


pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
    ...
}

//Clearer trait bounds using where clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{

}  

fn main() {
    let article = NewsArticle {
        author: String::from("Skibidi toilet"),
        headline: String::from("Donald Trump loves men"),
        content: String::from("Ong is that the lapras transform"),
    };

    let tweet = Tweet {
        username: String::from("Skibidi toilet"),
        content: String::from("Donald Trump loves men"),
        reply: true,
        retweet: false,
    };
    
    println!("Tweet Summary: {}", tweet.summarize());
    println!("News Article Summary: {}", article.summarize());

    notify(&article);
}
