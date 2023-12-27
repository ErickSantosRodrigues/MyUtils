use std::fs;
pub fn rm (args: Vec<String>) {
    if args.len() < 2 {
        println!("rm: missing operand");
    }
    else {
        // check for flags
        if args[1].starts_with("-") {
            if args[1] == "-r" {
                for i in 2..args.len() {
                    fs::remove_dir_all(&args[i]).expect("rm: cannot remove directory");
                }
            }
            else if args[1] == "-f" {
                for i in 2..args.len() {
                    fs::remove_file(&args[i]).expect("rm: cannot remove file");
                }
            }
            else {
                println!("rm: invalid option");
            }
        }
        else {
            for i in 1..args.len() {
                fs::remove_file(&args[i]).expect("rm: cannot remove file");
            }
        }
    }
}
