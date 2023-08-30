use std::io::stdin;

fn main() {
    let a = first_value();
    let b = second_value();

    menu(a, b);
}

fn plus(a: f64, b: f64) {
    let res = a + b;
    println!("{a} + {b} = {res}")
}

fn minus(a: f64, b: f64) {
    let res = a - b;
    println!("{a} - {b} = {res}");
}

fn divide(a: f64, b: f64) {
    let res = a / b;
    println!("{a} / {b} = {res}")
}

fn multiply(a: f64, b: f64) {
    let res = a * b;
    println!("{a} * {b} = {res}")
}

fn first_value() -> f64 {
    println!("Type the first value: ");
    loop {
        let mut a = String::new();
        stdin()
            .read_line(&mut a)
            .expect("Failed to read from stdin");

        let a: f64 = match a.trim().parse::<f64>() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Please, input a number");
                continue;
            }
        };
        return a;
    }
}

fn second_value() -> f64 {
    println!("Type the second value: ");
    loop {
        let mut b = String::new();
        stdin()
            .read_line(&mut b)
            .expect("Failed to read from stdin");

        let b: f64 = match b.trim().parse::<f64>() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Please, input a number");
                continue;
            }
        };
        return b;
    }
}

fn menu(a: f64, b: f64) {
    println!(" - - - Menu - - - ");
    println!("0. Exit");
    println!("1. Plus");
    println!("2. Minus");
    println!("3. Multiply");
    println!("4. Divide");
    println!("");
    println!("Select an option: ");

    loop {
        let mut option = String::new();
        stdin().read_line(&mut option).expect("Not a number");
        let option = option.trim();

        match option {
            "0" => println!("Exiting..."),
            "1" => plus(a, b),
            "2" => minus(a, b),
            "3" => multiply(a, b),
            "4" => divide(a, b),
            _ => {
                println!("Input a valid value");
                continue;
            }
        }
        break;
    }
}
