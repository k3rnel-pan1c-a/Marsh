mod builtins;
mod parser;
mod tokenizer;

use parser::*;
use std::{env, path::Path};
use tokenizer::*;
use marsh::{print_prompt_char, process_cmd, read_cmd};

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


