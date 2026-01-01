use std::path::PathBuf;
use assert_cmd::Command;
use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

fn strata() -> Command {
    cargo_bin_cmd!("strata")
}

fn temp_file(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("strata_test_{}", name));
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn cli_compile_smoke() {
        let input = temp_file("input.st");
        let output = temp_file("output.scb");

        fs::write(
            &input,
            r#"
            a { x: 1 }
        "#,
        )
        .unwrap();

        strata()
            .args(["compile", input.to_str().unwrap(), output.to_str().unwrap()])
            .assert()
            .success();

        assert!(output.exists());

        let bytes = fs::read(&output).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn cli_hash_st() {
        let input = temp_file("hash.st");

        fs::write(&input, "1").unwrap();

        strata()
            .args(["hash", input.to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::is_match(r"^[0-9a-f]{64}\n?$").unwrap());
    }

    #[test]
    fn cli_hash_scb() {
        let input = temp_file("hash.scb");

        // arbitrary canonical bytes
        fs::write(&input, vec![0x10, 0x01]).unwrap();

        strata()
            .args(["hash", input.to_str().unwrap()])
            .assert()
            .success();
    }
}
