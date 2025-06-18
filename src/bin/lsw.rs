use std::error::Error;
use std::env;
use colored::Colorize;
use wnucore_utils::utils::{list_dir, FileType, is_help_flag};

fn main() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect();
     // check args length
    if args.len() < 2 || is_help_flag(&args[1]) {
        println!(
            "{}",
        "
lsw - list files and directories

Usage:
    lsw <directory_path>

Description:
    Lists all entries in the given directory.
    Directories are shown in blue.
    Executable files are shown in green.
    Regular files are shown in white.

Example:
    lsw ./some_folder
"
        .bold());
    std::process::exit(0);
}
    let entries = list_dir(&args[1])?;

    if entries.is_empty() {
        println!("No entries found or not a directory.");
    } else {
        for (name, file_type) in entries {
            match file_type {
                FileType::Directory => println!("{}", name.blue().bold()),
                FileType::Executable => println!("{}", name.green().bold()),
                FileType::File => println!("{}", name.white().bold()),            
            }
        }
    }
    Ok(())
}

// a test
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_lsw() -> Result<(), Box<dyn Error>> {
        let path = "./tests/my-test/lsw"; // this directory should contain known test files
        let entries = list_dir(path)?;

        // Extract just the names for comparison
        let names: HashSet<_> = entries.into_iter().map(|(name, _)| name).collect();

        let expected: HashSet<_> = ["file1", "file2"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(names, expected);
        Ok(())
    }
}