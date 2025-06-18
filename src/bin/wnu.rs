use colored::Colorize;

fn main() {
    println!("{}", "wnu - WNU Core Utilities".bold());

    println!("\n{}:", "Available Commands".underline().bold());
    println!("  {}      - Print file contents", "cat".cyan());
    println!("  {}    - Clear the screen", "clear".cyan());
    println!("  {}       - Show first N lines of a file", "head".cyan());
    println!("  {}        - List files/directories", "lsw".cyan());
    println!("  {}         - Search for text in a file", "grep".cyan());
    println!("  {}           - Delete file/directory", "rm".cyan());
    println!("  {}      - Show last N lines of a file", "tail".cyan());
    println!("  {}     - Create new file", "touch".cyan());

    println!(
        "\n{}: Run with '--help' or '-h' for usage per command.",
        "Tip".italic()
    );
}
