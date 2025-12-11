use assert_cmd::cargo::*;
use predicates::prelude::*;

#[test]
fn properties_file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("alignment-colorizer");

    cmd.arg("rates").arg("test/file/doesnt/exist").arg("test/file/doesnt/exist2");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
