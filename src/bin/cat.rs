use std::error::Error;
use std::env;
use wnucore_utils::utils::{read_file, is_help_flag};
use colored::Colorize;
fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    //  -h/--help 
    if args.len() < 2 || is_help_flag(&args[1]) {
        println!(
            "{}",
        "
cat - display contents of a file

Usage:
    cat <file_path>

Description:
    Reads and prints the content of the specified file.

Example:
    cat notes.txt
"
        .bold()); 
    std::process::exit(0);
    }
    let contents = read_file(&args[1])?;
    println!("{:#}", contents);
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_cat() -> Result<(), Box<dyn Error>> {
        // Ensure the test directory exists
        let dir_path = "./tests";
        fs::create_dir_all(dir_path)?;

        // Define the test file path and content
        let file_path = format!("{}/cat.txt", dir_path);
        let expected_content = "Cat\nTest\nOutput";

        // Create and write to the test file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Cat")?;
        writeln!(file, "Test")?;
        write!(file, "Output")?; // no newline here to match expected content

        // Read the file using your function
        let actual_content = read_file(&file_path)?;

        // Compare the result
        assert_eq!(expected_content, actual_content);

        Ok(())
    }
}
