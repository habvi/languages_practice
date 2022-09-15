pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("read more from {}", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// different types
pub fn notify1(item1: &impl Summary, item2: &impl Summary) {
    println!("news! {} {}", item1.summarize(), item2.summarize());
}

// same types
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("news! {} {}", item1.summarize(), item2.summarize());
}

use std::fmt::Display;
pub fn notify3(item: &(impl Summary + Display)) {
    println!("{}", item);
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Bob"),
        content: String::from("Bob's tweet."),
        reply: false,
        retweet: false,
    }
}

fn run1() {
    let tweet = Tweet {
        username: String::from("Alice"),
        content: String::from("Alice's tweet."),
        reply: false,
        retweet: false,
    };
    println!("{} : {}", tweet.summarize(), tweet.content);

    println!("{}", return_summarizable().summarize());
}

fn main() {
    run1();
}
