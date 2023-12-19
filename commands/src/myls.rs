// create my own ls command
// usage: myls [path]
// default path is current directory

//use std::env;
use std::fs;

pub fn myls(args: Vec<String>) {
        let paths = match args.len() {
            1 => fs::read_dir(".").unwrap(),
            _ => fs::read_dir(&args[1]).unwrap()
        };
        for path in paths {
            println!("{}", path.unwrap().path().display());
        }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_myls() {
        let args = vec![String::from("myls"), String::from("src")];
        myls(args);
    }
}
