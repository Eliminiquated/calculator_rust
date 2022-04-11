use std::io::{stdin, stdout, Write};
use std::{thread, time};

fn main() {
    println!("Welcome to Frank's Calculator!");
    thread::sleep(time::Duration::from_secs(2));
    println!(
        "You may choose from the following calculator options by entering the phrase or the number:\n\
    * '1' OR 'Basic Operators'\n\
    * '2' OR 'Quadratic Equation'"
    );
    thread::sleep(time::Duration::from_secs(3));
    let mut option = String::new();
    println!("Which option would you like to use?\n");
    let _ = stdout().flush();
    stdin().read_line(&mut option).expect("");
    if option.trim() == "Basic Operators" || option.trim() == "1" {
        println!("You have chosen 'Basic Operators'");
        thread::sleep(time::Duration::from_millis(1500));
        println!(
            "The available operators are:\n\
        * 'Addition' OR '+'\n\
        * 'Subtraction' OR '-'\n\
        * 'Multiplication' OR '*'\n\
        * 'Division' OR '/'\n"
        );
        let mut operator = String::new();
        let _ = stdout().flush();
        println!("What operator would you like to use?\n");
        stdin().read_line(&mut operator).expect("");
        if operator.trim() == "Addition" || operator.trim() == "+" {
            println!("You have chosen 'Addition'");
            thread::sleep(time::Duration::from_millis(1500));
            let mut firstnumber = String::new();
            println!("What is your first number?: ");
            let _ = stdout().flush();
            stdin().read_line(&mut firstnumber).expect("");
            if let Some('\n') = firstnumber.chars().next_back() {
                firstnumber.pop();
                let mut secondnumber = String::new();
                println!("What is your second number?: ");
                let _ = stdout().flush();
                stdin().read_line(&mut secondnumber).expect("");
                if let Some('\n') = secondnumber.chars().next_back() {
                    let firstfromstring: i32 = firstnumber.trim().parse().unwrap();
                    let secondfromstring: i32 = secondnumber.trim().parse().unwrap();
                    let calculation = firstfromstring + secondfromstring;
                    println!("The answer is: {}!\n", calculation);
                    let mut quit = String::new();
                    println!("Press enter to quit!");
                    let _ = stdout().flush();
                    stdin().read_line(&mut quit).expect("");
                    if quit.trim() == "" {
                        std::process::exit(0);
                    }
                }
            }
        } else if operator.trim() == "Subtraction" || operator.trim() == "-" {
            println!("You have chosen 'Subtraction'");
            thread::sleep(time::Duration::from_millis(1500));
            let mut firstnumber = String::new();
            println!("What is your first number?: ");
            let _ = stdout().flush();
            stdin().read_line(&mut firstnumber).expect("");
            if let Some('\n') = firstnumber.chars().next_back() {
                firstnumber.pop();
                let mut secondnumber = String::new();
                println!("What is your second number?: ");
                let _ = stdout().flush();
                stdin().read_line(&mut secondnumber).expect("");
                if let Some('\n') = secondnumber.chars().next_back() {
                    let firstfromstring: i32 = firstnumber.trim().parse().unwrap();
                    let secondfromstring: i32 = secondnumber.trim().parse().unwrap();
                    let calculation = firstfromstring - secondfromstring;
                    println!("The answer is: {}!\n", calculation);
                    thread::sleep(time::Duration::from_secs(2));
                    let mut quit = String::new();
                    println!("Press enter to quit!");
                    let _ = stdout().flush();
                    stdin().read_line(&mut quit).expect("");
                    if quit.trim() == "" {
                        std::process::exit(0);
                    }
                }
            }
        } else if operator.trim() == "Multiplication" || operator.trim() == "*" {
            println!("You have chosen 'Multiplication'");
            thread::sleep(time::Duration::from_millis(1500));
            let mut firstnumber = String::new();
            println!("What is your first number?: ");
            let _ = stdout().flush();
            stdin().read_line(&mut firstnumber).expect("");
            if let Some('\n') = firstnumber.chars().next_back() {
                firstnumber.pop();
                let mut secondnumber = String::new();
                println!("What is your second number?: ");
                let _ = stdout().flush();
                stdin().read_line(&mut secondnumber).expect("");
                if let Some('\n') = secondnumber.chars().next_back() {
                    let firstfromstring: i32 = firstnumber.trim().parse().unwrap();
                    let secondfromstring: i32 = secondnumber.trim().parse().unwrap();
                    let calculation = firstfromstring * secondfromstring;
                    println!("The answer is: {}!\n", calculation);
                    thread::sleep(time::Duration::from_secs(2));
                    let mut quit = String::new();
                    println!("Press enter to quit!");
                    let _ = stdout().flush();
                    stdin().read_line(&mut quit).expect("");
                    if quit.trim() == "" {
                        std::process::exit(0);
                    }
                }
            }
        } else if operator.trim() == "Division" || operator.trim() == "/" {
            println!("You have chosen 'Division'");
            thread::sleep(time::Duration::from_millis(1500));
            let mut firstnumber = String::new();
            println!("What is your first number?: ");
            let _ = stdout().flush();
            stdin().read_line(&mut firstnumber).expect("");
            if let Some('\n') = firstnumber.chars().next_back() {
                firstnumber.pop();
                let mut secondnumber = String::new();
                println!("What is your second number?: ");
                let _ = stdout().flush();
                stdin().read_line(&mut secondnumber).expect("");
                if let Some('\n') = secondnumber.chars().next_back() {
                    let firstfromstring: i32 = firstnumber.trim().parse().unwrap();
                    let secondfromstring: i32 = secondnumber.trim().parse().unwrap();
                    let calculation = firstfromstring / secondfromstring;
                    println!("The answer is: {}!\n", calculation);
                    thread::sleep(time::Duration::from_secs(2));
                    let mut quit = String::new();
                    println!("Press enter to quit!");
                    let _ = stdout().flush();
                    stdin().read_line(&mut quit).expect("");
                    if quit.trim() == "" {
                        std::process::exit(0);
                    }
                }
            }
        } else {
            println!("Invalid operator. Calculator closing in 3 seconds.");
            thread::sleep(time::Duration::from_secs(3));
            std::process::exit(0);
        }
    } else if option.trim() == "Quadratic Equation" || option.trim() == "2" {
        println!("You have chosen 'Quadratic Equation'");
        thread::sleep(time::Duration::from_millis(1500));
        let mut a = String::new();
        println!("What is the value of a?: ");
        let _ = stdout().flush();
        stdin().read_line(&mut a).expect("");
        if let Some('\n') = a.chars().next_back() {
            let mut b = String::new();
            println!("What is the value of b?: ");
            let _ = stdout().flush();
            stdin().read_line(&mut b).expect("");
            if let Some('\n') = a.chars().next_back() {
                let mut c = String::new();
                println!("What is the value of c?: ");
                let _ = stdout().flush();
                stdin().read_line(&mut c).expect("");
                if let Some('\n') = c.chars().next_back() {
                    let atoint: f32 = a.trim().parse().unwrap();
                    let btoint: f32 = b.trim().parse().unwrap();
                    let ctoint: f32 = c.trim().parse().unwrap();

                    let x = -btoint
                        + f32::sqrt(btoint * btoint - 4.0 * atoint * ctoint) / (2.0 * atoint);
                    let y = -btoint
                        - f32::sqrt(btoint * btoint - 4.0 * atoint * ctoint) / (2.0 * atoint);
                    println!("The values are: {} and {}", x, y);
                }
            }
        }
    } else {
        println!("Invalid option. Calculator closing in 3 seconds.");

        thread::sleep(time::Duration::from_secs(3));
        std::process::exit(0);
    }
}
