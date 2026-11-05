use std::io;
fn main() {
    println!("Input a number");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Invalid Input");


    another_function(x);
}

fn another_function(x:String){
    println!("The value of x is: {x}");
}
