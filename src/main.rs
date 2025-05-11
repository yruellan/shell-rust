#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run_cmd(input);
    }
}

fn run_cmd(cmd: String){

    println!("{}: command not found", cmd.trim());
}