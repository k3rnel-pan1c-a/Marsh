use std::{io, io::Write, str::FromStr, env, path::Path};


const PROMPT_CHAR: &str = "->";

struct Command {
    keyword: String,
    args: Vec<String>,
}

enum Builtin{
    Cd,
    Pwd,
    Echo,
}

impl FromStr for Builtin{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "echo" => Ok(Builtin::Echo),
            "pwd" => Ok(Builtin::Pwd),
            "cd" => Ok(Builtin::Cd),
            _ => {Err(()) }
        }
    }
}

fn main() {
    //cd to the root directory on start
    let root = Path::new("/");
    env::set_current_dir(&root).expect("root '/' doesn't exist");

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

fn tokenize_cmd(cmd: String) -> Command {
    let mut cmd_args: Vec<String> = cmd
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();
    Command {
        keyword: cmd_args.remove(0),
        args: cmd_args,
    }
}

fn process_cmd(cmd: Command) -> () {
    match Builtin::from_str(&*cmd.keyword){
        Ok(Builtin::Echo) => {
            builtin_echo(cmd.args)
        }
        Ok(Builtin::Cd) => {
            builtin_cd(cmd.args)
        }
        Ok(Builtin::Pwd) => {
            builtin_pwd(cmd.args)
        }
        Err(_) => {}
    }
}

fn builtin_echo(args: Vec<String>) ->() {
    let mut line = String::from("");
    args.iter().for_each(|item| line += item);
    println!("{}", line);
}

fn builtin_cd(args: Vec<String>){
    let mut path = String::from("");
    args.iter().for_each(|item| path+= item);
    let path = Path::new(path.as_str());
    env::set_current_dir(path).expect("specified path doesn't exist");
}

fn builtin_pwd(args: Vec<String>){
    //Need to use a Result enum here to not exit
    if args.len() > 0{
        panic!("Too many arguments")
    };
    let cwd = env::current_dir().unwrap();
    println!("{:?}", cwd);
}
