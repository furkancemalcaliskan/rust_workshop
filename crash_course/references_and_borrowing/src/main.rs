// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References
// References: Enable you to borrow values without taking ownership
//
// Immutable Reference.
// Mutable Reference.
// Create Reference by add "&"

// Mutable Reference
// Create Reference by add "&"

//  -I- Immutable Reference

// fn main() {
//     // let x: i32 = 5;
//     // let r = x; // transfering ownership, x will dropped
//     let _x: i32 = 5;
//     let _r: &i32 = &_x;
//     println!("Value of _x: {}", _x);
//     println!("Value of _r: {}", _r);
// }

//  -II- Mutable Reference

// fn main() {
//     let mut _x: i32 = 5;
//     let _r: &mut i32 = &mut _x;
//     *_r += 1;
//     *_r -= 3;
//
//     // println!("Value of _x: {}", _x);
//     println!("Value of _r: {}", _r);
// }

// Demonstration on one mutable reference or many immutable references

fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        )
    }
}
