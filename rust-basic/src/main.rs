use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

struct User {
    id: i32,
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User{
        id: 0,
        email: email,
        username: username,
        sign_in_count: 0,
        active: false,
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Color(i32, i32, i32);
struct Currency(String, u64);

struct Point<T> {
    x: T,
    y: T,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
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

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    println!("Hello, world!");
    let user = build_user(
        String::from("hello@raka.dev"), 
        String::from("mraka"),
    );
    println!("{}", user.username);

    let number_list = vec![102, 34, 6000, 3000];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let float = Point { x: 1.0, y: 4.0 };
    let integer = Point { x: 5, y: 10 };

    let tweet = Tweet{
        username: String::from("mraka"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("another new tweet: {}", returns_summarizable().summarize());

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
