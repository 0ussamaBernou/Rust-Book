use std::io;

fn main() {
    // Get the temperature
    println!("Enter the temperature:");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("couldn't read line!");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Error parsing"),
    };
    dbg!(temp);

    // Get the unit
    println!("Enter the unit('f' for Fahrenheit of 'c' for Celsius):");
    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("couldn't read line!");

    let unit = unit.trim().to_lowercase();
    dbg!(&unit);

    if unit == "f" {
        println!("temperature in Celsius is : {}°c", f_to_c(temp));
    } else if unit == "c" {
        println!("temperature in Fahrenheit is : {}°f", c_to_f(temp));
    } else {
        println!("'{unit}' unit not supported!");
    }
}

fn f_to_c(temp: f32) -> f32 {
    return (temp - 32.0) * (5.0 / 9.0);
}

fn c_to_f(temp: f32) -> f32 {
    return (temp * (5.0 / 9.0)) + 32.0;
}
