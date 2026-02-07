use std::io;
pub mod accounts_file;
pub use accounts_file::Account;

pub struct Bank {
    name: String,
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new(name: String) -> Bank {
        Bank {
            name, accounts: Vec::new(),
        }
    }

    pub fn open_account(&mut self, owner: String) -> String {
        let acc = Account::new(owner);
        self.accounts.push(acc);
        self.accounts.last().unwrap().account_number.clone()
    }

    pub fn find_account(&self, account_number: &str) -> Option<&Account> {
        for acc in &self.accounts {
            if account_number == acc.account_number {
                return Some(acc);
            }
        }
        None
    }

    pub fn find_account_mut(&mut self, account_number: &str) -> Option<&mut Account> {
        for acc in &mut self.accounts {
            if account_number == acc.account_number {
                return Some(acc);
            }
        }
        None
    }

    pub fn total_deposits(&self) -> f64 {
        let mut summa = 0.0;
        for acc in &self.accounts {
            summa += acc.balance;
        }
        println!("Total deposit of {} consists: {}", self.name, summa);
        summa
    }
}

pub fn introduce_string() -> String {
    println!("Introduce name: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error!");
    input.trim().to_string()
}