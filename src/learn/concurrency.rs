use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

#[derive(Debug)]
struct BankAccount {
    balance: Mutex<i64>,
}

impl BankAccount {
    fn new() -> Self {
        BankAccount {
            balance: Mutex::new(0),
        }
    }
    fn balance(&self) -> i64 {
        *self.balance.lock().unwrap()
    }
    fn deposit(&self, amount: i64) {
        *self.balance.lock().unwrap() += amount;
    }
    fn withdraw(&self, amount: i64) -> bool {
        if *self.balance.lock().unwrap() < amount {
            println!("Your balance is not enough");
            return false;
        }
        *self.balance.lock().unwrap() -= amount;
        true
    }
}

fn pay_in(account: &BankAccount, amount: i64) {
    account.deposit(amount);
}

fn take_out(account: &BankAccount, amount: i64) {
    account.withdraw(amount);
}

pub fn test() {
    let account = Arc::new(BankAccount::new());

    let account2 = account.clone();
    let taker = spawn(move || take_out(&account2, 100));

    let account3 = account.clone();
    let payer = spawn(move || pay_in(&account3, 30));

    payer.join().unwrap();
    taker.join().unwrap();

    println!("Account = {:?}", account);
}
