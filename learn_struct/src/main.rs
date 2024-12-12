struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let mut user1 = User{
        email: String::from("amadeus@example.com"),
        username: String::from("Amadeus"),
        active:true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone@example.com");
    println!("email is {}", user1.email);

    let user2 =  User{
        email: String::from("22@example.com"),
        username: String::from("22"),
        active:user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 可简化
    let user2 =  User{
        email: String::from("22@example.com"),
        username: String::from("22"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 可以作为函数返回值
fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active:true,
        sign_in_count: 1,
    }
}
