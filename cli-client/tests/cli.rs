use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn missing_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("guenther");
    cmd.assert().failure().stderr(predicate::str::contains(
        "unrecognized subcommand 'guenther'",
    ));

    Ok(())
}

#[test]
fn add_strike() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    file.write_str("{}")?;

    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("--db-path")
        .arg(file.path())
        .arg("strike")
        .arg("guenther");
    cmd.assert().success();

    Ok(())
}

#[test]
fn list_strikes() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    file.write_str("{\"guenther\": 1}")?;

    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("--db-path").arg(file.path()).arg("ls");
    cmd.assert().success().stdout("{\"guenther\": 1}\n");

    Ok(())
}
