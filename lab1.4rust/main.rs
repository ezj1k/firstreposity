use lab1::Bank;
use lab1::introduce_string;

fn main() {
    let mut bank1 = Bank::new(introduce_string());

    for _i in 1..=3 {
        let name = introduce_string();
        bank1.open_account(name);
    }

    let (first_slice, rest) = bank1.accounts.split_at_mut(1);
    let first = &mut first_slice[0];
    let (second_slice, third_slice) = rest.split_at_mut(1);
    let second = &mut second_slice[0];
    let third = &mut third_slice[0];

    first.deposit(100.0);
    first.withdraw(100.0);
    first.withdraw(100.0);

    second.deposit(100.0);
    second.withdraw(200.0);
    second.transfer(first, 200.0);

    third.deposit(100.0);
    third.withdraw(50.0);
    third.transfer(first, 50.0);

    println!("{:?}", first);
    println!("{:?}", second);


    bank1.total_deposits();
}