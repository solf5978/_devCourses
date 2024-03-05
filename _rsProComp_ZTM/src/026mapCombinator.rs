#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(3),
        "hello" => Some(5),
        "kitty" => Some(7),
        _ => None,
    }
}

fn main() {
    let user_name = "sam";
    let user = find_user(user_name).map(|user_id| User {
        user_id,
        name: user_name.to_owned(),
    });

    match user {
        Some(User) => println!("{user:?}"),
        None => println!("user not found"),
    }
}