enum Account{
    Saving{
        amount:i32,
    },
    Credit{
        amount:i32,
        interest_rate:f32,
    },
    Periodic{
        amount:i32,
        periodic_increment:i32,
    },
}

enum Change{
    Deposit(i32),
    Withdraw(i32),
}

fn main() {
    println!("Hello, world!");
}
