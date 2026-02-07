use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, Duration};

static NEXT_ACC_NR: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(1));

#[derive(Debug)]
pub struct Account {
    pub account_number: String,
    pub balance: f64,
    owner: String,
    withdraw_today: f64,
    last_withdraw_time: SystemTime,
    history: Vec<f64>,
}

impl Account {
    pub fn new(owner: String) -> Account {
        let numeric_account_number = NEXT_ACC_NR.fetch_add(1, Ordering::SeqCst);
        Account {
            account_number: numeric_account_number.to_string(),
            balance: 0.0,
            owner,
            withdraw_today: 100.0,
            last_withdraw_time: SystemTime::UNIX_EPOCH,
            history: Vec::new(),
        }
    }

    pub fn refresh_daily_limit(&mut self) {
        let now = SystemTime::now();

        if now.duration_since(self.last_withdraw_time).unwrap() >= Duration::from_secs(5) { //seconds
            self.withdraw_today = 0.0;
            self.last_withdraw_time = now;
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.history.push(amount);
        println!("{}, your balance replenished by {}$!", self.owner, amount);
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        let daily_amount = 100.0;
        self.refresh_daily_limit();

        if self.withdraw_today + amount > daily_amount {
            println!("{} tries to withdraw more than the daily limit!", self.owner);
            return false;
        }
        if self.balance < amount {
            println!("{} dont have enough money on balance!", self.owner);
            return false;
        }

        self.balance -= amount;
        self.withdraw_today += amount;
        self.last_withdraw_time = SystemTime::now();
        println!("Withdraw passed succesfull!");
        self.history.push(-amount);
        true
    }

    pub fn transfer(&mut self, to: &mut Account, amount: f64) -> bool {
        if self.withdraw(amount) {
            to.deposit(amount);
            println!("The {}$ was transfered from {} to {}",
            amount,
            self.account_number,
            to.account_number);
            return true;
        }
        false
    }
}