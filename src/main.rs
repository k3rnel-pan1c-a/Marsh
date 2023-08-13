use std::io;
use std::io::Write;

const PROMPT_CHAR: &str = "->";

struct Command {
    keyword: String,
    args: Vec<String>,
}

fn main() {
    //since shells are REPL, then we'll have an infinite loop
    loop {
        print_prompt_char();
        let cmd = parse_cmd(read_cmd());
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
