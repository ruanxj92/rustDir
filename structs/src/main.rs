
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn print_user(user:& User) {
    println!("user: active: {}, username: {}, email:{}, sign_in_count: {}", user.active, user.username, user.email, user.sign_in_count);
}
fn build_user(email: String, username: String) -> User{
    User {
        email: email,// or email,
        username: username,// or username
        active: true,
        sign_in_count:1,
    }
}

fn build_user_short_hand(email: String, username: String) -> User{
    User {
        email,//short for email: email,
        username,//short for username: username,
        active: true,
        sign_in_count:1,
    }
}

//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/*
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
life time has to be conserned carefully
*/

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//元组
fn area_dimension(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };
    print_user(&user1);

    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user_short_hand("aaa@exapmle.com".to_string(), "aaa".to_string());
    let user3 = build_user("aaa@exapmle.com".to_string(), "aaa".to_string());

    let user4 = User {
        email: String::from("anthor@example.com"),
        ..user1
    };
    print_user(&user2);
    print_user(&user3);
    print_user(&user4);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_dimension(rect1)
    );
}
