use crate::parser::parse_cmd;
use std::process::Child;
use marsh::{Cmd, pipe};

pub fn tokenize_cmd(cmd: String) -> (Vec<String>, Option<Child>) {
    if cmd.contains("|") {
        let cmds = cmd
            .split("|")
            .map(|slice| slice.to_string())
            .collect::<Vec<String>>();
        let mut cmds: Vec<(Vec<String>, Option<Child>)> =
            cmds.into_iter().map(|cmd| tokenize_cmd(cmd)).collect();
        let mut cmds_strings: Vec<Vec<String>> = cmds.into_iter().map(|cmd| cmd.0).collect();
        let parsed_cmds: Vec<Cmd> = cmds_strings
            .clone()
            .into_iter()
            .map(|cmd| parse_cmd(cmd))
            .collect();
        let last_cmd = cmds_strings.pop().unwrap();
        (last_cmd, pipe(parsed_cmds))
    } else {
        let cmd_args: Vec<String> = cmd
            .split_whitespace()
            .map(|item| item.to_string())
            .collect();

        (cmd_args, None)
    }
}
