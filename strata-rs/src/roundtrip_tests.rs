#[cfg(test)]
mod tests {
    use crate::decode::decode;
    use crate::encode::encode_value;
    use crate::parser::parse;

    #[test]
    fn roundtrip_structural_simple() {
        let input = r#"
            user {
                id: 42
                active: true
                name: "Gencho"
            }
        "#;

        let v1 = parse(input).expect("Parsing failed");
        let bytes = encode_value(&v1);
        let v2 = decode(&bytes).expect("Decoding failed");

        assert_eq!(v1, v2);
    }

    #[test]
    fn roundtrip_allows_reordering() {
        let input = r#"
            data {
                z: 1
                a: 2
                m: 3
            }
        "#;

        let v1 = parse(input).expect("Parsing failed");
        let bytes = encode_value(&v1);
        let v2 = decode(&bytes).expect("Decoding failed");

        assert_eq!(v1, v2);
    }

    #[test]
    fn roundtrip_nested() {
        let input = r#"
            profile {
                id: 9007199254740993
                tags: ["logistics", "state", "integrity"]
                avatar: 0xdeadbeef
            }
        "#;

        let v1 = parse(input).expect("Parsing failed");
        let bytes = encode_value(&v1);
        let v2 = decode(&bytes).expect("Decoding failed");

        assert_eq!(v1, v2);
    }
}
