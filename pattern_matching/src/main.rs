/*enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    let nickel = Coin::Nickel;
    let value = value_in_cents(nickel);
    println!("{value}");
}

fn value_in_cents(coin: Coin) -> u8{ // pattern matching using variants of an enum Coin type.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}*/

use crate::UsStates::{Alabama, Alaska};

// Pattern that binds to values
#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
    NewJersey,
    //etc
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates) // specifying what state the quarter belongs to i.e what type
}


fn main(){
    let quarter = Coin::Quarter(Alabama);
    let value = value_in_cents(quarter);
    println!("{value}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of the variable five is {six:#?}")
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // match statement that deduces what state when sorting change
            println!("State quarter from {state:?}!");
            25
        }
    }
}
// we can also use match to determine if a value exists in some or it's none
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // if no value exists it returns None
            Some(i) => Some(i + 1), // if a value exists it returns the value +1 
            // This function is for i32 datatypes. We can also do the same with char and other datatypes
        }
}

/*
 let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch all pattern in rust
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll()

     let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // _ is also a catch all in rust but in this case we don't want to do anything with the value so we tell rust to ignore
        the value and reroll.

        we can also replace "reroll()" with "()" which is an empty unit type essentially telling rust to do nothing.
    }

*/