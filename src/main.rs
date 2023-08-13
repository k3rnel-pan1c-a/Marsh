use std::io;
use std::io::Write;

fn main() {
    //since shells are REPL, then we'll have an infinite loop
    loop{
        let prompt_chat: &str = "->";
        print!("{}", prompt_chat);
        io::stdout().flush().unwrap();
        let mut cmd: String = String::new();
        io::stdin().read_line(&mut cmd).expect("Failed to read the entered command");
        let cmd_args: Vec<&str> = cmd.split(" ").collect();
        println!("{:?}", cmd_args)

    }

}
