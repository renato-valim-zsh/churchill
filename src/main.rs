use std::collections::HashMap;
use std::env as std_env;
use std::fs;

use churchill::ast::Expr;
use churchill::evaluator::{evaluate, expand};
use churchill::parser::parse;
use churchill::utils::is_valid_ident;

/// Runs program in file mode: reads expressions and definitions from a file.
fn run_file_mode(filename: &str, definitions: &mut HashMap<String, Expr>) {
    let Ok(content) = fs::read_to_string(filename) else {
        eprintln!("Failed to read file");
        return;
    };
    for raw in content.lines() {
        let uncommented = if let Some(pos) = raw.find('#') {
            &raw[..pos]
        } else {
            raw
        };
        let line = uncommented.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(pos) = line.find('=') {
            let (lhs, rhs) = line.split_at(pos);
            let name = lhs.trim();
            let expr_str = &rhs[1..].trim();
            if is_valid_ident(name) {
                // This unwrap is safe beacuse we are checking
                // if the ident is valid first.
                let expr = parse(expr_str).unwrap();
                let def = expand(&expr, definitions);
                definitions.insert(name.to_string(), def);
                continue;
            }
        }
        let expr = match parse(line) {
            Ok(expr) => expr,
            Err(err) => {
                eprintln!("Failed to parse expr: {}", err);
                return;
            }
        };
        let expr = expand(&expr, definitions);
        let normal_form = match evaluate(&expr) {
            Ok(nf) => nf,
            Err(err) => {
                eprintln!("Failed to evaluate expr: {}", err);
                return;
            }
        };

        println!("{}", normal_form);
    }
}

/// Runs program in expression mode: evaluates a single expression from command-line arguments.
fn run_expr_mode(args: &[String], definitions: &mut HashMap<String, Expr>) {
    let mut input_raw = args.join(" ");
    if let Some(pos) = input_raw.find('#') {
        input_raw.truncate(pos);
    }
    let input = input_raw.trim();
    if input.is_empty() {
        return;
    }
    let expr = match parse(input) {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!("Failed to parse expr: {}", err);
            return;
        }
    };

    let expr = expand(&expr, definitions);
    let normal_form = match evaluate(&expr) {
        Ok(nf) => nf,
        Err(err) => {
            eprintln!("Failed to evaluate expr: {}", err);
            return;
        }
    };

    println!("{}", normal_form);
}

/// Runs the REPL mode.
fn run_repl(definitions: &mut HashMap<String, Expr>) {
    println!("Churchill REPL ~ lambdas and chilling (type 'exit' or Ctrl+D to quit)");
    loop {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let uncommented = if let Some(pos) = line.find('#') {
                    &line[..pos]
                } else {
                    &line[..]
                };
                let input = uncommented.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" || input == "quit" {
                    break;
                }
                if let Some(pos) = input.find('=') {
                    let (lhs, rhs) = input.split_at(pos);
                    let name = lhs.trim();
                    let rhs = &rhs[1..].trim();
                    if is_valid_ident(name) {
                        match parse(rhs) {
                            Ok(expr) => {
                                let def = expand(&expr, definitions);
                                definitions.insert(name.to_string(), def.clone());
                                println!("Defined {} = {}", name, def);
                            }
                            Err(err) => {
                                eprintln!("Error: {}", err);
                            }
                        }
                        continue;
                    }
                }
                match parse(input) {
                    Ok(expr) => {
                        let expr = expand(&expr, definitions);

                        match evaluate(&expr) {
                            Ok(nf) => println!("{}", nf),
                            Err(err) => eprintln!("Error: {}", err),
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std_env::args().collect();
    let mut definitions: HashMap<String, Expr> = HashMap::new();

    if args.len() >= 3 && args[1] == "--file" {
        run_file_mode(&args[2], &mut definitions);
        return;
    }

    if args.len() > 1 {
        run_expr_mode(&args[1..], &mut definitions);
        return;
    }

    run_repl(&mut definitions);
}
