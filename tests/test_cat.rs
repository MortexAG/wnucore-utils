use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

#[test]
fn test_cat_output() {
    // Set up
    let dir_path = "./tests/tmp";
    fs::create_dir_all(dir_path).unwrap();
    let file_path = format!("{}/cat.txt", dir_path);
    let expected_content = "Cat\nTest\nOutput";

    // Write the file
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Cat").unwrap();
    writeln!(file, "Test").unwrap();
    write!(file, "Output").unwrap(); // no newline at end

    // Run the binary and capture the output
    let output = Command::new("target/debug/cat")
        .arg(&file_path)
        .output()
        .expect("Failed to run cat binary");

    assert!(output.status.success());
    let actual_content = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Check output
    assert_eq!(expected_content, actual_content);
}
