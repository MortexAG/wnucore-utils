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
    let (path, recursieve) = if args[1] == "-r" || args[1] == "--recursive"{
        if args.len() < 3 {
            eprintln!("Please provide a path to delete with -r flag");
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
            confirm_and_delete(path, true)?;
        } else {
            confirm_and_delete(path, false)?;
        }
    }else {
        confirm_and_delete(path, false)?;
    }
    Ok(())

}
fn confirm_and_delete(path: &str, recursive: bool) -> Result<(), Box<dyn Error>> {
    let path_obj = Path::new(path);
    let mut input = String::new();
    let action = if recursive { "force delete" } else { "delete" };

    if path_obj.is_dir() {
        println!(
            "Sure you want to {} {} and all of its contents? (y/n)",
            action,
            format!("{}", path.red().bold())
        );
    } else {
        println!(
            "Sure you want to {} {}? (y/n)",
            action,
            format!("{}", path.red().bold())
        );
    }

    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        if path_obj.is_dir() {
            if recursive {
                fs::remove_dir_all(path_obj)?;
            } else {
                fs::remove_dir(path_obj).map_err(|err| {
                    format!("Couldn't delete the directory. Perhaps it's not empty?{}",err)})?;
                }
        } else {
            fs::remove_file(path_obj)?;
        }
        println!("Deleted {}", path.green().bold());
    } else {
        println!("Cancelled.");
    }

    Ok(())
}