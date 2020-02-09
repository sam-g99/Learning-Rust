struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("anyone@example.com"),
        username: String::from("anyusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("adifferentemail.com");

    println!("The users email is {}", user1.email);

    let user2 = create_user(
        String::from("bobby@example.com"),
        String::from("cool_username"),
    );

    println!("The user username is {}", user2.email);

    // Update Syntax
    let user3 = User {
        email: String::from("sally@example.com"),
        username: String::from("sally123"),
        ..user1 // this fills in all the fields not set with the values from user 2
    };

    println!("This user signed in {} time(s)", user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {} Point: {}", black.0, origin.0);
}

fn create_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username, (no need to repeat [shorthand])
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
