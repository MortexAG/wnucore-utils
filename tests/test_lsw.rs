use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use wnucore_utils::utils::list_dir;

#[test]
fn test_lsw() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test directory and files
    let test_dir = "./tests/tmp_lsw";
    fs::create_dir_all(test_dir)?;

    let file1_path = format!("{}/file1", test_dir);
    let file2_path = format!("{}/file2", test_dir);

    File::create(&file1_path)?.write_all(b"Test file 1")?;
    File::create(&file2_path)?.write_all(b"Test file 2")?;

    // Run lsw function
    let entries = list_dir(test_dir)?;

    // Collect just the names
    let names: HashSet<_> = entries.into_iter().map(|(name, _)| name).collect();
    let expected: HashSet<_> = ["file1", "file2"].iter().map(|s| s.to_string()).collect();

    // Assertion
    assert_eq!(names, expected);

    // Optional: clean up after test
    fs::remove_file(file1_path)?;
    fs::remove_file(file2_path)?;
    fs::remove_dir(test_dir)?;

    Ok(())
}
