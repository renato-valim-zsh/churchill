use std::env as std_env;
use std::fs;
use std::collections::HashMap;

use churchill::ast::Expr;
use churchill::parser::parse;
use churchill::evaluator::{evaluate, expand};
use churchill::utils::is_valid_ident;

/// Runs program in file mode: reads expressions and definitions from a file.
fn run_file_mode(filename: &str, definitions: &mut HashMap<String, Expr>) {
    let content = fs::read_to_string(filename).expect("Failed to read file");
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
                let expr = parse(expr_str);
                let def = expand(&expr, &definitions);
                definitions.insert(name.to_string(), def);
                continue;
            }
        }
        let expr = parse(line);
        let expr = expand(&expr, &definitions);
        let normal_form = evaluate(&expr);

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
    let expr = parse(input);
    let expr = expand(&expr, &definitions);
    let normal_form = evaluate(&expr);
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
                        match std::panic::catch_unwind(|| parse(rhs)) {
                            Ok(expr) => {
                                let def = expand(&expr, &definitions);
                                definitions.insert(name.to_string(), def.clone());
                                println!("Defined {} = {}", name, def);
                            }
                            Err(err) => {
                                if let Some(msg) = err.downcast_ref::<&str>() {
                                    println!("Error: {}", msg);
                                } else if let Some(msg) = err.downcast_ref::<String>() {
                                    println!("Error: {}", msg);
                                } else {
                                    println!("Unknown error during parsing");
                                }
                            }
                        }
                        continue;
                    }
                }
                match std::panic::catch_unwind(|| parse(input)) {
                    Ok(expr) => {
                        let expr = expand(&expr, &definitions);
                        let nf = evaluate(&expr);
                        println!("{}", nf);
                    }
                    Err(err) => {
                        if let Some(msg) = err.downcast_ref::<&str>() {
                            println!("Error: {}", msg);
                        } else if let Some(msg) = err.downcast_ref::<String>() {
                            println!("Error: {}", msg);
                        } else {
                            println!("Unknown error during parsing");
                        }
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
