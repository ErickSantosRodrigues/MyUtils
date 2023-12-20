use std::fs;

pub fn mkdir(args: Vec<String>) -> Result<(), &'static str> {
    if args.len() != 2 {
        return Err("mkdir: missing operand")
    }
    
    fs::create_dir(args[1]).map_err(|_| "mkdir: failed to create directory")
}
