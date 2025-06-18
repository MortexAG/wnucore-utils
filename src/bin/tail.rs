use std::{env, fs::File, io::{ BufRead, BufReader}, process};
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return;
    }

    let path = &args[1];
    let lines_to_show = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(10)
    } else {
        10
    };

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
            let start = lines.len().saturating_sub(lines_to_show);
            for line in &lines[start..] {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error reading file".red(), e);
            process::exit(1);
        }
    }
}

fn print_help() {
    println!(
        "{}",
        "
tail - show last lines of a file

Usage:
    tail <file_path> [line_count]

Description:
    Shows the last N lines of the given file.
    Defaults to 10 lines.

Example:
    tail file.txt 15
"
        .bold()
    );
}
