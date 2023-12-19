use std::env;
use commands::myls;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments");
    } else {
        //remove the first argument 
        let args = &args[1..];
        match args[0].as_str() {
            "myls" => myls::myls(args.to_vec()),
            _ => println!("Unknown command"),
        }
    }
    
}
