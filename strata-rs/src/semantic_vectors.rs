use std::fs;
use std::path::Path;

use crate::parser::parse;
use crate::encode::encode_value;
use crate::hash::hash_value;

#[allow(dead_code)]
fn read_hex_file(path: &Path) -> Vec<u8> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read {}", path.display()));

    let text = text.trim();

    assert!(
        text.len() % 2 == 0,
        "hex file has odd length: {}",
        path.display()
    );

    let mut out = Vec::with_capacity(text.len() / 2);
    for i in (0..text.len()).step_by(2) {
        let byte = u8::from_str_radix(&text[i..i + 2], 16)
            .unwrap_or_else(|_| panic!("invalid hex in {}", path.display()));
        out.push(byte);
    }

    out
}

#[allow(dead_code)]
pub fn run_vector(name: &str) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("vectors")
        .join(name);

    let st_path = base.with_extension("st");
    let scb_path = base.with_extension("scb.hex");
    let hash_path = base.with_extension("hash.hex");

    let st = fs::read_to_string(&st_path)
        .unwrap_or_else(|_| panic!("failed to read {}", st_path.display()));

    let value = parse(&st)
        .unwrap_or_else(|| panic!("parse failed for {}", st_path.display()));

    let encoded = encode_value(&value);

    let hash = hash_value(&value);

    let exp_scb = read_hex_file(&scb_path);
    let exp_hash = read_hex_file(&hash_path);

    assert_eq!(
        encoded, exp_scb,
        "SCB mismatch for vector {}",
        name
    );

    assert_eq!(
        hash.to_vec(), exp_hash,
        "hash mismatch for vector {}",
        name
    );
}

#[cfg(test)]
mod tests {
    use crate::semantic_vectors::run_vector;

    #[test]
    fn vector_v1_01_basic() {
        run_vector("v1/01-basic");
    }

    #[test]
    fn vector_v1_02_map_order() {
        run_vector("v1/02-map-order");
    }

    #[test]
    fn vector_v1_03_bigint_bytes() {
        run_vector("v1/03-bigint-bytes");
    }

    // other tests - to move later
    use crate::parser::{Parser};
    use crate::value::Value;

    #[test]
    fn semantic_vector_config() {
        let input = r#"
            config {
                enabled: true
                retries: 3
                name: "strata"
                empty: null
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut config = BTreeMap::new();
        config.insert("enabled".into(), Value::Bool(true));
        config.insert("retries".into(), Value::Int(3));
        config.insert("name".into(), Value::String("strata".into()));
        config.insert("empty".into(), Value::Null);

        let mut root = BTreeMap::new();
        root.insert("config".into(), Value::Map(config));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }

    #[test]
    fn semantic_vector_key_ordering() {
        let input = r#"
            data {
                z: 1
                a: 2
                m: 3
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut data = BTreeMap::new();
        data.insert("a".into(), Value::Int(2));
        data.insert("m".into(), Value::Int(3));
        data.insert("z".into(), Value::Int(1));

        let mut root = BTreeMap::new();
        root.insert("data".into(), Value::Map(data));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }

    #[test]
    fn semantic_vector_profile() {
        let input = r#"
            profile {
                id: 9007199254740993
                avatar_hash: 0x9f86d081884c7d659a2feaa0c55ad015
                tags: ["logistics", "state", "integrity"]
            }
        "#;

        let mut p = Parser::new(input);

        use std::collections::BTreeMap;

        let mut profile = BTreeMap::new();
        profile.insert("id".into(), Value::Int(9007199254740993));
        profile.insert(
            "avatar_hash".into(),
            Value::Bytes(vec![
                0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65,
                0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15,
            ]),
        );
        profile.insert(
            "tags".into(),
            Value::List(vec![
                Value::String("logistics".into()),
                Value::String("state".into()),
                Value::String("integrity".into()),
            ]),
        );

        let mut root = BTreeMap::new();
        root.insert("profile".into(), Value::Map(profile));

        assert_eq!(p.parse_value(), Some(Value::Map(root)));
    }
}
