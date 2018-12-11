extern crate rsc;

use std::io::prelude::*;
use std::process::Command;

use rsc::computer::*;
use rsc::lexer::*;
use rsc::parser::*;

fn main() {
    let mut buffer = String::new();

    loop {
        print!("y=");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_owned();

        if &buffer[..] == "quit" || &buffer[..] == "exit" {
            break;
        }

        let mut y_vals = Vec::<f64>::new(); // 20 values
        for x in -10..=10 {
            match tokenize(&buffer) {
                Ok(tokens) => {
                    match parse(&tokens) {
                        Ok(mut ast) => {
                            ast.replace(
                                &Expr::Identifier(String::from("x")),
                                &Expr::Constant(x as f64),
                                false,
                            );
                            y_vals.push(compute(&ast));
                            //println!("{}", compute(&ast));
                        }
                        Err(err) => {
                            println!("Parser error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Lexer error: {:?}", err);
                }
            }
        }

        println!("x: {:?}", (-10..=10).collect::<Vec<i32>>());
        println!("y: {:?}", y_vals);

        Command::new("python")
            .args(&[
                "plotxy.py",
                &format_numbers(
                    &(-10..=10)
                        .collect::<Vec<i32>>()
                        .iter()
                        .map(|&n| n as f64)
                        .collect::<Vec<f64>>(),
                ),
                &format_numbers(&y_vals),
            ])
            .spawn()
            .expect("failed to execute process");

        buffer = String::new();
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
