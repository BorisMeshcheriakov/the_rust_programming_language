fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    struct AlwaysEquel;

    let subject = AlwaysEquel;

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of rectangle is {} square pixels.",
        tupleArea(rect1)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tupleArea(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
