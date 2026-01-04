#[cfg(test)]
mod tests {

    #[test]
    fn hash_is_deterministic() {
        use crate::hash::hash_value;
        use crate::value::Value;

        let value = Value::Int(42);

        let hash1 = hash_value(&value);
        let hash2 = hash_value(&value);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn hash_changes_on_value_change() {
        use crate::hash::hash_value;
        use crate::value::Value;

        let value1 = Value::Int(42);
        let value2 = Value::Int(43);

        let hash1 = hash_value(&value1);
        let hash2 = hash_value(&value2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn hash_respects_structure() {
        use crate::hash::hash_value;
        use crate::value::Value;

        let list_a = Value::List(vec![Value::Int(1), Value::Int(2)]);
        let list_b = Value::List(vec![Value::Int(2), Value::Int(1)]);

        assert_ne!(hash_value(&list_a), hash_value(&list_b));
    }
}
