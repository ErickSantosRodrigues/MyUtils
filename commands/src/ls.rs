// create my own ls command
// usage: myls [path]
// default path is current directory

//use std::env;
use std::fs;

pub fn ls(args: Vec<String>) {
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
    fn test_ls() {
        let args = vec![String::from("ls"), String::from("src")];
        ls(args);
    }
}
