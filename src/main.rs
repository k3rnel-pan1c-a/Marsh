mod builtins;

use builtins::*;
use std::process::{Command, Output};
use std::{env, fs, io, io::Write, path::Path, str::FromStr};

const PROMPT_CHAR: &str = "->";
struct Cmd {
    keyword: String,
    args: Vec<String>,
}

enum Builtin {
    Cd,
    Pwd,
    Echo,
}
impl FromStr for Builtin {
    type Err = ();
    // check if the passed command is a built-in
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "pwd" => Ok(Builtin::Pwd),
            "cd" => Ok(Builtin::Cd),
            _ => Err(()),
        }
    }
}

fn main() {
    //read the environment variables
    //need to add a fn to read a dotfile
    //cd to the root directory on start
    let root = Path::new(env::var("HOME").unwrap().as_str());
    env::set_current_dir(&root).expect("'/Users/anasbadr' doesn't exist");

    //since shells are REPL, then we'll have an infinite loop
    loop {
        print_prompt_char();
        let cmd = tokenize_cmd(read_cmd());
        process_cmd(cmd)
    }
}

fn print_prompt_char() {
    print!("{} ", PROMPT_CHAR);
    io::stdout().flush().unwrap();
}

fn read_cmd() -> String {
    let mut cmd: String = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read the entered command");
    cmd
}

fn tokenize_cmd(cmd: String) -> Cmd {
    // if cmd.contains("|"){
    //     let mut cmd_args: Vec<String> = cmd.split("|")
    //         .map(|item| item.to_string())
    //         .collect();
    //     //I need to take the result of the first command and use it with the next one
    //     tokenize_cmd(cmd_args[0])
    // }

    let mut cmd_args: Vec<String> = cmd
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();
    Cmd {
        keyword: cmd_args.remove(0),
        args: cmd_args,
    }

}

fn process_cmd(cmd: Cmd) -> () {
    match Builtin::from_str(&*cmd.keyword) {
        Ok(Builtin::Echo) => builtin_echo(cmd.args),
        Ok(Builtin::Cd) => builtin_cd(cmd.args),
        Ok(Builtin::Pwd) => builtin_pwd(cmd.args),
        Err(_) => {
            external_cmd(cmd);
        }
    }
}

fn external_cmd(cmd: Cmd)  -> Output{
    let output = Command::new(cmd.keyword)
        .args(cmd.args)
        .output()
        .expect("TODO");
    //the output method returns a vector of bytes as an output so we use the from_utf8_lossy method
    //to convert to a String object
    print!("{}", String::from_utf8_lossy(&output.stdout));
    io::stdout().flush().unwrap();
    output
}

fn piping(cmd_args: Vec<String>) {}