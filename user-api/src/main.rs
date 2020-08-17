use std::io::{stdout, BufWriter};
use mysql::*;
use mysql::prelude::*;
use std::env::var;

#[macro_use]
extern crate mysql;

pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    display_name: String,
    avatar: String,
    active: bool,
}

pub fn build_user(username: String, email: String, password: String) -> User {
    User {
        id: 0,
        username: username,
        password: password,
        email: email,
        display_name: String::from(""),
        avatar: String::from(""),
        active: false,
    }
}

pub fn store_user(pool: &Pool, user: User) -> i32 {
    if user.id == 0 {
        insert_user(pool, user)
    } else {
        update_user(pool, user)
    }
}

pub fn insert_user(pool: &Pool, user: User) -> i32 {
    let query = r"
        insert into user (
            username, 
            password, 
            email, 
            display_name, 
            avatar, 
            active
        ) values (
            :username, 
            :password, 
            :email, 
            :display_name, 
            :avatar, 
            :active)";
    let mut stmt = pool.prepare(&query).unwrap();
    let last_id = stmt.execute(params! {
        "username" => user.username,
        "password" => user.password,
        "email" => user.email,
        "display_name" => user.display_name,
        "avatar" => user.avatar,
        "active" => user.active,
    }).unwrap().last_insert_id();
    last_id as i32
}

pub fn update_user(pool: &Pool, user: User) -> i32 {
    let query = r"
        update user set
            username = :username,
            password = :password,
            email = :email,
            display_name = :display_name,
            avatar = :avatar,
            active = :active
        where id = :id";
    let mut conn = pool.get_conn().unwrap();
    let result = conn.prep_exec(query, params!{
        "username" => user.username,
        "password" => user.password,
        "email" => user.email,
        "display_name" => user.display_name,
        "avatar" => user.avatar,
        "active" => user.active,
        "id" => user.id,
    }).unwrap();
    let affected_rows = result.affected_rows();
    println!("affected rows: {}", affected_rows);
    return user.id
}

pub fn find_user_by_id(pool: &Pool, id: i32) -> Option<User> {
    let mut conn = pool.get_conn().unwrap();
    let result = conn.first_exec(
        "SELECT * FROM user WHERE id=:id", params!{
            "id" => id
        }).map(|row| {
            row.map(|(id, username, password, email, display_name, avatar, active)| User {
                id,
                username,
                password,
                email,
                display_name,
                avatar,
                active,
            })
        }).unwrap();
    result
}

pub fn find_all_users(pool: &Pool) -> Vec<User> {
    let result = pool.prep_exec("SELECT * FROM user", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, username, email, password, display_name, avatar, active) = mysql::from_row(row);
                User {
                    id,
                    username,
                    password,
                    email,
                    display_name,
                    avatar,
                    active,
                }
            }).collect()
        }).unwrap();
    result
}

fn main() {
    let host = var("MYSQL_HOST").unwrap();
    let user = var("MYSQL_USER").unwrap();
    let password = var("MYSQL_PASSWORD").unwrap();
    let database = var("MYSQL_DATABASE").unwrap();
    let url = format!("mysql://{}:{}@{}/{}", user, password, host, database);
    println!("mysql url: {}", url);
    let pool = mysql::Pool::new(url).unwrap();

    let user = build_user(String::from("mraka7"), String::from("hello7@raka.dev"), String::from("foobar"));
    let id = store_user(&pool, user);
    match find_user_by_id(&pool, id) {
        Some(user) => println!("{}: {}", user.id, user.email),
        None => println!("Sorry user couldn't be found"),
    }
    let all_users = find_all_users(&pool);
    for user in all_users {
        println!("{}: {}", user.id, user.email);
    }
}
