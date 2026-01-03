use crate::encode::encode;
use crate::value::Value;

/// Hash a Strata Value using canonical encoding
/// Returns raw 32-byte BLAKE3 hash.
pub fn hash_value(value: &Value) -> [u8; 32] {
    let bytes = encode(value).unwrap();
    blake3::hash(&bytes).into()
}
