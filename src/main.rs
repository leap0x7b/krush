use std::process::Command;
use std::process::exit;
use std::path::Path;
use std::env;
use unescape::unescape;
use shell_words;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn prompt(prompt: &str) -> String {
    let cwd = &format!("{}", env::current_dir().unwrap().display()).as_str().replace(env::var("HOME").unwrap().as_str(), "~");
    return prompt
        .replace("{cwd}", cwd)
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let mut prompt_ = String::from("\x1b[0;33m{cwd}\x1b[0m $ ");
    rl.load_history(&format!("{}/.krush_history", env::var("HOME").unwrap().as_str())).ok();
    loop {
        let readline = rl.readline(&prompt(&prompt_));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let cmd = line.split_whitespace().next().unwrap();
                let cmd_ = cmd.clone();
                let args: Vec<_> = shell_words::split(&line[cmd.len()..line.len()]).unwrap();
                let args: Vec<_> = args.iter().map(|s| s as &str).collect();
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
                        "prompt" => prompt_ = String::from(unescape(&args_.join(" ")).unwrap()),
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
    rl.save_history(&format!("{}/.krush_history", env::var("HOME").unwrap().as_str())).unwrap();
}

