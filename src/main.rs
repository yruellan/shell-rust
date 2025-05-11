#[allow(unused_imports)]
use std::io::{self, Write};

enum Result {
    CommandNotFound,
    Exit
}

fn main() {
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match run_cmd(input) {
            Result::Exit => {break},
            _ => {}
        }
    }
}

fn run_cmd(cmd_: String) -> Result {

    let cmd = cmd_.trim();

    // println!("Running command: {}, ({})", cmd, cmd == "exit 0");

    match cmd {
        "exit 0" => {
            println!("Exiting...");
            return Result::Exit;
        },
        _ => {
            println!("{}: command not found", cmd.trim());
            return Result::CommandNotFound;
        }
    }
}