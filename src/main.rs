#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::Command;

enum CmdRes {
    Exit,
    Ok,
    Error
}

enum BuiltinCmd {
    Echo,
    Exit,
    Type,
    Pwd,
}

enum CmdType {
    CommandNotFound,
    ShellBuiltin(BuiltinCmd),
    Path(String,String),
    Void
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

fn find_cmd(cmd : &str) ->  CmdType{
    match cmd {
        "" => return CmdType::Void,
        "echo" => return CmdType::ShellBuiltin(BuiltinCmd::Echo),
        "exit" => return CmdType::ShellBuiltin(BuiltinCmd::Exit),
        "type" => return CmdType::ShellBuiltin(BuiltinCmd::Type),
        "pwd" => return CmdType::ShellBuiltin(BuiltinCmd::Pwd),
        _ => {}
    }
    
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {

                if let Result::Ok(files) = fs::read_dir(path) {
                for file in files {

                    match file {
                        Result::Ok(file_path) 
                            if file_path.path().file_name().unwrap() == cmd 
                        => {
                            let path_name = file_path.path()
                                .into_os_string()
                                .to_owned();
                            
                            return CmdType::Path(
                                cmd.to_owned(),
                                path_name.to_str().unwrap().to_owned()
                            );
                        },
                        _ => {}
                    };
                }}
            }

            
            return CmdType::CommandNotFound ;
        }
        None => {
            println!("PATH is not defined in the environment.");
            return CmdType::CommandNotFound ;
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

    match find_cmd(args[0]) {
        CmdType::Void => {
            return CmdRes::Ok;
        }
        CmdType::CommandNotFound => {
            println!("{}: command not found", cmd.trim());
            return CmdRes::Error;
        }
        CmdType::ShellBuiltin(BuiltinCmd::Exit) => {
            if nargs >= 2 && args[1] == "0" {return CmdRes::Exit;}
            else {return CmdRes::Error;}
        }
        CmdType::ShellBuiltin(BuiltinCmd::Echo) => {
            for i in 1..nargs {
                print!("{} ", args[i].trim());
            }
            println!("");
            return CmdRes::Ok;
        }
        CmdType::ShellBuiltin(BuiltinCmd::Type) => {
            if nargs < 2 {return CmdRes::Error}

            match find_cmd(args[1]) {
                CmdType::Void => println!("{}: not found", args[1]),
                CmdType::CommandNotFound => println!("{}: not found", args[1]),
                CmdType::ShellBuiltin(_) => println!("{} is a shell builtin", args[1]),
                CmdType::Path(name,path_name) => println!("{} is {}", name, path_name)
            }

            return CmdRes::Ok;
        }
        CmdType::ShellBuiltin(BuiltinCmd::Pwd) => {
            let mut cmd = Command::new("pwd") ;
            for arg in args[1..].iter() {
                cmd.arg(arg);
            }
            cmd.status()
                .expect("failed to execute process");

            return CmdRes::Ok;
        }
        CmdType::Path(name,_) => {

            let mut cmd = Command::new(name) ;
            for arg in args[1..].iter() {
                cmd.arg(arg);
            }
            cmd.status()
                .expect("failed to execute process");

            return CmdRes::Ok;
        }
    }
    
}