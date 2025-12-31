use std::collections::BTreeMap;

/// Core Strata value type.
/// This is the in-memory representation used by encoders/decoders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}
