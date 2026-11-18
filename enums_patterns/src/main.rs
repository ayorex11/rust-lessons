enum IPAddrKind{ // custom datatype that can be used to specify if a value is one of a variant.
    V4(String),
    V6(String)
}

enum IPAddr2{
    V4(u8,u8,u8,u8),
    V6(String) // additional benefit of enum each variant can have different types and amounts of data attached to each variant.
}

enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String)
}

enum Option<T>{
    None,
    Some(T)
}

// defining a method for enums
impl Message{
    fn call(&self){
    }
}

/*struct IpAddress{ using structs with enum 
    kind: IPAddrKind,
    address: String

}*/
fn main() {
    let four = IPAddrKind::V4(String::from("127.0.0.1")); // variants are namespaced but they are still
    let six = IPAddrKind::V6(String::from("::1"));// the same 
    // Instead of using structs we can put data in directly into each variants of our enum.
    /*let home1 = IpAddress{
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };*/
    let broadcast = IPAddr2::V4(255, 0, 0, 0);
    // defining and using methods on enums
    let m = Message::Write(String::from("Hello"));
    m.call();
    let some_number = Some(5); // Option<T> becomes type i32
    let some_char = Some('e'); // Option<T> becomes type char
    //let absent_number: Option<i32> = None; // This is None so we have to manually specify what type it is.

    /*let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;*/ // This won't run as the compiler does not know that Option<i8> is a definite value.

    

}
