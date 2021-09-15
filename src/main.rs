use std::process::Command;
use std::process::exit;
use std::path::Path;
use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    rl.load_history(".krush_history").ok();
    loop {
        let prompt = &format!("{} $ ", env::current_dir().unwrap().display()).as_str().replace(env::var("HOME").unwrap().as_str(), "~");
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let cmd = line.split_whitespace().next().unwrap();
                let cmd_ = cmd.clone();
                let args: Vec<_> = line[cmd.len()..line.len()].split_whitespace().collect();
                let args_ = args.clone();
                if let Ok(mut child) = Command::new(cmd).args(args).spawn() {
                    child.wait().unwrap();
                } else {
                    match cmd_ {
                        "cd" => {
                            if args_.len() < 1 || args_.len() > 1 {
                                eprintln!("cd: expected 1 argument, got {} argument", args_.len());
                            } else {
                                if let Ok(_) = env::set_current_dir(args_[0]) {
                                    print!("");
                                } else {
                                    if Path::new(args_[0]).is_file() {
                                        eprintln!("{}: not a directory", args_[0])
                                    } else if !Path::new(args_[0]).is_dir() {
                                        eprintln!("{}: no such directory", args_[0])
                                    }
                                }
                            }
                        },
                        "exit" => {
                            if args_.len() < 1 || args_.len() > 1 {
                                break
                            } else {
                                if args_[0].parse::<f64>().is_ok() {
                                    exit(args_[0].parse().unwrap());
                                } else {
                                    eprintln!("exit: not a number");
                                }
                            }
                        },
                        _ => eprintln!("{}: command not found", cmd_),
                    }
                }
            },
            Err(ReadlineError::Interrupted) => eprint!(""),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(".krush_history").unwrap();
}

