// Day 7: Trait

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News: {}: {}", self.headline, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet: {}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Thanks for being here!"),
    };

    let news = NewsArticle {
        headline: String::from("Rust lang is awesome"),
        content: String::from("Rust is a general-purpose programming language without relying on a garbage collector."),
    };

    println!("{}", tweet.summarize());
    println!("{}", news.summarize());
}