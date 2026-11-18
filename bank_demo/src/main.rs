use bank_demo::bank::{self, account};
fn main() {
    let new_account = account::open("Ayo", 500);
    let bank_name = bank::bank_name();
    let name = &new_account.owner;
    let balance = new_account.balance();
    println!("Bank: {bank_name}");
    println!("Owner: {name}");
    println!("Balance: {balance}")
}

