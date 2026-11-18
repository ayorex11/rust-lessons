#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),   // holds the state name
}

fn value_in_cents(coin: &Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state}");
            25
        }
    }
}
fn describe_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else{
        return None;
    };
    Some(format!("Quarter from {state}"))
}


fn main() {
    let coins = vec![
        Coin::Penny,
        Coin::Quarter(String::from("Lagos")),
        Coin::Dime,
        Coin::Nickel
    ];
    let mut total = 0;
    let mut quarters = 0;
    for coin in &coins{
        total+=value_in_cents(coin);
        if let Coin::Quarter(_) = coin{
            quarters+=1
        }
    }
    println!("total: {total}");
    println!("quarters: {quarters}");
    println!("{:?}", describe_quarter(&coins[1]));
    println!("{:?}", describe_quarter(&coins[0]));

    //let x: Option<i32> = Some(5);
    //let y = x + 1; This wwon't run because you can't add integer to Option type.
}


// Snippet 1
/*fn label(n: u8) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
    }
} This won't run as all possible values of n has not been compared, essentially non exhuastive pattern

// Snippet 2
fn label2(n: u8) -> &'static str {
    match n {
        0 => "zero",
        other => "something else", 
        1 => "one", 1 won't be reached because of the placement of other, any other thing after is ignored
    }
}*/
