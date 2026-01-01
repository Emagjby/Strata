#[cfg(test)]
mod tests {

    #[test]
    fn hash_is_deterministic() {
        use crate::value::Value;
        use crate::hash::hash_value;

        let v = Value::Int(42);

        let h1 = hash_value(&v);
        let h2 = hash_value(&v);

        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_changes_on_value_change() {
        use crate::value::Value;
        use crate::hash::hash_value;

        let v1 = Value::Int(42);
        let v2 = Value::Int(43);

        let h1 = hash_value(&v1);
        let h2 = hash_value(&v2);

        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_respects_structure() {
        use crate::value::Value;
        use crate::hash::hash_value;

        let a = Value::List(vec![Value::Int(1), Value::Int(2)]);
        let b = Value::List(vec![Value::Int(2), Value::Int(1)]);

        assert_ne!(hash_value(&a), hash_value(&b));
    }
}
