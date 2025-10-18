use std::io;

fn main() {
    println!("Welcome to the Smart Temperature Coverter");
    let unit = input("Select the Coversion option\n1)Celsius to Farenheit\n2)Farenheit to Celsius");
    let temp = input("Input temperature");
    let result = match unit.trim() {
        "1" => covert_celsius(temp),
        "2" => covert_farenheit(temp),
        _ => String::new(),
    };
    println!("Coverted Temperature {}", result)
}

fn covert_celsius(temp: String) -> String {
    let celsius = temp
        .trim()
        .parse::<f32>()
        .expect("Failed to parse input.Input is not a valid number");
    return format!("{}°F", (9.0 / 5.0 * celsius) + 32.0);
}

fn covert_farenheit(temp: String) -> String {
    let farenheit = temp
        .trim()
        .parse::<f32>()
        .expect("Failed to parse input.Input is not a valid number");
    return format!("{}°C", (farenheit - 32.0) * 5.0 / 9.0);
}

fn input(que: &str) -> String {
    println!("{}", que);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input;
}
