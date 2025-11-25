#[derive(Debug)]
struct Account {
    account_number: String,
    balance: f64,
    owner: String,
}

impl Account {
    fn new(account_number: String, owner: String) -> Account {
        Account {
            account_number,
            balance: 0.0,
            owner,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn transfer(&mut self, to: &mut Account, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            to.balance += amount;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(name: String) -> Bank {
        Bank {
            name,
            accounts: Vec::new(),
        }
    }

    fn open_account(&mut self, account_number: String, owner: String) -> String {
        let cont = Account::new(account_number.clone(), owner);
        self.accounts.push(cont);
        account_number
    }

    fn find_account(&self, owner: String) -> Option<&Account> {
        self.accounts.iter().find(|boss| boss.owner == owner)
    }

    fn find_account_mut(&mut self, account_number: &str) -> Option<&mut Account> {
        self.accounts
            .iter_mut()
            .find(|account| account.account_number == account_number)
    }

    fn total_deposits(&self) -> f64 {
        self.accounts.iter().map(|a| a.balance).sum()
    }
}

pub fn main() {
    let mut bank = Bank::new("MICB".to_string());

    bank.open_account("1".to_string(), "Ion".to_string());
    bank.open_account("2".to_string(), "Ana".to_string());
    bank.open_account("3".to_string(), "Leo".to_string());

    let c2 = bank.find_account("Ion".to_string());
    println!("contul 2 este: {:?}", c2);

    let c1 = bank.find_account_mut("1").unwrap();
    c1.deposit(1000.0);
    c1.withdraw(500.0);

    let from_index = bank
        .accounts
        .iter()
        .position(|a| a.account_number == "1")
        .unwrap();
    let to_index = bank
        .accounts
        .iter()
        .position(|a| a.account_number == "2")
        .unwrap();

    let (from_acc, to_acc) = if from_index < to_index {
        let (left, right) = bank.accounts.split_at_mut(to_index);
        (&mut left[from_index], &mut right[0])
    } else {
        let (left, right) = bank.accounts.split_at_mut(from_index);
        (&mut right[0], &mut left[to_index])
    };

    from_acc.transfer(to_acc, 200.0);

    for acc in &bank.accounts {
        println!("{:?}", acc);
    }

    println!(
        "Banca {:?} are depozit total: {:?}",
        bank.name,
        bank.total_deposits()
    );
}
