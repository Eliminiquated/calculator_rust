use std::io::{stdin, stdout, Write};
use std::{thread, time};

fn main() {
    println!("Welcome to Frank's Calculator!");
    thread::sleep(time::Duration::from_secs(2));
    println!(
        "You may choose from the following calculator options by entering the phrase or the number:\n\
    * '1' OR 'Basic Operators'\n\
    * '2' OR 'Algebraic Equations'"
    );
    thread::sleep(time::Duration::from_secs(1));
    let mut option = String::new();
    println!("Which option would you like to use?");
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
    } else if option.trim() == "Algebraic Equations" || option.trim() == "2" {
        println!("You have chosen 'Algebraic Equations'");
        thread::sleep(time::Duration::from_millis(1500));
        let mut equation=String::new();
        println!("The avaliable equations are:\n\
        * '1' OR 'Linear Equation'\n\
        * '2' OR 'Quadratic Equation'\n\
        * '3' OR 'Circular Equation'\n
        ");
        thread::sleep(time::Duration::from_secs(1));
        println!("Which equation do you want?: ");
        let _= stdout().flush();
        stdin().read_line(&mut equation).expect("");
        if equation.trim() == "Linear Equation" || equation.trim() == "1"{
            println!("You have chosen 'Linear Equation'");
            thread::sleep(time::Duration::from_millis(1500));
            println!("You may choose from the follow Linear Options by typing the corresponding number:\n\
            * '1' 'Find the slope and y-intercept'\n\
            * '2' 'Create equation with slope and y-intercept'");
            let mut linearoption=String::new();
            println!("Which option would you like to use?: ");
            let _=stdout().flush();
            stdin().read_line(&mut linearoption).expect("");
            if linearoption.trim() == "1"{
                println!("You have chosen 'Find the slope and y intercept'");
                thread::sleep(time::Duration::from_millis(1500));
                println!("Every Linear Equation has a slope and y-intercept. To find them, you must locate where they are in the equation.");
                thread::sleep(time::Duration::from_secs(3));
                println!("What number is right next to the x in the equation?\n\
                Example: y = 2x + 5\n\
                             ^");
            }else if linearoption.trim() == "2" {

            }else{
                println!("Invalid Linear Option. Calculator closing in 3 seconds!");
                thread::sleep(time::Duration::from_secs(3));
                std::process::exit(0);
            }

        }else if equation.trim() == "Quadratic Equation" || equation.trim() == "2" {
            println!("You have chosen 'Quadratic Equation'");
            thread::sleep(time::Duration::from_secs(2));
            println!("You may choose from the following Quadratic Options by typing the corresponding number:\n\
        * '1' Find the value of x from the Quadratic Formula\n");
            thread::sleep(time::Duration::from_secs(1));
            let mut quadraticoption=String::new();
            println!("Which option would you like?: ");
            let _= stdout().flush();
            stdin().read_line(&mut quadraticoption).expect("");
            if quadraticoption.trim() == "1" {
                println!("You have chosen 'Find the value of x from the Quadratic Formula'");
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
                            let delta: f32 = (btoint * btoint) - 4.0 * atoint * ctoint;
                            let testx = ((-1.0 * btoint) + (delta as f32).sqrt() as f32) / 2.0 * atoint;
                            let testy = ((-1.0 * btoint) - (delta as f32).sqrt() as f32) / 2.0 * atoint;
                            if testx.is_nan() || testy.is_nan() {
                                println!("There are no real solutions!");
                                thread::sleep(time::Duration::from_secs(2));
                                let mut exit=String::new();
                                println!("Press ENTER to exit!");
                                let _= stdout().flush();
                                stdin().read_line(&mut exit).expect("");
                                if exit.trim() == "" {
                                    std::process::exit(0);
                                }else{
                                    std::process::exit(0);
                                }
                            } else {
                                println!("The values are: {} and {}", testx, testy);
                            }
                        }
                    }
                }
            }
        }else if equation.trim() == "Circular Equation" || equation.trim() == "3" {

        }else{
            println!("Invalid equation. Calculator closing in 3 seconds!");
            thread::sleep(time::Duration::from_secs(3));
            std::process::exit(0);
        }
    } else {
        println!("Invalid option. Calculator closing in 3 seconds.");

        thread::sleep(time::Duration::from_secs(3));
        std::process::exit(0);
    }
}
