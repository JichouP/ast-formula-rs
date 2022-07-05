use ast_formula::calc;
use std::process;

pub fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        match calc(&input) {
            Ok(result) => println!("OK: {}", result),
            Err(_) => println!("Syntax Error!"),
        }
    }
}
