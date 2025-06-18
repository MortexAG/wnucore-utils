use std::env;
use std::process;
use wnucore_utils::utils::{grep_run, GrepConfig, is_help_flag};
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if no args or help flag is present
    if args.len() < 3 || args.iter().any(|arg| is_help_flag(arg)) {
        println!(
            "{}",
            "
grep - search for patterns in files

Usage:
    grep <pattern> <file_path>

Description:
    Searches for lines in the specified file that contain the given pattern.

Examples:
    grep \"hello\" ./file.txt
    grep main src/main.rs
"
            .bold()
        );
        process::exit(0);
    }

    let config = GrepConfig::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep_run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use wnucore_utils::utils::{grep_run, GrepConfig};

    #[test]
    fn test_grep_basic_match() {
        // Create temp file with known content
        let path = "./tests/my-test/grep_test.txt";
        let mut file = File::create(path).expect("Failed to create test file");
        writeln!(file, "Hello\nWorld\nHello Rust").expect("Failed to write to test file");

        // Simulate args
        let args = vec![
            "wgrep".to_string(),
            "Hello".to_string(),
            path.to_string(),
        ];

        let config = GrepConfig::new(&args).unwrap();
        let result = grep_run(config);

        assert!(result.is_ok());
    }
}
