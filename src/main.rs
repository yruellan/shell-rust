#[allow(unused_imports)]
use std::io::{self, Write};

enum Result {
    CommandNotFound,
    Exit,
    Ok,
    Error
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

fn run_cmd(input: String) -> Result {

    let args : Vec<&str> = input.split(' ').map(|x| x.trim() ).collect();
    let nargs = args.len() ;

    let valid_cmds = vec![
        "type","echo","exit"
    ];

    if nargs == 0 {
        return Result::Ok ;
    }
    let cmd = args[0] ;

    if cmd == "exit"{
        if nargs >= 2 && args[1] == "0" {return Result::Exit;}
        else {return Result::Error;}
    } else if cmd == "echo" {
        for i in 1..nargs {
            print!("{} ", args[i].trim());
        }
        println!("");
        return Result::Ok;
    } else if cmd == "type" {
        
        if nargs < 2 {return Result::Error}
        else if valid_cmds.contains(&args[1]) {
            println!("{} is a shell builtin", args[1]);
            return Result::Ok;
        } else {
            println!("{}: not found", args[1]);
            return Result::Error;
        }

    } else {
        println!("{}: command not found", cmd.trim());
        return Result::CommandNotFound;
    }
    
}