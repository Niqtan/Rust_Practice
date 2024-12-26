struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut  user1 = User {
        email: String::from("niqban123@gmail.com"),
        username: String::from("Niqban"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username String::from("Skibidi Toilet");

    let user2 = build_user(String::from("skibidi@gmail.com"), 
    String::from("yNOT1"));
}
    let user3 = User {
        email: String::from("me when@gmail.com"),
        username: String::from("me when@gmail.com"),
        ..user2

    //Structs without name fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    }

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count 1,
    }
}