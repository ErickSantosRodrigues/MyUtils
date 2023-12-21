use std::env;
use commands::{*};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments");
    } else {
        //remove the first argument 
        let args = &args[1..];
        match args[0].as_str() {
            "ls" => ls::ls(args.to_vec()),
            "mkdir" => mkdir::mkdir(args.to_vec()).unwrap(),

            _ => println!("Unknown command"),
        }
    }
    
}
