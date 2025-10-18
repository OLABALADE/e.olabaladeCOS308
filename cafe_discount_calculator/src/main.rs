use std::io;

fn main() {
    println!("Welcome to the Cafe Discount Calculator\nInput your bill");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let bill = input.trim().parse::<f32>().expect("Input is not a number");
    let mut discount_bill: f32 = 0.0;
    let mut discount: f32 = 0.0;

    if bill > 5000.0 && bill < 10000.0 {
        discount_bill = bill * 0.9;
        discount = 10.0;
    } else if bill > 10000.0 {
        discount_bill = bill * 0.85;
        discount = 15.0;
    }

    println!("Original Bill ₦{}", bill);
    println!("Discount Applied {}%", discount);
    println!("Final Bill ₦{}", discount_bill);
}
