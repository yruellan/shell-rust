#[allow(unused_imports)]
use std::io::{self, Write};

enum Result {
    CommandNotFound,
    Exit,
    Ok
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

    if cmd == "exit 0" {
        return Result::Exit;
    } else if cmd.starts_with("echo "){
        let args = &cmd[5..] ;
        println!("{}", args) ;
        return Result::Ok;
    } else {
        println!("{}: command not found", cmd.trim());
        return Result::CommandNotFound;
    }
    
}