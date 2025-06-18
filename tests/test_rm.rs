// tests/rm_test.rs
use std::fs::{ File};
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

#[test]
fn test_rm_deletes_file() {
    let temp = tempdir().unwrap();
    let file_path = temp.path().join("test.txt");

    File::create(&file_path).unwrap();
    assert!(file_path.exists());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("rm"))
        .arg(file_path.to_str().unwrap())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to run rm");

    {
        let stdin = cmd.stdin.as_mut().unwrap();
        stdin.write_all(b"y\n").unwrap();
    }

    let status = cmd.wait().unwrap();
    assert!(status.success());
    assert!(!file_path.exists());
}
