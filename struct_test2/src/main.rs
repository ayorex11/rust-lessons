struct Account {
    owner: String,
    balance: u32
}

impl Account{
    fn new(owner: &str) -> Self{
        Self { 
            owner: owner.to_string(), 
            balance: 0 
        }
    }
    fn deposit(&mut self, amount: u32) {
        self.balance += amount
    }
    fn withdraw(&mut self, amount: u32) -> bool{
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else{
            false
        }
    }
    fn summary(&self) -> String{
        let owner = &self.owner; // reference for string
        let balance = self.balance; // copy automatically because it's an integer
        format!("{owner} has {balance}")
    }
    fn close(self) -> u32{
        self.balance
    }

}
fn main() {
    let mut acc = Account::new("Ayo"); // if i remove mut it affects the deposit and withdraw method
    acc.deposit(100);
    let ok = acc.withdraw(30);
    println!("withdraw 30: {ok}");
    let ok = acc.withdraw(500);
    println!("withdraw 500: {ok}");
    println!("{}", acc.summary());
    let remaining = acc.close();
    println!("closed with {remaining}");
    //println!("{}", acc.summary()); it won't work as it has been moved by the close method.
}
