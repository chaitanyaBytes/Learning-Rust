struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("chaitanya"),
        sign_in_count: 1,
    };

    print!(
        "{} has signed in {} times and is currently {}",
        user.username, user.sign_in_count, user.active
    );
}
