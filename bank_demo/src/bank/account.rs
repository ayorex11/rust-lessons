pub struct Account {
    pub owner: String,
    balance: u32
}

pub fn open(owner: &str, opening: u32) -> Account{
    Account{ 
        owner: owner.to_string(), 
        balance: opening 
    }
}

impl Account{
    pub fn balance(&self) -> u32{
        self.balance
    }
}