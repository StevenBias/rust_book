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

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

/****   Trait bounds with where Clauses  ****/
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    42
}

fn notify<T: Summary>(item: T) {
    println!("Breaking new! {}", item.summarize());
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

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
}
