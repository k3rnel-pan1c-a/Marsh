use std::env;
use std::path::Path;
use crate::ROOT;

fn builtin_echo(args: Vec<String>) -> () {
    let mut line = String::from("");
    args.iter().for_each(|item| line += item);
    println!("{}", line);
}

fn builtin_cd(args: Vec<String>) {
    //add the .. thingy
    if args.len() == 0 {
        let root = Path::new(ROOT);
        env::set_current_dir(&root).expect("root '/Users/anasbadr' doesn't exist");
    }
    else if args[0] == "..".to_string(){
        todo!()
    }
    else {
        let mut path = String::from("");
        args.iter().for_each(|item| path += item);
        let path = Path::new(path.as_str());
        env::set_current_dir(&path).expect("specified path doesn't exist");
    }

}

fn builtin_pwd(args: Vec<String>) {
    //Need to use a Result enum here to not exit
    if args.len() > 0 {
        panic!("Too many arguments")
    };
    let cwd = env::current_dir().unwrap();
    println!("{:?}", cwd);
}