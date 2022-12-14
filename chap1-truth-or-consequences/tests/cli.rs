// tests

use assert_cmd::Command;

// checks if main program prints correct output
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("chap1-truth-or-consequences").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

// tests true program
#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

// tests false program
#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}