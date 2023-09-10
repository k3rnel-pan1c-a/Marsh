use marsh::{Builtin, Cmd};
use std::str::FromStr;

pub fn parse_cmd(mut cmd: Vec<String>) -> Cmd {
    let mut cmd: Cmd = Cmd {
        keyword: cmd.remove(0),
        args: cmd,
        builtin: None,
    };
    match Builtin::from_str(&*cmd.keyword.trim()) {
        Ok(Builtin::Echo) => cmd.builtin = Some(Builtin::Echo),
        Ok(Builtin::Cd) => cmd.builtin = Some(Builtin::Cd),
        Ok(Builtin::Pwd) => cmd.builtin = Some(Builtin::Pwd),
        Err(_) => cmd.builtin = None,
    }
    cmd
}
