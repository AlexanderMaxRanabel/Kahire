use std::io::{self, Write};
use std::str::SplitWhitespace;

fn main() {
    /*
    new x 1
    new y = 1
    new add z x y
    new subtract h z y

    print z
    print "hello world"

    if x = 1 do print y else do print "göttingen"
    */
    println!("Welcome to Kahire REPL");

    let mut variable_stack: Vec<f64> = Vec::new();
    let mut variable_access_point: usize = 0;

    loop {
        let mut statement = String::new();
        if let Err(err) = io::stdin().read_line(&mut statement) {
            eprintln!("Error reading input: {}", err);
            continue;
        }
        let statement = statement.trim();
        let tokens: Vec<&str> = statement.split_whitespace().collect();

        match tokens[0] {
            "print" => {
                if tokens.len() < 2 {
                    eprintln!("Not sufficent tokens");
                    continue;
                } else {
                    for word in &tokens[1..] {
                        println!("{}", word);
                    }
                }
            },

            "new" => {
                if tokens.len() < 3 {
                    eprintln!("Not sufficent tokens");
                    continue;
                } else {
                    if tokens[1] == "+" || tokens[1] == "-" || tokens[1] == "*" || tokens[1] == "/"{
                        if tokens.len() < 5 {
                            eprintln!("Not enough tokens");
                            continue;
                        }

                        if tokens[1] == "+" {
                            if let Ok(num1) = tokens[3].parse::<f64>() {
                                if let Ok(num2) = tokens[4].parse::<f64>() {
                                    let sum = num1 + num2;
                                    println!("{}", sum);
                                } else {
                                    println!("Syntax error: Invalid variable value");
                                }
                            } else {
                                println!("Syntax error: Invalid variable value");
                            }
                        } else if tokens[1] == "-" {
                            if let Ok(num1) = tokens[3].parse::<f64>() {
                                if let Ok(num2) = tokens[4].parse::<f64>() {
                                    let sub = num1 - num2;
                                    println!("{}", sub);
                                } else {
                                    println!("Syntax error: Invalid variable value");
                                }
                            } else {
                                println!("Syntax error: Invalid variable value");
                            }
                        } else if tokens[1] == "*" {
                            if let Ok(num1) = tokens[3].parse::<f64>() {
                                if let Ok(num2) = tokens[4].parse::<f64>() {
                                    let result = num1 * num2;
                                    println!("{}", result);
                                } else {
                                    println!("Syntax error: Invalid variable value");
                                }
                            } else {
                                println!("Syntax error: Invalid variable value");
                            }
                        } else if tokens[1] == "/" {
                            if let Ok(num1) = tokens[3].parse::<f64>() {
                                if let Ok(num2) = tokens[4].parse::<f64>() {
                                    let division = num1 / num2;
                                    println!("{}", division);
                                } else {
                                    println!("Syntax error: Invalid variable value");
                                }
                            } else {
                                println!("Syntax error: Invalid variable value");
                            }
                        }
                    } else {
                        if let Ok(variable) = tokens[2].parse::<f64>() {
                            variable_stack.push(variable);
                            variable_access_point += 1;
                            println!("{}", variable_stack[variable_access_point - 1]);
                        } else {
                            println!("Syntax error: Invalid variable value");
                        }
                    }
                }
            },
            _ => println!("Error: Unknown keyword: {:?}", tokens[0]),
        }
    }
}