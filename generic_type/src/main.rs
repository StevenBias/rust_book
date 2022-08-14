use std::fmt::{Debug, Display};
use aggregator::{Summary, Tweet, NewsArticle};

// T type in struct
// All T type fields of Point struct
// must be the same type!
struct Point<T, U> {
    x: T,
    y: U,
}

// Specify rust the we implement method on a struct with generic type
// by adding <T, U> after 'impl'
impl<T, U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + Debug + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x");
        } else {
            println!("The largest member is y");
        }
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    let mut index = 0;
    for (i, item) in list.iter().enumerate() {
        if item > largest {
            largest = &item;
            index = i;
        }
    }
    &list[index]
}

/****   Trait bounds with where Clauses  ****/
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//     42
// }

/****   Return types that implements trait  ****/
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("another_ebooks"),
        content: String::from("Something else to read"),
        reply: false,
        retweet: false,
    }
}
fn notify<T: Summary>(item: T) {
    println!("Breaking new! {}", item.summarize());
}

fn generic_type() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p0 = Point { x: 5, y: 10};
    let p1 = Point { x: 1.0, y: 4.0};
    let p2 = Point { x:5, y: 4.0};
    let p3 = p1.mixup(p0);
    println!("p2.x: {}, p2.y: {}", p2.x, p2.y);
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    notify(tweet);

    let ret_trait = returns_summarizable();
    println!("Test return trait type: {}", ret_trait.summarize());

    let pair = Pair::new(5, 8);
    pair.cmd_display();
}

/*******************    Lifetimes   *******************/
// Add "'a" to declare generic lifetime parameters inside angle brackets
// between the function name and the parameter list
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

/*******************    Lifetimes   *******************/

fn main() {
    // generic_type();

    /*******************    Lifetimes   *******************/
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    println!("The string is: {}", novel);
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let sentence = ImportantExcerpt { part: first_sentence };
    println!("The value of the first sentence is: {}", sentence.part);
    /*******************    Lifetimes   *******************/
}
