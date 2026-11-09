/*struct User{ // defining a struct
        active: bool,
        username: String,
        email: String,
        number_of_signins: u64, 
    }

//tuple structs - structs without name fields
struct Color(i32,i32,i32); // creating a new type but with the simplicity of tuples


fn main() {
    let user1 = User{ // using a struct
        active: true,
        username: String::from("Ayorex"),
        email: String::from("adewale@gmail.com"),
        number_of_signins: 1,
    };

    let mut user2 = User { // mutable instance
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        number_of_signins: 1,
    };
    user2.email = String::from("anotheremail@example.com"); // usage of dot notation to gain access
    // to a specific field.

    let user3 = User{ // using values from another instance this is struct update syntax.
        active: user1.active,
        username: user1.username,
        email: String::from("another@gmail.com"),
        number_of_signins: user1.number_of_signins,
    };

    let user4 = User{
        email: String::from("Justanother@gmail.com"),
        ..user2 // faster way of using update syntax. It tells the program that all the fields not
        // explicitly stated should remain the same.
    };
    let user5 = build_user(String::from("example@gmail.com"),
     String::from("example234"));
}

fn build_user(email: String, username: String) -> User{ // function to build user
    User { active: true, 
        username, // we can leave it like this since it has the same name as the func paremeters.
        email, // same here instead of doing email: email.
        number_of_signins: 1 
    }
}*/
// examaple of how struct can be used
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle { // creating a method specific to the Rectangle struct
    fn area(&self) -> u32 { // makes it clear that the area function is associated to rectangle
        self.width * self.height
    }
}

impl Rectangle{
    fn sqaure(size: u32) -> Rectangle{ // associated function different from method as it does not reference self
        Rectangle { width: size, height: size }
    }
}

fn main(){
    let rectangle1 = Rectangle{
        width: 30,
        height: 50,
    };
    //let area = area(&rectangle1);
    let area = rectangle1.area();
    let rect2 = Rectangle::sqaure(25); // instead of manually creating rectangle1, 2, etc we can easily reuse this.
    let area2 = rect2.area();

    println!("The area of the rectangle is {area}");
    println!("The area of the rectangle 2 is {area2}");

}

/*fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}*/