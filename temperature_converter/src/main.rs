use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Enter the temperature value: ");
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Select the input unit of the temperature (C or F): ");
        let mut input_unit = String::new();

        io::stdin()
            .read_line(&mut input_unit)
            .expect("Failed to read line");

        let input_unit = input_unit.trim();

        let (from, to) = match input_unit {
            "F" | "f" => ("Fahrenheit", "Celsius"),
            "C" | "c" => ("Celsius", "Fahrenheit"),
            _ => continue,
        };

        let converted_temperature = if from == "Fahrenheit" {
            (temperature - 32.0) * 5.0 / 9.0
        } else {
            temperature * 9.0 / 5.0 + 32.0
        };

        println!(
            "{} degrees {} is equivalent to {:.2} degrees {}",
            temperature, from, converted_temperature, to
        );

        println!("Do you want to convert another temperature? (Y/N)");
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        if answer.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }
}
