struct User {
    active: bool,
    username: String,
    email: String,
    visits: u64,
}

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        visits: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("rusty"),
        email: String::from("rustyxlol@pm.me"),
        visits: 10,
    };

    let user2 = build_user(String::from("hello@world.com"), String::from("helloworld"));

    let user3 = User {
        email: String::from("NewEmail@email.com"),
        ..user2
    };

    let black = Color(0, 0, 0);

    println!("{}", black.0);

    println!(
        "{} {} {} {}",
        user1.username, user1.active, user1.visits, user3.email
    );
}
