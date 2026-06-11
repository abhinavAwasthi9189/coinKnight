#[derive(Debug, Clone, Copy, PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone)]
enum Frequency_spend {
    Monthly { increment: i32, times: i32 }, // e.g., how many times in a month
    Weekly { increment: i32, times: i32 },  // e.g., how many times in a week
    CustomDays { increment: i32, days: Vec<Day> }, // e.g., vec![Day::Monday, Day::Tuesday, Day::Friday]
}

enum Account {
    Saving {
        amount: i32,
    },
    Credit {
        amount: i32,
        interest_rate: f32,
    },
    Periodic {
        amount: i32,
        periodic_increment: Vec<Frequency_spend>,
    },//account if you spend a lot of tiny payment on same things multiple times a week or month, you can set up a periodic account to track those spending and make sure you have enough money for them.
}
enum Change {
    Deposit(i32),
    Withdraw(i32),
}

impl Account {
    fn create_account(&mut self) {//create account and set up what type it is.
        println!("what type of account do you want to create? (Saving[1], Credit[2], Periodic[3])");
        let account_type = intput();
        match account_type {
            1 => {
                println!("Enter initial amount for Saving account:");
                let mut amount = intput() as i32;
                *self = Account::Saving { amount };
            }
            2 => {//credit account will have interest rate, so we need to set up interest rate when creating credit account.
                println!("Enter initial amount for Credit account:");
                let mut amount = intput() as i32;
                println!("Enter interest rate for Credit account:");
                let interest_rate = intput() as f32;
                *self = Account::Credit {
                    amount,
                    interest_rate,
                };
            }
            3 => {//periodic account will have periodic increment, so we need to set up periodic increment when creating periodic account.
                println!("Enter initial amount for Periodic account:");
                let mut amount = intput() as i32;
                let mut periodic_increment: Vec<Frequency_spend> = Vec::new();
                loop {
                    println!("Add a periodic increment? (yes[1], no[0])");
                    let add_increment = intput();
                    if add_increment == 0 {
                        break;
                    } else {
                        self.periodic_incre_decrement(&mut periodic_increment);
                    }
                }
            }
        }
    }
    fn periodic_incre_decrement(&mut self, &mut periodic_increment: Vec<Frequency_spend>) {//made it a function so you can add values later too.
        println!(
            "what type of increment do you want to add? (Monthly[1], Weekly[2], CustomDays[3])"
        );
        let increment_type = intput();
        match increment_type {
            1 => {
                println!("Enter increment amount for Monthly:");
                let increment = intput() as i32;
                println!("Enter how many times in a month:");
                let times = intput() as i32;
                periodic_increment.push(Frequency_spend::Monthly { increment, times });
            }
            2 => {
                println!("Enter increment amount for Weekly:");
                let increment = intput() as i32;
                println!("Enter how many times in a week:");
                let times = intput() as i32;
                periodic_increment.push(Frequency_spend::Weekly { increment, times });
            }
            3 => {
                println!("Enter increment amount for CustomDays:");
                let increment = intput() as i32;
                println!("Enter days for CustomDays (e.g., Monday, Tuesday, Friday):");
                let mut days_input = String::new();
                io::stdin()
                    .read_line(&mut days_input)
                    .expect("Failed to read line");
                let days: Vec<Day> = days_input
                    .trim()
                    .split(',')
                    .map(|day| match day.trim() {
                        "Monday" => Day::Monday,
                        "Tuesday" => Day::Tuesday,
                        "Wednesday" => Day::Wednesday,
                        "Thursday" => Day::Thursday,
                        "Friday" => Day::Friday,
                        "Saturday" => Day::Saturday,
                        "Sunday" => Day::Sunday,
                        _ => panic!("Invalid day input"),
                    })
                    .collect();
                periodic_increment.push(Frequency_spend::CustomDays { increment, days });
            }
            _ => println!("Invalid increment type"),
        }
    }
}
fn main() {
    println!("Hello, world!");
}

fn intput() -> u8 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u8 = input.trim().parse().expect("Please type a number!");
    input
}
