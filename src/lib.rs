mod builtins;

use std::io;
use std::io::{Error, Write};
use std::process::{Child, Command, Output, Stdio};
use std::str::FromStr;
use builtins::*;

const PROMPT_CHAR: &str = "->";

pub struct Cmd {
    pub keyword: String,
    pub args: Vec<String>,
    pub builtin: Option<Builtin>,
}

pub enum Builtin {
    Cd,
    Pwd,
    Echo,
    Exit,
}

impl FromStr for Builtin {
    type Err = ();
    // check if the passed command is a built-in
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "pwd" => Ok(Builtin::Pwd),
            "cd" => Ok(Builtin::Cd),
            "exit" => Ok(Builtin::Exit),
            _ => Err(()),
        }
    }
}


pub fn print_prompt_char() {
    print!("{} ", PROMPT_CHAR);
    io::stdout().flush().unwrap();
}

pub fn pipe(cmds: Vec<Cmd>) -> Option<Child> {
    let mut cmds = cmds.into_iter();
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

pub fn read_cmd() -> String {
    let mut cmd: String = String::new();
    let output = io::stdin()
        .read_line(&mut cmd);

    match output {
        Ok(_) => {}
        Err(err) => {println!("{}", err)}
    }

    cmd
}
fn handle_output(result: Result<Output, Error>) {
    match result {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub fn process_cmd(cmd: Cmd, stdin: Option<Child>) -> () {
    //should pick a better name.
    match cmd.builtin {
        Some(builtin) => match builtin {
            Builtin::Cd => builtin_cd(cmd.args),
            Builtin::Pwd => builtin_pwd(cmd.args),
            Builtin::Echo => builtin_echo(cmd.args),
            Builtin::Exit => builtin_exit(),
        },
        None => {
            match stdin {
                None => {
                    let output: Result<Output, Error> = Command::new(cmd.keyword)
                        .args(cmd.args)
                        .output();
                    handle_output(output);
                }
                Some(output) => {
                    let output: Result<Output, Error> = Command::new(cmd.keyword)
                        .stdin(output.stdout.unwrap())
                        .args(cmd.args)
                        .output();
                    handle_output(output);
                }
            }
            io::stdout().flush().unwrap();
        }
    }
}