use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn channel_status() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("balanzierer-cli")?;
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("hello, foobar"));
    Ok(())
}

