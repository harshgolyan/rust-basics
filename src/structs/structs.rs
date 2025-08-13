#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user1: User = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@gmail.com"),
        active: true,
    };
    println!("User name : {}", user1.name);
}