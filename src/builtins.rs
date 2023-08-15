use crate::HOME;
use std::env;
use std::path::Path;

pub fn builtin_echo(args: Vec<String>) -> () {
    //add the option to echo env_var
    if args.len() == 1{
        if let Some(env_var) = args.get(0).unwrap().as_str().strip_prefix("$"){
            match env::var(env_var.trim()) {
                Ok(value) => println!("Value of env_var: {}", value),
                Err(_) => println!("env_var is not set"),
            }
        }
    }
    let mut line = String::from("");
    args.iter().for_each(|item| line += item);
    println!("{}", line);
}

pub fn builtin_cd(args: Vec<String>) {
    //add the .. thingy
    if args.len() == 0 {
        let root = Path::new(HOME);
        env::set_current_dir(&root).expect("root '/Users/anasbadr' doesn't exist");
    } else if args[0] == "..".to_string() {
        todo!()
    } else {
        let mut path = String::from("");
        args.iter().for_each(|item| path += item);
        let path = Path::new(path.as_str());
        env::set_current_dir(&path).expect("specified path doesn't exist");
    }
}

pub fn builtin_pwd(args: Vec<String>) {
    //Need to use a Result enum here to not exit
    if args.len() > 0 {
        panic!("Too many arguments")
    };
    let cwd = env::current_dir().unwrap();
    println!("{:?}", cwd);
}
