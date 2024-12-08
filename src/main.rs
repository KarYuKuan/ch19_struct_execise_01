use std::process::ExitStatus;

fn main() {

    let u1 = User{
        name:String::from("mike"),
        user_id:123,
        email:String::from("jyjzone@hotmail.com"),
        score:89,
    };
}

struct User {
    name:String,
    user_id:u8,
    email:String,
    score:u16,
}
