use crate::helpers::command;
use predicates::prelude::*;

#[test]
fn invalid_levels() {
    for invalid_level in &[
        "not-a-real-level",
        "'-1-", // Only positive values are allowed
        "256",  // Out of bounds for u8
    ] {
        let mut cmd = command();

        cmd.arg("-l").arg(invalid_level);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Invalid level"));
    }
}

#[test]
fn valid_levels() {
    for valid_level in &[
        "info", "10", "InFo", "INFO", "warn", "debug", "trace", "fatal", "error",
    ] {
        let mut cmd = command();

        cmd.arg("-l").arg(valid_level);
        cmd.assert().success();
    }
}
