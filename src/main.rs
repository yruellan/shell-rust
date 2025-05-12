#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;

enum CmdRes {
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
            CmdRes::Exit => {break},
            _ => {}
        }
    }
}

fn run_cmd(input: String) -> CmdRes {

    let args : Vec<&str> = input.split(' ').map(|x| x.trim() ).collect();
    let nargs = args.len() ;

    if nargs == 0 {
        return CmdRes::Ok ;
    }
    let cmd = args[0] ;

    if cmd == "exit"{
        if nargs >= 2 && args[1] == "0" {return CmdRes::Exit;}
        else {return CmdRes::Error;}
    } else if cmd == "echo" {
        for i in 1..nargs {
            print!("{} ", args[i].trim());
        }
        println!("");
        return CmdRes::Ok;
    } else if cmd == "type" {
        
        if nargs < 2 {return CmdRes::Error}
        
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {

                    if let Result::Ok(files) = fs::read_dir(path) {
                    for file in files {

                        match file {
                            Result::Ok(file_path) 
                                if file_path.path().file_name().unwrap() == args[1] 
                            => {
                                let path_name = file_path.path()
                                    .into_os_string()
                                    .to_owned();
                                println!("{} is {}", args[1], path_name.to_str().unwrap());
                                return CmdRes::Ok;
                            },
                            _ => {}
                        };
                    }}
                }

                println!("{}: not found", args[1]);
                return CmdRes::Error;
            }
            None => {
                println!("PATH is not defined in the environment.");
                return CmdRes::Error;
            }
        }

    } else {
        println!("{}: command not found", cmd.trim());
        return CmdRes::CommandNotFound;
    }
    
}