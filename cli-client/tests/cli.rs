use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;
use std::{env, fs, process::Command};

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
fn it_should_add_strike() -> Result<(), Box<dyn std::error::Error>> {
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
fn it_should_list_strikes_in_descending_order() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    file.write_str("{\"guenther\": 1, \"heinz\": 2}")?;

    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("--db-path").arg(file.path()).arg("ls");

    let expected_output = "Tarnished  | Strikes    |\n\
                           heinz      | 2          |\n\
                           guenther   | 1          |\n";

    cmd.assert().success().stdout(expected_output);

    Ok(())
}

#[test]
fn it_should_use_default_directory_if_no_configuration_directory_exists(
) -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("HOME", "./tests/fixtures");

    let mut cmd = Command::cargo_bin("strikes")?;
    cmd.arg("strike").arg("guenther");
    cmd.assert().success();

    let _ = fs::remove_dir_all("./tests/fixtures/.strikes")?;

    Ok(())
}
