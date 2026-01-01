use crate::value::Value;
use crate::encode::encode_value;

const STRATA_MAGIC: &[u8; 7] = b"STRATA1";
const STRATA_VERSION: u8 = 0x01;

pub fn encode_framed(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(STRATA_MAGIC);
    out.push(STRATA_VERSION);
    out.extend_from_slice(&encode_value(value));

    out
}
