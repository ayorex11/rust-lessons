/*fn main() {
    let name = String::from("Ayo");
    let age = 25;
    let greeting = build_greeting(name, age);
    println!("{greeting}");
    println!("Original name : {name}"); // won't work because the string value has been moved
    println!("Original age : {age}");
}


fn build_greeting(name: String, age:i32) -> String{
    format!("Hello {name}, you are {age}")
}*/
// Age works after being passed to the function because it is a known size at compile time
//Essentially, it is copied, not moved. Name doesn't work because it is moved by default
//Unless we specify .clone()

// Compiler says ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
// and we are trying to borrow what has been moved

fn main() {
    let name = String::from("Ayo");
    let age = 25;
    let greeting = build_greeting(name.clone(), age);
    println!("{greeting}");
    println!("Original name: {name}");
    println!("Original age: {age}");
}


fn build_greeting(name: String, age:i32) -> String{
    format!("Hello {name}, you are {age}")
}