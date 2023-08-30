mod builtins;
mod parser;
mod tokenizer;

use builtins::*;
use parser::*;
use std::process::{Child, Command, Output, Stdio};
use std::{env, io, io::Write, path::Path, str::FromStr};
use tokenizer::*;
const PROMPT_CHAR: &str = "->";
pub struct Cmd {
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
        process_cmd(parse_cmd(cmd.0), cmd.1);
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

fn process_cmd(cmd: Cmd, stdin: Option<Child>) -> () {
    //should pick a better name.
    match cmd.builtin {
        Some(builtin) => match builtin {
            Builtin::Cd => builtin_cd(cmd.args),
            Builtin::Pwd => builtin_pwd(cmd.args),
            Builtin::Echo => builtin_echo(cmd.args),
        },
        None => {
            match stdin {
                None => {
                    let output: Output = Command::new(cmd.keyword)
                        .args(cmd.args)
                        .output()
                        .expect("TODO");
                    //the output method returns a vector of bytes as an output  so we use the from_utf8_lossy method
                    //to convert to a String object
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                Some(output) => {
                    let output: Output = Command::new(cmd.keyword)
                        // .stdin(Stdio::piped())
                        .stdin(output.stdout.unwrap())
                        .args(cmd.args)
                        .output()
                        .expect("TODO");
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    //the output method returns a vector of bytes as an output so we use the from_utf8_lossy method
                    //to convert to a String object
                }
            }
            io::stdout().flush().unwrap();
        }
    }
}
fn pipe(cmds: Vec<Cmd>) -> Option<Child> {
    let mut cmds = cmds.into_iter().peekable();
    let mut previous_cmd_output: Option<Child> = None;
    while let Some(cmd) = cmds.next() {
        let stdin = previous_cmd_output.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
        });
        let stdout = Stdio::piped();
        let output = Command::new(cmd.keyword)
            .args(cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
            .expect("TODO: panic message");

        previous_cmd_output = Some(output);
    }
    previous_cmd_output
    // cmds.next().unwrap()
}
