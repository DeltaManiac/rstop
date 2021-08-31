use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rstop")?;
    cmd.arg("info").arg("");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file at"));
    Ok(())
}

#[test]
fn file_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rstop")?;
    cmd.arg("info").arg("tests/fixtures/a.json");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Information"))
        .stdout(predicate::str::contains("Name : Sample Postman Collection"));
    Ok(())
}


#[test]
fn invalid_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rstop")?;
    cmd.arg("info").arg("tests/fixtures/b.json");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("error: while parsing a block mapping"));
    Ok(())
}
