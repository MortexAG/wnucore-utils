use std::path::Path;
use std::{env, io};
use std::error::Error;
use std::fs;
use colored::Colorize;
fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [-r|--recursive] <path>", args[0]);
        return Ok(());
    }
    let (path, recursieve) = if args[1] == "r" || args[1] == "--recursive"{
        if args.len() < 3 {
            eprintln!("Please provide a path to delete with -r flag")
            return Ok(());
        }
        (&args[2], true)
    }else {
        (&args[1], false)
    };

    let path_obj = Path::new(path);

    if !path_obj.exists() {
        eprintln!("Path '{}' does not exist", path.red());
        return Ok(());
    }
    if path_obj.is_dir() {
        if recursieve {
            let hi = confirm_and_delete(path, true);
            Ok(())
        }
        Ok(())
    }

}

fn confirm_and_delete(path: &str, recursive: bool) -> Result<(), Box<dyn Error>>{
    //continue later
    Ok(())
}