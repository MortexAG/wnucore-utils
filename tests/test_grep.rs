use std::fs::{self, File};
use std::io::{Write, BufWriter};
use wnucore_utils::utils::{grep_run, GrepConfig};

#[test]
fn test_grep_basic_match() {
    // Ensure test directory
    let test_dir = "./tests/tmp";
    fs::create_dir_all(test_dir).unwrap();
    let file_path = format!("{}/grep_test.txt", test_dir);

    // Write known content to test file
    let file = File::create(&file_path).unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "Hello").unwrap();
    writeln!(writer, "World").unwrap();
    writeln!(writer, "Hello Rust").unwrap();

    // Prepare args to simulate: wgrep Hello tests/tmp/grep_test.txt
    let args = vec![
        "wgrep".to_string(),
        "Hello".to_string(),
        file_path.clone(),
    ];

    let config = GrepConfig::new(&args).expect("Failed to parse grep config");
    let result = grep_run(config);

    assert!(result.is_ok());
    // Optionally: capture and assert output if grep_run returns it
}
