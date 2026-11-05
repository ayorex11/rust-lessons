fn main() {
    let mut counter = 0;
    let result = loop{
        counter +=1;
        if counter == 10{
            break counter *2;
        }
    };
    countdown();
    println!("The result is {result}");
}

fn countdown(){
    for number in (1..4).rev(){
        println!("{number}");

    }
    println!("TAKEOFF");
}
