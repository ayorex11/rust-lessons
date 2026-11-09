#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height // check if self can contain other rectangle instance
    }
    fn is_square(&self) -> bool{
        self.width == self.height
    }
    fn square(size: u32) -> Self{
        Self { width: size, height: size }
    }
}
struct Color(u8, u8, u8);
struct Point(i32, i32);


fn main(){
    let c = Color(255, 128, 0);
    let p = Point(3, -4);
    let Color(r,g,b) = c;
    println!("Color: {r}, {g}, {b}");
    println!("Point: x={}, y={}", p.0, p.1);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("square(20) is square: {}", Rectangle::square(20).is_square());
    println!("rect1 is square: {}", rect1.is_square());
    dbg!(&rect1); // this prints to stderr
    // if we wrote dbg!(rect1) it takes ownership of it essentially moving it.
}
