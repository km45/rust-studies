trait Summary {
    fn summarize(&self) -> String;

    fn summarize2(&self) -> String {
        // default implementation
        String::from("(Read more...)")
    }

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize2(&self) -> String {
        // override
        format!("(Show details...)")

        // NOTE: We cannot call default implementation from override method.
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// use trait as parameter
// NOTE: `impl Trait` syantax is syntax sugar for `trait bound`.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

trait HashTag {}
impl HashTag for Tweet {}
// specify multiple trait bound using `+`
fn notify3<T: Summary + HashTag>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// specify trait bound(s) using `where` clause
fn notify4<T>(item: &T)
where
    T: Summary + HashTag,
{
    println!("Breaking news! {}", item.summarize());
}

// use trait as return value type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("tos"),
        content: String::from("Good night."),
        reply: false,
        retweet: false,
    }
}

fn main() {
    {
        let tweet = Tweet {
            username: String::from("tos"),
            content: String::from("Good morning!"),
            reply: false,
            retweet: false,
        };

        let news_article = NewsArticle {
            headline: String::from("sample"),
            location: String::from("moon"),
            author: String::from("bot"),
            content: String::from("this is sample content"),
        };

        println!("1 new tweet: {}", tweet.summarize());
        println!("1 new tweet: {}", tweet.summarize2());
        println!("1 new tweet: {}", tweet.summarize3());

        println!("1 new article: {}", news_article.summarize());
        println!("1 new article: {}", news_article.summarize2());
        println!("1 new article: {}", news_article.summarize3());

        notify(&tweet);
        notify2(&tweet);
        notify3(&tweet);
        notify4(&tweet);
        notify(&news_article);
        notify2(&news_article);
        // notify3(&news_article); // this causes error
        // notify4(&news_article); // this causes error

        notify(&returns_summarizable());
    }
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    {
        let p = Pair { x: 1, y: 2 };
        p.cmp_display();
    }
}

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::fmt::Display;

// implement method depending on trait bound
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementation
//
// > We can also conditionally implement a trait for any type that implements
// > another trait. Implementations of a trait on any type that satisfies the
// > trait bounds are called blanket implementations and are extensively used
// > in the Rust standard library.
