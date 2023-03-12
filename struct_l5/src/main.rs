fn main() {
    let user1 = User{
        email: String::from("mail@mail.ru"),
        username: String::from("Pavel"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("second user"));
    print_user(user1);
    println!("====================");
    print_user(user2);

}

fn print_user(user: User) {
    println!("Hello, {} ~ {} ~ {} ~ {}",
             user.username,
             user.email,
             user.active,
             user.sign_in_count
    );
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String) -> User {
    User{
        username,
        email: String::from("default"),
        active: true,
        sign_in_count: 0,
    }
}