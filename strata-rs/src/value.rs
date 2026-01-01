use std::collections::BTreeMap;

/// Core Strata value type.
/// This is the in-memory representation used by encoders/decoders.
/// Integer semantics (Northstar v1):
/// - All integers are signed 64-bit (i64)
/// - No floats, no implicit coercions
/// - Encoded using canonical SLEB128
/// - Cross-language implementations MUST map to i64 exactly
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
