use std :: io;
fn main() {
    let x = [1,2,3,4,5];
    println!("Please input your index ");
    let mut index = String::new();
    io::stdin()
        .read_line(& mut index)
        .expect("Failed to read input");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let value = x[index];
    println!("The value at index {index} is {value}");


}
