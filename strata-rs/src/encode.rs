pub fn encode_uleb128(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

pub fn encode_sleb128(mut value: i64, out: &mut Vec<u8>) {
    loop {
        let byte = (value & 0x7F) as u8;
        let sign_bit = byte & 0x40;
        value >>= 7;

        let done = (value == 0 && sign_bit == 0)
            || (value == -1 && sign_bit != 0);

        out.push(if done { byte } else { byte | 0x80 });

        if done {
            break;
        }
    }
}

use crate::value::Value;

pub fn encode_value(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    encode_into(value, &mut out);
    out
}

fn encode_into(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => {
            out.push(0x00);
        }

        Value::Bool(false) => {
            out.push(0x01);
        }

        Value::Bool(true) => {
            out.push(0x02);
        }

        Value::Int(number) => {
            out.push(0x10);
            encode_sleb128(*number, out);
        }

        Value::String(string) => {
            out.push(0x20);
            let bytes = string.as_bytes();
            encode_uleb128(bytes.len() as u64, out);
            out.extend_from_slice(bytes);
        }

        Value::Bytes(bytes) => {
            out.push(0x21);
            encode_uleb128(bytes.len() as u64, out);
            out.extend_from_slice(bytes);
        }

        Value::List(items) => {
            out.push(0x30);
            encode_uleb128(items.len() as u64, out);
            for item in items {
                encode_into(item, out);
            }
        }

        Value::Map(map) => {
            out.push(0x40);
            encode_uleb128(map.len() as u64, out);

            // BTreeMap guarantees canonical order
            for (key, value) in map {
                // key encoded exactly like a String
                out.push(0x20);
                let key_bytes = key.as_bytes();
                encode_uleb128(key_bytes.len() as u64, out);
                out.extend_from_slice(key_bytes);

                encode_into(value, out);
            }
        }
    }
}
