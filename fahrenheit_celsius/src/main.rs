use std::io;



#[derive(Debug)]
enum Mode {
    F2C,
    C2F
}


fn str_to_mode(mode: &str) -> Mode {
    match mode.trim() {
        "f2c" => Mode::F2C,
        "c2f" => Mode::C2F,
        input => panic!("Unsupported mode: use either 'c2f' or 'f2c', entered: {}", input)
    }
}
fn get_params() -> (Mode, f64) {
    println!("Select the mode [c2f, f2c]: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mode = str_to_mode(&input);
    let number: f64 = loop {
        input = String::new();        
        println!("Insert the number to convert: ");
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading the line, retry...");
            continue;
        }
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue
        };
    };
    println!("You selected the {:?} mode. You want to convert: {}", mode, number);
    (mode, number)
}

fn main() {
    let (mode, number) = get_params();
    let res = match mode {
        Mode::F2C => f2c(number),
        Mode::C2F => c2f(number)
    };
    print!("The result is: {res}")
}

pub fn f2c(number: f64) -> f64{
    (number - 32.0) * 5.0 / 9.0
}

pub fn c2f(number: f64) -> f64{
    (number * 9.0 / 5.0) + 32.0
}


#[cfg(test)]
mod tests {
    use crate::{c2f, f2c};

    #[test]
    fn reverse() {
        assert_eq!(c2f(f2c(42.0)), 42.0)
    }
}
