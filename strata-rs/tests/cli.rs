use assert_cmd::Command;
use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::path::PathBuf;

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
    fn cli_decode_smoke() {
        let input = temp_file("decode.scb");

        // Encode: Int(1) -> 0x10 0x01
        fs::write(&input, vec![0x10, 0x01]).unwrap();

        let output = strata()
            .args(["decode", input.to_str().unwrap()])
            .output()
            .unwrap();

        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Int"));
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

    // more tests
    #[test]
    fn cli_decode_map() {
        let scb = temp_file("decode_map.scb");

        // { 'a': 1 } -> 0x40 0x01 0x20 0x01 'a' 0x10 0x01
        let bytes = vec![
            0x40, 0x01, // map with 1 entry
            0x20, 0x01, b'a', // key: 'a'
            0x10, 0x01, // value: 1
        ];

        fs::write(&scb, bytes).unwrap();

        let output = strata()
            .args(["decode", scb.to_str().unwrap()])
            .output()
            .unwrap();

        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Map"));
        assert!(stdout.contains("a"));
    }

    #[test]
    fn cli_decode_invalid() {
        let scb = temp_file("decode_invalid.scb");

        // invalid tag 0xFF
        let bytes = vec![0xFF];

        fs::write(&scb, bytes).unwrap();

        let output = strata()
            .args(["decode", scb.to_str().unwrap()])
            .output()
            .unwrap();

        assert!(!output.status.success());
    }
}
