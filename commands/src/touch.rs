use std::fs::OpenOptions;

pub fn touch(args: Vec<String>) -> Result<(), &'static str> {
    if args.len() < 2 {
        return Err("touch: missing operand");
    }

    for arg in args.iter().skip(1) {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(arg)
            .expect("touch: failed to create file");
    }

    Ok(())
}
