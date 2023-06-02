fn main() {
    let mut user1 = User { // Instance of user struct
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // entire instance must be mutable - rust doesn't allow specific fields to be mutable

    let user1 = build_user(
        String::from("anotherUser"),
        String::from("newemail@example.com"));

    let user2 = User {
        email: String::from("newemail2@example.com"),
        ..user1
    };

    let color1 = Color(32, 78, 92);
    println!("Color: {}R {}G {}B", color1.0, color1.1, color1.2);

    let point1 = Point(-5, 32, 872);
    println!("Point: ({}, {}, {})", point1.0, point1.1, point1.2);

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool, // Each data member is a "field"
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // Even though the data of these are equal, they cannot be used interchangably
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit-Like structs. These structs are good for testing, and don't have any fields.