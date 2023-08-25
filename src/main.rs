mod builtins;

use builtins::*;
use std::process::{Command, Output};
use std::{env, io, io::Write, path::Path, str::FromStr};

const PROMPT_CHAR: &str = "->";
struct Cmd {
    keyword: String,
    args: Vec<String>,
    builtin: Option<Builtin>,
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
    let home_dir: String = env::var("HOME").unwrap();
    //read the environment variables
    //need to add a fn to read a dotfile
    //cd to the root directory on start
    let root = Path::new(home_dir.as_str());
    env::set_current_dir(&root).expect(format!("{home_dir} doesn't exist").as_str());

    //since shells are REPL, then we'll have an infinite loop
    loop {
        print_prompt_char();
        let cmd = tokenize_cmd(read_cmd());
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

fn tokenize_cmd(cmd: String) -> Vec<String> {
    if cmd.contains("|"){

        let mut iterator = cmd.split("|")
            .map(|slice| slice.to_string());

        let first_cmd_output = pipe(parse_cmd( tokenize_cmd(iterator.next().unwrap())));
        tokenize_cmd(iterator.next().unwrap() + &*first_cmd_output)

    }
    else{
        let mut cmd_args: Vec<String> = cmd
            .split_whitespace()
            .map(|item| item.to_string())
            .collect();
        cmd_args
    }
}

fn parse_cmd(mut cmd_args: Vec<String>) -> Cmd{
    let mut cmd: Cmd =
        Cmd {
            keyword: cmd_args.remove(0),
            args: cmd_args,
            builtin: None,
        };
    match Builtin::from_str(&*cmd.keyword) {
        Ok(Builtin::Echo) => {
            cmd.builtin = Some(Builtin::Echo)
        },
        Ok(Builtin::Cd) => {
            cmd.builtin = Some(Builtin::Cd)
        },
        Ok(Builtin::Pwd) => {
            cmd.builtin = Some(Builtin::Pwd)
        },
        Err(_) => {
            cmd.builtin = None
        }

    }
    cmd
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
fn pipe(first_cmd: Cmd) -> String{
    //piping process the first command and returns the output to be used as an arg for the second command
    String::from_utf8_lossy(&*external_cmd(first_cmd).stdout).parse().unwrap()
}