extern crate rsc;

use std::io::prelude::*;
use std::process::Command;

use rsc::computer::*;
use rsc::lexer::*;
use rsc::parser::*;

fn main() {
    // Properties
    let mut x_min: f64 = -10.;
    let mut x_max: f64 = 10.;
    let mut step: f64 = 0.5;

    let mut reading_computations = false;

    loop {
        let mut buffer = String::new();

        if reading_computations {
            print!("y=");
        } else {
            print!(">");
        }

        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            if reading_computations {
                reading_computations = false;
                continue;
            } else {
                break;
            }
        }

        if !reading_computations {
            if buffer.starts_with("xmin=") {
                match rsc::eval(&buffer[5..]) {
                    Ok(val) => x_min = val,
                    Err(e) => println!("Failed to set xmin: {:?}", e),
                }
            } else if buffer.starts_with("xmax=") {
                match rsc::eval(&buffer[5..]) {
                    Ok(val) => x_max = val,
                    Err(e) => println!("Failed to set xmax: {:?}", e),
                }
            } else if buffer.starts_with("step=") {
                match rsc::eval(&buffer[5..]) {
                    Ok(val) => step = val,
                    Err(e) => println!("Failed to set step: {:?}", e),
                }
            } else {
                match &buffer[..] {
                    "help" | "h" => {
                        println!("Commands: quit,exit,begin\nOptions: xmin=,xmax=,step=")
                    }
                    "start" | "begin" => reading_computations = true,
                    _ => println!("Unrecognized command {:?}: try 'help'", buffer),
                }
            }
        } else {
            let x_vals = step_iter(x_min, x_max, step);
            let mut y_vals = Vec::<f64>::new();

            let ast: Expr;
            match tokenize(&buffer) {
                Ok(tokens) => match parse(&tokens) {
                    Ok(expr) => ast = expr,
                    Err(err) => {
                        println!("Parser error: {:?}", err);
                        continue;
                    }
                },
                Err(err) => {
                    println!("Lexer error: {:?}", err);
                    continue;
                }
            }
            println!("ast: {:?}", &ast);

            for &x in &x_vals {
                let mut tmp = ast.clone();
                tmp.replace(
                    &Expr::Identifier(String::from("x")),
                    &Expr::Constant(x as f64),
                    false,
                );
                y_vals.push(compute(&tmp));
            }

            println!("x: {:?}", x_vals);
            println!("y: {:?}", y_vals);

            Command::new("python")
                .args(&[
                    "plotxy.py",
                    &format_numbers(&x_vals),
                    &format_numbers(&y_vals),
                ])
                .spawn()
                .expect("failed to execute process");
        }
    }
}

fn format_numbers(numbers: &[f64]) -> String {
    let mut out = String::new();
    let mut commas = numbers.len() - 1;
    for n in numbers {
        out.push_str(&n.to_string());
        if commas > 0 {
            out.push(',');
            commas -= 1;
        }
    }
    out
}

fn step_iter(min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut items = vec![min];
    let mut n = min;
    while n + step <= max {
        n += step;
        items.push(n);
    }
    items
}
