use std::io;

fn main() {
    println!("Welcome to the EKEDC Smart Meter\nInput your usage(KWh)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let usage = input.trim().parse::<f32>().expect("Input is not a number");
    let mut bill: f32 = 0.0;

    if usage > 100.0 && usage <= 200.0 {
        bill = usage * 25.0;
    } else if usage > 200.0 {
        bill = usage * 30.0;
    } else {
        bill = usage * 20.0;
    }

    println!("Total Bill ₦{}", bill);
}
