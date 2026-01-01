use crate::value::Value;
use crate::encode::encode_value;

/// Hash a Strata Value using canonical encoding
/// Returns raw 32-byte BLAKE3 hash.
pub fn hash_value(value: &Value) -> [u8; 32] {
    let bytes = encode_value(value);
    blake3::hash(&bytes).into()
}
