trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("New: {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("rusty"),
        content: String::from("Modules are hard"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    println!("1 new tweet: {}", tweet.summarize());
}
