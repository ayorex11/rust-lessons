fn main() {
    println!("Hello, world!");
    fizz_buzz();
    seven_multiple();
}

fn fizz_buzz(){
    for number in 1..21{
        if (number % 3) == 0{
            if (number % 5) == 0{
                println!("FizzBuzz");
            } else {
                println!("Fizz");
            }
        } else if (number % 5) == 0 {
            println!("Buzz");
        } else {
            println!("{number}");
        }
    }
}

fn seven_multiple(){
    const SEVEN_MULTIPLE : u32 = 7;
    let mut checker : u32 = 0;
    loop {
        checker += SEVEN_MULTIPLE;
        if checker > 50 {
            break println!("{checker}");
        }
    }


}