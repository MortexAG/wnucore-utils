use std::{env, error::Error, fs::OpenOptions};
use wnucore_utils::utils::is_help_flag;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || is_help_flag(&args[1]) {
        println!(
            "
touch - create an empty file (like Unix 'touch')

Usage:
    touch <file_path>

Description:
    Creates the file if it doesn't exist.
    If it does, its modification time is updated (not yet implemented).

Example:
    touch myfile.txt
    touch myfile.txt myfile2.txt myfile3.bat
    touch /path/myfile1 /path/myfile2.txt
"
        );
        std::process::exit(0);
    }
    for file in &args[1..] { // the first argument and whatever follows will be files to create
        OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file)?;
    }
    Ok(())
}
