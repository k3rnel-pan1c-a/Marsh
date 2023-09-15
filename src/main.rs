mod builtins;
mod parser;
mod tokenizer;

use parser::*;
use std::{env, path::Path};
use std::env::VarError;
use tokenizer::*;
use marsh::{print_prompt_char, process_cmd, read_cmd};

fn main() {
    let home_dir = env::var("HOME");

    match home_dir{
        Ok(dir) => {
            let root = Path::new(&dir);
            env::set_current_dir(&root).expect("$HOME environment variable doesn't exist...");
        }
        Err(_) => {}
    }
    //read the environment variables
    //need to add a fn to read a dotfile
    //cd to the root directory on start

    //since shells are REPL, then we'll have an infinite loop
    loop {
        print_prompt_char();
        let cmd = tokenize_cmd(read_cmd());
        process_cmd(parse_cmd(cmd.0), cmd.1);
    }
}


