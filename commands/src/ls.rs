use std::fs;
use colors::{Colors, to_color};
pub fn ls(args: Vec<String>) {
        let paths = match args.len() {
            1 => fs::read_dir(".").unwrap(),
            _ => fs::read_dir(&args[1]).unwrap()
        };
        for path in paths {
            let mut s = path.as_ref().unwrap().path().display().to_string();
            s.remove(0);
            s.remove(0);
            if path.as_ref().unwrap().path().is_dir() {
                s.push_str("/");
                println!("{}", to_color(&s, Colors::Blue));
            } else {
                println!("{}", to_color(&s, Colors::Green)); 
            }
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
