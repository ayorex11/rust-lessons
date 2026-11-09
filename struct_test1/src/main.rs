#[derive(Debug)]
struct User{
    active: bool, 
    username: String, 
    email: String, 
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User{
    User { active: true, 
        username, 
        email, 
        sign_in_count: 1 
    }
}
fn main(){
    let user1 = 
    build_user(
        String::from("ayo@example.com"),
        String::from("ayo123")
    );
    
    let user2 = User{
        email: String::from("new@example.com"),
        ..user1
    };

    println!("{user2:#?}");
    println!("{}", user1.email);
}