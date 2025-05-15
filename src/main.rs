#[allow(unused_imports)]
use std::io::{self, Write};
use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;

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
    Cd,
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
        "cd" => return CmdType::ShellBuiltin(BuiltinCmd::Cd),
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

fn parse_cmd(cmd_: String) -> Vec<String> {

    let mut args = vec![];
    let mut delimiter = None;
    let mut last_i = 0;

    let cmd = cmd_
        .replace("\'\'", "")
        .replace("\"\"", "");

    for (i,c) in cmd.chars().enumerate() {
        if (c == '\'' || c == '\"') && delimiter == None {
            delimiter = Some(c);
            last_i = i+1 ;
        } else if delimiter == Some(c) {
            args.push(cmd[last_i..i].trim().to_owned());
            delimiter = None ;
            last_i = i+1 ;
        } else if (c == ' ' || c=='\n') && delimiter == None {
            if cmd[last_i..i].trim() != "" {
                args.push(cmd[last_i..i].trim().to_owned());
            }
            last_i = i+1 ;
        }
    }
    // args.retain(|x| x != "");

    // for x in &args {
    //     println!("_{x}_");
    // }

    return args ;
}

fn run_cmd(input: String) -> CmdRes {

    let args = parse_cmd(input);
    let nargs = args.len() ;

    if nargs == 0 {
        return CmdRes::Ok ;
    }
    let cmd = &args[0] ;

    match find_cmd(&args[0]) {
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

            match find_cmd(&args[1]) {
                CmdType::Void => println!("{}: not found", args[1]),
                CmdType::CommandNotFound => println!("{}: not found", args[1]),
                CmdType::ShellBuiltin(_) => println!("{} is a shell builtin", args[1]),
                CmdType::Path(name,path_name) => println!("{} is {}", name, path_name)
            }

            return CmdRes::Ok;
        }
        CmdType::ShellBuiltin(BuiltinCmd::Pwd) => {
            
            let current_dir = env::current_dir().unwrap();
            println!("{}", current_dir.display());

            return CmdRes::Ok;
        }
        CmdType::ShellBuiltin(BuiltinCmd::Cd) => {

            let home = env::var_os("HOME")
                .unwrap()
                .into_string()
                .unwrap();
            let path_str = args[1]
                .to_owned()
                .replace("~", &home) ;
            let new_path = Path::new(path_str.as_str());

            match env::set_current_dir(new_path) {
                Result::Ok(_) => {
                    return CmdRes::Ok;
                }
                Result::Err(_) => {
                    println!("cd: {}: No such file or directory", args[1]);
                    return CmdRes::Error;
                }
            }
            
            
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